// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use crypto::ciphers::aes::Aes256Gcm;
use crypto::ciphers::traits::Aead;
use futures::executor;
use iota_stronghold::procedures;
use iota_stronghold::Location;
use rand::distributions::DistString;
use rand::rngs::OsRng;
use rand::Rng;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;
use zeroize::Zeroize;

use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_core::crypto::X25519;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::tangle::NetworkName;

use crate::error::Error;
use crate::error::Result;
use crate::identity::ChainState;
use crate::storage::Storage;
use crate::stronghold::default_hint;
use crate::stronghold::ClientPath;
use crate::stronghold::Context;
use crate::stronghold::Snapshot;
use crate::stronghold::Store;
use crate::stronghold::StrongholdError;
use crate::stronghold::Vault;
use crate::types::CEKAlgorithm;
use crate::types::EncryptedData;
use crate::types::EncryptionAlgorithm;
use crate::types::EncryptionOptions;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::derive_encryption_key;

// The name of the stronghold client used for indexing, which is global for a storage instance.
static INDEX_CLIENT_PATH: &str = "$index";
// The key in the index store that contains the serialized index.
// This happens to be the same as the client path, but for explicitness we define them separately.
static INDEX_STORE_KEY: &str = INDEX_CLIENT_PATH;
static CHAIN_STATE_CLIENT_PATH: &str = "$chain_state";
static DOCUMENT_CLIENT_PATH: &str = "$document";
static VAULT_PATH: &[u8; 6] = b"$vault";

#[derive(Debug)]
pub struct Stronghold {
  snapshot: Arc<Snapshot>,
  // Used to prevent race conditions when updating the index concurrently.
  index_lock: RwLock<()>,
  dropsave: bool,
}

impl Stronghold {
  /// Constructs a Stronghold storage instance.
  ///
  /// Arguments:
  ///
  /// * snapshot: path to a local Stronghold file, will be created if it does not exist.
  /// * password: password for the Stronghold file.
  /// * dropsave: save all changes when the instance is dropped. Default: true.
  pub async fn new<'a, T>(snapshot: &T, mut password: String, dropsave: Option<bool>) -> Result<Self>
  where
    T: AsRef<Path> + ?Sized,
  {
    let snapshot: Snapshot = Snapshot::new(snapshot);
    snapshot.load(derive_encryption_key(&password)).await?;
    password.zeroize();

    Ok(Self {
      snapshot: Arc::new(snapshot),
      index_lock: RwLock::new(()),
      dropsave: dropsave.unwrap_or(true),
    })
  }

  fn store(&self, client_path: ClientPath) -> Store<'_> {
    self.snapshot.store(client_path)
  }

  fn vault(&self, did: &IotaDID) -> Vault<'_> {
    self.snapshot.vault(ClientPath::from(did))
  }

  /// Returns whether save-on-drop is enabled.
  pub fn dropsave(&self) -> bool {
    self.dropsave
  }

  /// Set whether to save the storage changes on drop.
  /// Default: true
  pub fn set_dropsave(&mut self, value: bool) {
    self.dropsave = value;
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl Storage for Stronghold {
  async fn did_create(
    &self,
    network: NetworkName,
    fragment: &str,
    private_key: Option<PrivateKey>,
  ) -> Result<(IotaDID, KeyLocation)> {
    // TODO: Temporarily implemented via in-memory generation as opposed to using stronghold procedures.
    let keypair: KeyPair = match private_key {
      Some(private_key) => KeyPair::try_from_private_key_bytes(KeyType::Ed25519, private_key.as_ref())?,
      None => KeyPair::new(KeyType::Ed25519)?,
    };

    let did: IotaDID = IotaDID::new_with_network(keypair.public().as_ref(), network)
      .map_err(|err| crate::Error::DIDCreationError(err.to_string()))?;

    let index_lock: RwLockWriteGuard<'_, _> = self.index_lock.write().await;
    let store: Store<'_> = self.store(ClientPath::from(INDEX_CLIENT_PATH));
    let mut index: BTreeSet<IotaDID> = get_index(&store).await?;

    if index.contains(&did) {
      return Err(crate::Error::IdentityAlreadyExists);
    } else {
      index.insert(did.clone());
    }

    set_index(&store, index).await?;
    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    let location: KeyLocation = KeyLocation::new(KeyType::Ed25519, fragment.to_owned(), keypair.public().as_ref());
    let vault: Vault<'_> = self.vault(&did);

    vault
      .insert(
        (&location).into(),
        keypair.private().as_ref().to_vec(),
        default_hint(),
        &[],
      )
      .await?;

    Ok((did, location))
  }

  async fn did_purge(&self, did: &IotaDID) -> Result<bool> {
    // Remove index entry if present.
    let index_lock: RwLockWriteGuard<'_, _> = self.index_lock.write().await;
    let store: Store<'_> = self.store(ClientPath::from(INDEX_CLIENT_PATH));
    let mut index: BTreeSet<IotaDID> = get_index(&store).await?;

    if !index.remove(did) {
      return Ok(false);
    }

    set_index(&store, index).await?;
    // Explicitly release the lock early.
    std::mem::drop(index_lock);

    let store: Store<'_> = self.store(ClientPath::from(did));

    store.del(DOCUMENT_CLIENT_PATH).await?;
    store.del(CHAIN_STATE_CLIENT_PATH).await?;

    // TODO: Remove leftover keys (#757).

    Ok(true)
  }

  async fn did_exists(&self, did: &IotaDID) -> Result<bool> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let store: Store<'_> = self.store(ClientPath::from(INDEX_CLIENT_PATH));

    let dids: BTreeSet<IotaDID> = get_index(&store).await?;

    let has_did: bool = dids.contains(did);

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    Ok(has_did)
  }

  async fn did_list(&self) -> Result<Vec<IotaDID>> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let store: Store<'_> = self.store(ClientPath::from(INDEX_CLIENT_PATH));

    let dids: BTreeSet<IotaDID> = get_index(&store).await?;

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    Ok(dids.into_iter().collect())
  }

  async fn key_generate(&self, did: &IotaDID, key_type: KeyType, fragment: &str) -> Result<KeyLocation> {
    let vault: Vault<'_> = self.vault(did);

    let tmp_location: KeyLocation = random_location(key_type);

    match key_type {
      KeyType::Ed25519 | KeyType::X25519 => {
        generate_private_key(&vault, &tmp_location).await?;
      }
    }

    let public_key: PublicKey = self.key_public(did, &tmp_location).await?;

    let location: KeyLocation = KeyLocation::new(key_type, fragment.to_owned(), public_key.as_ref());
    move_key(&vault, (&tmp_location).into(), (&location).into()).await?;

    Ok(location)
  }

  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<()> {
    let vault: Vault<'_> = self.vault(did);

    let stronghold_location: Location = location.into();

    vault
      .insert(stronghold_location, private_key.as_ref(), default_hint(), &[])
      .await?;

    Ok(())
  }

  async fn key_public(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(did);

    match location.key_type {
      KeyType::Ed25519 | KeyType::X25519 => retrieve_public_key(&vault, location).await,
    }
  }

  async fn key_delete(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    // Explicitly implemented to hold the lock across the existence check & removal.
    let context: _ = Context::scope(self.snapshot.path(), ClientPath::from(did).as_ref(), &[]).await?;

    let exists: bool = context
      .record_exists(location.into())
      .await
      .map_err(StrongholdError::from)?;

    context
      .runtime_exec(procedures::RevokeData {
        location: location.into(),
        should_gc: true,
      })
      .await
      .map_err(StrongholdError::from)?
      .map_err(StrongholdError::from)?;

    Ok(exists)
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vault: Vault<'_> = self.vault(did);

    match location.key_type {
      KeyType::Ed25519 => sign_ed25519(&vault, data, location).await,
      KeyType::X25519 => Err(identity_did::Error::InvalidMethodType.into()),
    }
  }

  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    self.vault(did).exists(location.into()).await.map_err(Into::into)
  }

  async fn encrypt_data(
    &self,
    did: &IotaDID,
    data: Vec<u8>,
    associated_data: Vec<u8>,
    encryption_options: &EncryptionOptions,
    private_key: &KeyLocation,
    public_key: PublicKey,
  ) -> Result<EncryptedData> {
    let vault: Vault<'_> = self.vault(did);
    match private_key.key_type {
      KeyType::Ed25519 => Err(Error::InvalidPrivateKey(
        "ED25519 keys are not suported for encrypting/decrypting data".to_owned(),
      )),
      KeyType::X25519 => {
        let public_key: [u8; X25519::PUBLIC_KEY_LENGTH] = public_key
          .as_ref()
          .try_into()
          .map_err(|_| Error::InvalidPublicKey(format!("expected type: [u8, {}]", X25519::PUBLIC_KEY_LENGTH)))?;
        match encryption_options.cek_algorithm() {
          CEKAlgorithm::ECDH_ES => {
            let shared_key: Location = diffie_hellman(&vault, private_key, public_key).await?;
            let encrypted_data: Result<EncryptedData> = aead_encrypt(
              &vault,
              &encryption_options.encryption_algorithm(),
              shared_key.clone(),
              data,
              associated_data,
            )
            .await;
            revoke_data(&vault, shared_key, true).await?;
            encrypted_data
          }
        }
      }
    }
  }

  async fn decrypt_data(
    &self,
    did: &IotaDID,
    data: EncryptedData,
    encryption_options: &EncryptionOptions,
    private_key: &KeyLocation,
    public_key: PublicKey,
  ) -> Result<Vec<u8>> {
    let vault: Vault<'_> = self.vault(did);
    match private_key.key_type {
      KeyType::Ed25519 => Err(Error::InvalidPrivateKey(
        "ED25519 keys are not suported for encrypting/decrypting data".to_owned(),
      )),
      KeyType::X25519 => {
        let public_key: [u8; X25519::PUBLIC_KEY_LENGTH] = public_key
          .as_ref()
          .try_into()
          .map_err(|_| Error::InvalidPublicKey(format!("expected type: [u8, {}]", X25519::PUBLIC_KEY_LENGTH)))?;
        match encryption_options.cek_algorithm() {
          CEKAlgorithm::ECDH_ES => {
            let shared_key: Location = diffie_hellman(&vault, private_key, public_key).await?;
            let decrypted_data: Result<Vec<u8>> = aead_decrypt(
              &vault,
              &encryption_options.encryption_algorithm(),
              shared_key.clone(),
              data,
            )
            .await;
            revoke_data(&vault, shared_key, true).await?;
            decrypted_data
          }
        }
      }
    }
  }

  async fn chain_state_get(&self, did: &IotaDID) -> Result<Option<ChainState>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(ClientPath::from(did));
    let data: Option<Vec<u8>> = store.get(CHAIN_STATE_CLIENT_PATH).await?;

    match data {
      None => return Ok(None),
      Some(data) => Ok(Some(ChainState::from_json_slice(&data)?)),
    }
  }

  async fn chain_state_set(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(ClientPath::from(did));

    let json: Vec<u8> = chain_state.to_json_vec()?;

    store.set(CHAIN_STATE_CLIENT_PATH, json, None).await?;

    Ok(())
  }

  async fn document_get(&self, did: &IotaDID) -> Result<Option<IotaDocument>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(ClientPath::from(did));

    // Read the state from the stronghold snapshot
    let data: Option<Vec<u8>> = store.get(DOCUMENT_CLIENT_PATH).await?;

    match data {
      None => return Ok(None),
      Some(data) => Ok(Some(IotaDocument::from_json_slice(&data)?)),
    }
  }

  async fn document_set(&self, did: &IotaDID, document: &IotaDocument) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(ClientPath::from(did));

    // Serialize the state
    let json: Vec<u8> = document.to_json_vec()?;

    // Write the state to the stronghold snapshot
    store.set(DOCUMENT_CLIENT_PATH, json, None).await?;

    Ok(())
  }

  async fn flush_changes(&self) -> Result<()> {
    self.snapshot.save().await?;
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

async fn generate_private_key(vault: &Vault<'_>, location: &KeyLocation) -> Result<()> {
  let generate_key: procedures::GenerateKey = procedures::GenerateKey {
    ty: location_key_type(location),
    output: location.into(),
    hint: default_hint(),
  };

  vault.execute(generate_key).await?;

  Ok(())
}

async fn retrieve_public_key(vault: &Vault<'_>, location: &KeyLocation) -> Result<PublicKey> {
  let procedure: procedures::PublicKey = procedures::PublicKey {
    ty: location_key_type(location),
    private_key: location.into(),
  };

  Ok(vault.execute(procedure).await.map(|public| public.to_vec().into())?)
}

async fn sign_ed25519(vault: &Vault<'_>, payload: Vec<u8>, location: &KeyLocation) -> Result<Signature> {
  let procedure: procedures::Ed25519Sign = procedures::Ed25519Sign {
    private_key: location.into(),
    msg: payload,
  };
  let signature: [u8; 64] = vault.execute(procedure).await?;
  Ok(Signature::new(signature.into()))
}

async fn diffie_hellman(
  vault: &Vault<'_>,
  private_key: &KeyLocation,
  public_key: [u8; X25519::PUBLIC_KEY_LENGTH],
) -> Result<Location> {
  let location: [u8; 32] = OsRng.sample(rand::distributions::Standard);
  let shared_key: Location = Location::generic(VAULT_PATH.to_vec(), location.to_vec());
  let diffie_hellman: procedures::X25519DiffieHellman = procedures::X25519DiffieHellman {
    public_key,
    private_key: private_key.into(),
    shared_key: shared_key.clone(),
    hint: default_hint(),
  };
  vault.execute(diffie_hellman).await?;
  Ok(shared_key)
}

async fn aead_encrypt(
  vault: &Vault<'_>,
  algorithm: &EncryptionAlgorithm,
  key: Location,
  plaintext: Vec<u8>,
  associated_data: Vec<u8>,
) -> Result<EncryptedData> {
  match algorithm {
    EncryptionAlgorithm::Aes256Gcm => {
      let nonce: &[u8] = &Aes256Gcm::random_nonce().map_err(Error::NonceGenerationFailed)?;
      let aead_encrypt: procedures::AeadEncrypt = procedures::AeadEncrypt {
        cipher: procedures::AeadCipher::Aes256Gcm,
        associated_data: associated_data.clone(),
        plaintext,
        nonce: nonce.to_vec(),
        key,
      };
      let mut data = vault.execute(aead_encrypt).await?;
      Ok(EncryptedData::new(
        nonce.to_vec(),
        associated_data,
        data.drain(..Aes256Gcm::TAG_LENGTH).collect(),
        data,
      ))
    }
  }
}

async fn aead_decrypt(
  vault: &Vault<'_>,
  algorithm: &EncryptionAlgorithm,
  key: Location,
  encrypted_data: EncryptedData,
) -> Result<Vec<u8>> {
  match algorithm {
    EncryptionAlgorithm::Aes256Gcm => {
      let aead_decrypt: procedures::AeadDecrypt = procedures::AeadDecrypt {
        cipher: procedures::AeadCipher::Aes256Gcm,
        key,
        ciphertext: encrypted_data.ciphertext().to_vec(),
        associated_data: encrypted_data.associated_data().to_vec(),
        tag: encrypted_data.tag().to_vec(),
        nonce: encrypted_data.nonce().to_vec(),
      };
      vault.execute(aead_decrypt).await.map_err(Into::into)
    }
  }
}

async fn revoke_data<T: Into<Location>>(vault: &Vault<'_>, location: T, should_gc: bool) -> Result<()> {
  let revoke_data: procedures::RevokeData = procedures::RevokeData {
    location: location.into(),
    should_gc,
  };
  vault.execute(revoke_data).await.map_err(Into::into)
}

// Moves a key from one location to another, deleting the old one.
async fn move_key(vault: &Vault<'_>, source: Location, target: Location) -> Result<()> {
  let copy_record = procedures::CopyRecord {
    source: source.clone(),
    target,
    hint: default_hint(),
  };

  vault.execute(copy_record).await?;

  let revoke_data = procedures::RevokeData {
    location: source,
    should_gc: true,
  };

  vault.execute(revoke_data).await?;

  Ok(())
}

async fn get_index(store: &Store<'_>) -> Result<BTreeSet<IotaDID>> {
  let index: BTreeSet<IotaDID> = match store.get(INDEX_STORE_KEY).await? {
    Some(index_vec) => BTreeSet::<IotaDID>::from_json_slice(&index_vec)?,
    None => BTreeSet::new(),
  };

  Ok(index)
}

async fn set_index(store: &Store<'_>, index: BTreeSet<IotaDID>) -> Result<()> {
  let index_vec: Vec<u8> = index.to_json_vec()?;

  store.set(INDEX_STORE_KEY, index_vec, None).await?;

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

fn random_location(key_type: KeyType) -> KeyLocation {
  // NOTE: do not use rand::thread_rng() or rand::random(), breaks musl-libc cross-compilation.
  let fragment: String = rand::distributions::Alphanumeric.sample_string(&mut OsRng, 32);
  let public_key: [u8; 32] = OsRng.sample(rand::distributions::Standard);

  KeyLocation::new(key_type, fragment, &public_key)
}
