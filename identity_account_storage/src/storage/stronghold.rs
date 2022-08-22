// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeSet;

use async_trait::async_trait;
use crypto::ciphers::aes_gcm::Aes256Gcm;
use crypto::ciphers::traits::Aead;
use futures::executor;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_core::crypto::X25519;
use identity_did::did::CoreDID;
use identity_iota_core::did::IotaDID;
use identity_iota_core::tangle::NetworkName;
use iota_stronghold::procedures;
use iota_stronghold::procedures::ProcedureError;
use iota_stronghold::procedures::Sha2Hash;
use iota_stronghold::sync::MergePolicy;
use iota_stronghold::sync::SyncClientsConfig;
use iota_stronghold::Client;
use iota_stronghold::ClientVault;
use iota_stronghold::Location;
use iota_stronghold::Store;
use rand::distributions::DistString;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;
use zeroize::Zeroize;

use crate::error::Error;
use crate::error::Result;
use crate::storage::Storage;
use crate::stronghold::ClientOperation;
use crate::stronghold::ClientPath;
use crate::stronghold::StoreOperation;
use crate::stronghold::Stronghold;
use crate::stronghold::StrongholdError;
use crate::stronghold::VaultOperation;
use crate::types::AgreementInfo;
use crate::types::CekAlgorithm;
use crate::types::DIDType;
use crate::types::EncryptedData;
use crate::types::EncryptionAlgorithm;
use crate::types::KeyLocation;
use crate::types::Signature;

// The name of the stronghold client used for indexing, which is global for a storage instance.
static INDEX_CLIENT_PATH: &str = "$index";
// The key in the index store that contains the serialized index.
// This happens to be the same as the client path, but for explicitness we define them separately.
static INDEX_STORE_KEY: &str = INDEX_CLIENT_PATH;
static BLOB_STORE_KEY: &str = "$blob";
// The static identifier for vaults inside clients.
static VAULT_PATH: &[u8; 6] = b"$vault";

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl Storage for Stronghold {
  async fn did_create(
    &self,
    did_type: DIDType,
    network: NetworkName,
    fragment: &str,
    private_key: Option<PrivateKey>,
  ) -> Result<(CoreDID, KeyLocation)> {
    // =============================
    // KEY GENERATION/INSERTION
    // =============================

    let tmp_client: Client = Client::default();
    let tmp_location: KeyLocation = random_location(KeyType::Ed25519);

    match private_key {
      Some(private_key) => {
        insert_private_key(&tmp_client, private_key, &tmp_location)?;
      }
      None => {
        generate_private_key(&tmp_client, &tmp_location)?;
      }
    }

    let public_key: PublicKey = retrieve_public_key(&tmp_client, &tmp_location)?;

    let did: CoreDID = {
      match did_type {
        DIDType::IotaDID => IotaDID::new_with_network(public_key.as_ref(), network)
          .map_err(|err| crate::Error::DIDCreationError(err.to_string()))?
          .into(),
      }
    };

    // =============================
    // ADD DID TO INDEX
    // =============================

    let index_lock: RwLockWriteGuard<'_, _> = self.index_lock.write().await;

    let index_client_path: ClientPath = ClientPath::from(INDEX_CLIENT_PATH);
    let index_client: Client = self.client(&index_client_path)?;
    let index_store: Store = index_client.store();

    let mut index: BTreeSet<CoreDID> = get_index(&index_store)?;

    if index.contains(&did) {
      return Err(crate::Error::IdentityAlreadyExists);
    } else {
      index.insert(did.clone());
    }

    set_index(&index_store, index)?;

    self
      .stronghold
      .write_client(index_client_path.as_ref())
      .map_err(|err| StrongholdError::Client(ClientOperation::Persist, index_client_path, err))?;

    // Explicitly release the lock early.
    std::mem::drop(index_lock);

    // =============================
    // CLIENT SYNC & KEY MOVE
    // =============================
    let location: KeyLocation = KeyLocation::new(KeyType::Ed25519, fragment.to_owned(), public_key.as_ref());

    self.mutate_client(&did, |client| {
      // Sync the vault identified by VAULT_PATH from the tmp client to the client identified by the DID.
      let mut sync_config: SyncClientsConfig = SyncClientsConfig::new(MergePolicy::Replace);
      sync_config.sync_selected_vaults(vec![VAULT_PATH]);

      client
        .sync_with(&tmp_client, sync_config)
        .map_err(|err| StrongholdError::Client(ClientOperation::Sync, ClientPath::from(&did), err))?;
      std::mem::drop(tmp_client);

      // Within client, move the key from the tmp location to the expected location.
      move_key(&client, &tmp_location, &location)?;

      Ok(())
    })?;

    Ok((did, location))
  }

  async fn did_purge(&self, did: &CoreDID) -> Result<bool> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let index_client_path: ClientPath = ClientPath::from(INDEX_CLIENT_PATH);
    let index_client: Client = self.client(&index_client_path)?;
    let index_store: Store = index_client.store();

    let mut index: BTreeSet<CoreDID> = get_index(&index_store)?;

    // Remove index entry if present.
    if !index.remove(did) {
      return Ok(false);
    }

    set_index(&index_store, index)?;

    self
      .stronghold
      .write_client(index_client_path.as_ref())
      .map_err(|err| StrongholdError::Client(ClientOperation::Persist, index_client_path, err))?;

    // Explicitly release the lock early.
    std::mem::drop(index_lock);

    // Delete the client from the snapshot, which removes the store and the vaults (= all keys).
    let client_path: ClientPath = ClientPath::from(did);
    let client: Client = self.client(&client_path)?;
    self
      .stronghold
      .purge_client(client)
      .map_err(|err| StrongholdError::Client(ClientOperation::Purge, client_path, err))?;

    Ok(true)
  }

  async fn did_exists(&self, did: &CoreDID) -> Result<bool> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let client: Client = self.client(&ClientPath::from(INDEX_CLIENT_PATH))?;
    let store: Store = client.store();

    let dids: BTreeSet<CoreDID> = get_index(&store)?;

    let has_did: bool = dids.contains(did);

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    Ok(has_did)
  }

  async fn did_list(&self) -> Result<Vec<CoreDID>> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let client: Client = self.client(&ClientPath::from(INDEX_CLIENT_PATH))?;
    let store: Store = client.store();

    let dids: BTreeSet<CoreDID> = get_index(&store)?;

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    Ok(dids.into_iter().collect())
  }

  async fn key_generate(&self, did: &CoreDID, key_type: KeyType, fragment: &str) -> Result<KeyLocation> {
    self.mutate_client(did, |client| {
      let tmp_location: KeyLocation = random_location(key_type);

      match key_type {
        KeyType::Ed25519 | KeyType::X25519 => {
          generate_private_key(&client, &tmp_location)?;
        }
      }

      let public_key: PublicKey = retrieve_public_key(&client, &tmp_location)?;
      let location: KeyLocation = KeyLocation::new(key_type, fragment.to_owned(), public_key.as_ref());

      move_key(&client, &tmp_location, &location)?;

      Ok(location)
    })
  }

  async fn key_insert(&self, did: &CoreDID, location: &KeyLocation, private_key: PrivateKey) -> Result<()> {
    self.mutate_client(did, |client| insert_private_key(&client, private_key, location))
  }

  async fn key_public(&self, did: &CoreDID, location: &KeyLocation) -> Result<PublicKey> {
    let client: Client = self.client(&ClientPath::from(did))?;
    retrieve_public_key(&client, location)
  }

  async fn key_delete(&self, did: &CoreDID, location: &KeyLocation) -> Result<bool> {
    self.mutate_client(did, |client| {
      // Technically there is a race condition here between existence check and removal.
      // However, the RevokeData procedure does not return an error if the record doesn't exist, so it's fine.

      let exists: bool = client
        .record_exists(&location.into())
        .map_err(|err| StrongholdError::Vault(VaultOperation::RecordExists, err))
        .map_err(crate::Error::from)?;

      if !exists {
        return Ok(exists);
      }

      client
        .execute_procedure(procedures::RevokeData {
          location: location.into(),
          should_gc: true,
        })
        .map_err(|err| procedure_error::<procedures::RevokeData>(vec![location.clone()], err))
        .map_err(crate::Error::from)?;

      Ok(exists)
    })
  }

  async fn key_sign(&self, did: &CoreDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let client: Client = self.client(&ClientPath::from(did))?;

    match location.key_type {
      KeyType::Ed25519 => sign_ed25519(&client, data, location),
      KeyType::X25519 => Err(identity_did::Error::InvalidMethodType.into()),
    }
  }

  async fn key_exists(&self, did: &CoreDID, location: &KeyLocation) -> Result<bool> {
    let client: Client = self.client(&ClientPath::from(did))?;

    client
      .record_exists(&location.into())
      .map_err(|err| StrongholdError::Vault(VaultOperation::RecordExists, err))
      .map_err(Into::into)
  }

  #[cfg(feature = "encryption")]
  async fn data_encrypt(
    &self,
    did: &CoreDID,
    plaintext: Vec<u8>,
    associated_data: Vec<u8>,
    encryption_algorithm: &EncryptionAlgorithm,
    cek_algorithm: &CekAlgorithm,
    public_key: PublicKey,
  ) -> Result<EncryptedData> {
    // Changes won't be written to the snapshot state since the created keys are temporary
    let client: Client = self.client(&ClientPath::from(did))?;
    let public_key: [u8; X25519::PUBLIC_KEY_LENGTH] = public_key
      .as_ref()
      .try_into()
      .map_err(|_| Error::InvalidPublicKey(format!("expected public key of length {}", X25519::PUBLIC_KEY_LENGTH)))?;
    match cek_algorithm {
      CekAlgorithm::ECDH_ES(agreement) => {
        let (derived_secret, ephemeral_public_key): (Location, PublicKey) =
          diffie_hellman_with_concat_kdf(&client, encryption_algorithm, cek_algorithm, agreement, public_key).await?;
        let encrypted_data: EncryptedData = aead_encrypt(
          &client,
          encryption_algorithm,
          derived_secret,
          plaintext,
          associated_data,
          Vec::new(),
          ephemeral_public_key.as_ref().to_vec(),
        )
        .await?;
        Ok(encrypted_data)
      }
      CekAlgorithm::ECDH_ES_A256KW(agreement) => {
        let (derived_secret, ephemeral_public_key): (Location, PublicKey) =
          diffie_hellman_with_concat_kdf(&client, encryption_algorithm, cek_algorithm, agreement, public_key).await?;
        let cek: Location = generate_content_encryption_key(&client, encryption_algorithm)?;

        let encrypted_cek: Vec<u8> = aes_256_wrap_key(&client, derived_secret, cek.clone())?;

        let encrypted_data: EncryptedData = aead_encrypt(
          &client,
          encryption_algorithm,
          cek,
          plaintext,
          associated_data,
          encrypted_cek,
          ephemeral_public_key.as_ref().to_vec(),
        )
        .await?;
        Ok(encrypted_data)
      }
    }
  }

  #[cfg(feature = "encryption")]
  async fn data_decrypt(
    &self,
    did: &CoreDID,
    data: EncryptedData,
    encryption_algorithm: &EncryptionAlgorithm,
    cek_algorithm: &CekAlgorithm,
    private_key: &KeyLocation,
  ) -> Result<Vec<u8>> {
    // Changes won't be written to the snapshot state since the created keys are temporary
    let client: Client = self.client(&ClientPath::from(did))?;
    let public_key: [u8; X25519::PUBLIC_KEY_LENGTH] = data
      .ephemeral_public_key
      .clone()
      .try_into()
      .map_err(|_| Error::InvalidPublicKey(format!("expected public key of length {}", X25519::PUBLIC_KEY_LENGTH)))?;
    match cek_algorithm {
      CekAlgorithm::ECDH_ES(agreement) => {
        let shared_secret: Location = diffie_hellman(&client, private_key, public_key).await?;
        let derived_secret: Location = concat_kdf(
          &client,
          encryption_algorithm,
          cek_algorithm.name().to_owned(),
          agreement,
          shared_secret,
        )
        .await?;
        aead_decrypt(&client, encryption_algorithm, derived_secret, data).await
      }
      CekAlgorithm::ECDH_ES_A256KW(agreement) => {
        let shared_secret: Location = diffie_hellman(&client, private_key, public_key).await?;
        let derived_secret: Location = concat_kdf(
          &client,
          encryption_algorithm,
          cek_algorithm.name().to_owned(),
          agreement,
          shared_secret,
        )
        .await?;

        let cek: Location = aes_256_unwrap_key(&client, data.encrypted_cek.as_slice(), derived_secret)?;

        aead_decrypt(&client, encryption_algorithm, cek, data).await
      }
    }
  }

  async fn blob_set(&self, did: &CoreDID, blob: Vec<u8>) -> Result<()> {
    self.mutate_client(did, |client| {
      let store: Store = client.store();

      store
        .insert(BLOB_STORE_KEY.as_bytes().to_vec(), blob, None)
        .map(|_| ())
        .map_err(|err| StrongholdError::Store(StoreOperation::Insert, err).into())
    })
  }

  async fn blob_get(&self, did: &CoreDID) -> Result<Option<Vec<u8>>> {
    let client: Client = self.client(&ClientPath::from(did))?;
    let store: Store = client.store();
    let data: Option<Vec<u8>> = store
      .get(BLOB_STORE_KEY.as_bytes())
      .map_err(|err| StrongholdError::Store(StoreOperation::Get, err))?;
    Ok(data)
  }

  async fn flush_changes(&self) -> Result<()> {
    self.persist_snapshot().await?;

    Ok(())
  }
}

impl Drop for Stronghold {
  fn drop(&mut self) {
    if self.dropsave {
      let _ = executor::block_on(self.flush_changes());
    }
  }
}

pub(crate) fn generate_private_key(client: &Client, location: &KeyLocation) -> Result<()> {
  let generate_key: procedures::GenerateKey = procedures::GenerateKey {
    ty: location_key_type(location),
    output: location.into(),
  };

  client
    .execute_procedure(generate_key)
    .map_err(|err| procedure_error::<procedures::GenerateKey>(vec![location.clone()], err))?;

  Ok(())
}

pub(crate) fn insert_private_key(client: &Client, mut private_key: PrivateKey, location: &KeyLocation) -> Result<()> {
  let stronghold_location: Location = location.into();

  let vault: ClientVault = client.vault(stronghold_location.vault_path());

  let private_key_vec: Vec<u8> = private_key.as_ref().to_vec();
  private_key.zeroize();

  vault
    .write_secret(stronghold_location, private_key_vec)
    .map_err(|err| StrongholdError::Vault(VaultOperation::WriteSecret, err))
    .map_err(Into::into)
}

pub(crate) fn retrieve_public_key(client: &Client, location: &KeyLocation) -> Result<PublicKey> {
  match location.key_type {
    KeyType::Ed25519 | KeyType::X25519 => {
      let public_key: procedures::PublicKey = procedures::PublicKey {
        ty: location_key_type(location),
        private_key: location.into(),
      };

      let public = client
        .execute_procedure(public_key)
        .map_err(|err| procedure_error::<procedures::PublicKey>(vec![location.clone()], err))?;

      Ok(public.to_vec().into())
    }
  }
}

fn sign_ed25519(client: &Client, payload: Vec<u8>, location: &KeyLocation) -> Result<Signature> {
  let procedure: procedures::Ed25519Sign = procedures::Ed25519Sign {
    private_key: location.into(),
    msg: payload,
  };

  let signature: [u8; 64] = client
    .execute_procedure(procedure)
    .map_err(|err| procedure_error::<procedures::Ed25519Sign>(vec![location.clone()], err))?;

  Ok(Signature::new(signature.into()))
}

pub(crate) async fn diffie_hellman(
  client: &Client,
  private_key: &KeyLocation,
  public_key: [u8; X25519::PUBLIC_KEY_LENGTH],
) -> Result<Location> {
  let location: [u8; 32] = rand::Rng::gen(&mut rand::thread_rng());
  let shared_key: Location = Location::generic(VAULT_PATH.to_vec(), location.to_vec());
  let diffie_hellman: procedures::X25519DiffieHellman = procedures::X25519DiffieHellman {
    public_key,
    private_key: private_key.into(),
    shared_key: shared_key.clone(),
  };
  client
    .execute_procedure(diffie_hellman)
    .map_err(|err| procedure_error::<procedures::X25519DiffieHellman>(vec![private_key.clone()], err))?;
  Ok(shared_key)
}

pub(crate) async fn concat_kdf(
  client: &Client,
  encryption_algorithm: &EncryptionAlgorithm,
  algorithm_id: String,
  agreement: &AgreementInfo,
  shared_secret: Location,
) -> Result<Location> {
  let location: [u8; 32] = rand::Rng::gen(&mut rand::thread_rng());
  let output: Location = Location::generic(VAULT_PATH.to_vec(), location.to_vec());
  let derived_secret: procedures::ConcatKdf = {
    match encryption_algorithm {
      EncryptionAlgorithm::AES256GCM => procedures::ConcatKdf {
        hash: Sha2Hash::Sha256,
        algorithm_id,
        shared_secret,
        key_len: Aes256Gcm::KEY_LENGTH,
        apu: agreement.apu.clone(),
        apv: agreement.apv.clone(),
        pub_info: agreement.pub_info.clone(),
        priv_info: agreement.priv_info.clone(),
        output: output.clone(),
      },
    }
  };
  client
    .execute_procedure(derived_secret)
    .map_err(|err| procedure_error::<procedures::ConcatKdf>(vec![], err))?;
  Ok(output)
}

pub(crate) async fn aead_encrypt(
  client: &Client,
  algorithm: &EncryptionAlgorithm,
  key: Location,
  plaintext: Vec<u8>,
  associated_data: Vec<u8>,
  encrypted_cek: Vec<u8>,
  ephemeral_public_key: Vec<u8>,
) -> Result<EncryptedData> {
  match algorithm {
    EncryptionAlgorithm::AES256GCM => {
      let nonce: &[u8] = &Aes256Gcm::random_nonce().map_err(Error::EncryptionFailure)?;
      let aead_encrypt: procedures::AeadEncrypt = procedures::AeadEncrypt {
        cipher: procedures::AeadCipher::Aes256Gcm,
        associated_data: associated_data.clone(),
        plaintext,
        nonce: nonce.to_vec(),
        key,
      };
      let mut data = client
        .execute_procedure(aead_encrypt)
        .map_err(|err| procedure_error::<procedures::AeadEncrypt>(vec![], err))?;
      Ok(EncryptedData::new(
        nonce.to_vec(),
        associated_data,
        data.drain(..Aes256Gcm::TAG_LENGTH).collect(),
        data,
        encrypted_cek,
        ephemeral_public_key,
      ))
    }
  }
}

pub(crate) async fn aead_decrypt(
  client: &Client,
  algorithm: &EncryptionAlgorithm,
  key: Location,
  encrypted_data: EncryptedData,
) -> Result<Vec<u8>> {
  match algorithm {
    EncryptionAlgorithm::AES256GCM => {
      let aead_decrypt: procedures::AeadDecrypt = procedures::AeadDecrypt {
        cipher: procedures::AeadCipher::Aes256Gcm,
        key,
        ciphertext: encrypted_data.ciphertext,
        associated_data: encrypted_data.associated_data,
        tag: encrypted_data.tag,
        nonce: encrypted_data.nonce,
      };
      let data = client
        .execute_procedure(aead_decrypt)
        .map_err(|err| procedure_error::<procedures::AeadDecrypt>(vec![], err))?;
      Ok(data)
    }
  }
}

/// Creates an ephemeral pair of X25519 keys, obtains the shared secret by runnning the Diffie-Hellman algorithm and
/// derives key material for use in encryption/decryption through application of the Concatenation Key Derivation
/// function.
///
/// Returns the location of the dervied key material and the ephemeral public key.
async fn diffie_hellman_with_concat_kdf(
  client: &Client,
  encryption_algorithm: &EncryptionAlgorithm,
  cek_algorithm: &CekAlgorithm,
  agreement: &AgreementInfo,
  public_key: [u8; X25519::PUBLIC_KEY_LENGTH],
) -> Result<(Location, PublicKey)> {
  //Generate ephemeral key
  let ephemeral_location = random_location(KeyType::X25519);
  generate_private_key(client, &ephemeral_location)?;
  let ephemeral_public_key: PublicKey = retrieve_public_key(client, &ephemeral_location)?;
  // Obtain the shared secret by combining the ephemeral key and the static public key
  let shared_key: Location = diffie_hellman(client, &ephemeral_location, public_key).await?;
  let derived_secret: Location = concat_kdf(
    client,
    encryption_algorithm,
    cek_algorithm.name().to_owned(),
    agreement,
    shared_key,
  )
  .await?;
  Ok((derived_secret, ephemeral_public_key))
}

/// Generate a random content encryption key with the required length for `encryption_algorithm`.
fn generate_content_encryption_key(client: &Client, encryption_algorithm: &EncryptionAlgorithm) -> Result<Location> {
  let _len: usize = encryption_algorithm.key_length();

  // TODO: X25519 happens to match Aes256Gcm::KEY_LENGTH, but a proper solution is required.
  // See https://github.com/iotaledger/stronghold.rs/issues/374
  let location: KeyLocation = random_location(KeyType::X25519);
  generate_private_key(client, &location)?;

  Ok((&location).into())
}

/// Apply AES256 key wrap to the `cek` using `encryption_key` for encryption.
fn aes_256_wrap_key(client: &Client, encryption_key: Location, cek: Location) -> Result<Vec<u8>> {
  let encrypted_cek: Vec<u8> = client
    .execute_procedure(procedures::AesKeyWrapEncrypt {
      cipher: procedures::AesKeyWrapCipher::Aes256,
      encryption_key,
      wrap_key: cek,
    })
    .map_err(|err| procedure_error::<procedures::AesKeyWrapEncrypt>(vec![], err))?;
  Ok(encrypted_cek)
}

/// Unwrap the given `encrypted_key` using `decryption_key`.
fn aes_256_unwrap_key(client: &Client, encrypted_key: impl AsRef<[u8]>, decryption_key: Location) -> Result<Location> {
  let output: Location = random_stronghold_location();

  client
    .execute_procedure(procedures::AesKeyWrapDecrypt {
      cipher: procedures::AesKeyWrapCipher::Aes256,
      decryption_key,
      wrapped_key: encrypted_key.as_ref().to_vec(),
      output: output.clone(),
    })
    .map_err(|err| procedure_error::<procedures::AesKeyWrapDecrypt>(vec![], err))?;

  Ok(output)
}

// Moves a key from one location to another, deleting the old one.
fn move_key(client: &Client, source: &KeyLocation, target: &KeyLocation) -> Result<()> {
  let source_location: Location = source.into();
  let target_location: Location = target.into();

  let copy_record = procedures::CopyRecord {
    source: source_location.clone(),
    target: target_location,
  };

  client
    .execute_procedure(copy_record)
    .map_err(|err| procedure_error::<procedures::CopyRecord>(vec![source.clone(), target.clone()], err))?;

  let revoke_data = procedures::RevokeData {
    location: source_location,
    should_gc: true,
  };

  client
    .execute_procedure(revoke_data)
    .map_err(|err| procedure_error::<procedures::RevokeData>(vec![source.clone()], err))?;

  Ok(())
}

fn get_index(store: &Store) -> Result<BTreeSet<CoreDID>> {
  let data: Option<Vec<u8>> = store
    .get(INDEX_STORE_KEY.as_bytes())
    .map_err(|err| StrongholdError::Store(StoreOperation::Get, err))?;

  let index: BTreeSet<CoreDID> = match data {
    Some(index_vec) => BTreeSet::<CoreDID>::from_json_slice(&index_vec)?,
    None => BTreeSet::new(),
  };

  Ok(index)
}

fn set_index(store: &Store, index: BTreeSet<CoreDID>) -> Result<()> {
  let index_vec: Vec<u8> = index.to_json_vec()?;

  store
    .insert(INDEX_STORE_KEY.as_bytes().to_vec(), index_vec, None)
    .map_err(|err| StrongholdError::Store(StoreOperation::Insert, err))?;

  Ok(())
}

impl From<&KeyLocation> for Location {
  fn from(key_location: &KeyLocation) -> Self {
    let record_path: Vec<u8> = key_location.canonical().into_bytes();
    Location::generic(VAULT_PATH.to_vec(), record_path)
  }
}

fn location_key_type(location: &KeyLocation) -> procedures::KeyType {
  match location.key_type {
    KeyType::Ed25519 => procedures::KeyType::Ed25519,
    KeyType::X25519 => procedures::KeyType::X25519,
  }
}

pub(crate) fn random_location(key_type: KeyType) -> KeyLocation {
  let mut thread_rng: rand::rngs::ThreadRng = rand::thread_rng();
  let fragment: String = rand::distributions::Alphanumeric.sample_string(&mut thread_rng, 32);
  let public_key: [u8; 32] = rand::Rng::gen(&mut thread_rng);

  KeyLocation::new(key_type, fragment, &public_key)
}

fn random_stronghold_location() -> Location {
  let mut thread_rng: rand::rngs::ThreadRng = rand::thread_rng();
  let record_path: [u8; 32] = rand::Rng::gen(&mut thread_rng);
  Location::generic(VAULT_PATH.to_vec(), record_path.to_vec())
}

fn procedure_error<P>(locations: Vec<KeyLocation>, err: ProcedureError) -> StrongholdError {
  StrongholdError::Procedure(std::any::type_name::<P>(), locations, err)
}
