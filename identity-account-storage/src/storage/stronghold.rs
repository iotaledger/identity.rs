// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::type_name;
use std::collections::BTreeSet;

use async_trait::async_trait;
use futures::executor;
use iota_stronghold::sync::MergePolicy;
use iota_stronghold::sync::SyncClientsConfig;
use iota_stronghold::Client;
use iota_stronghold::ClientVault;
use iota_stronghold::Location;
use iota_stronghold::Store;
use iota_stronghold::StoreGuard;
use stronghold_engine::vault::RecordHint;
use zeroize::Zeroize;

use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::tangle::NetworkName;
use iota_stronghold::procedures;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use crate::error::Result;
use crate::identity::ChainState;
use crate::storage::Storage;
use crate::stronghold::ClientOperation;
use crate::stronghold::ClientPath;
use crate::stronghold::StoreOperation;
use crate::stronghold::Stronghold;
use crate::stronghold::StrongholdError;
use crate::stronghold::VaultOperation;
use crate::types::KeyLocation;
use crate::types::Signature;

// The name of the stronghold client used for indexing, which is global for a storage instance.
static INDEX_CLIENT_PATH: &str = "$index";
// The key in the index store that contains the serialized index.
// This happens to be the same as the client path, but for explicitness we define them separately.
static INDEX_STORE_KEY: &str = INDEX_CLIENT_PATH;
static CHAIN_STATE_CLIENT_PATH: &str = "$chain_state";
static DOCUMENT_CLIENT_PATH: &str = "$document";
static VAULT_PATH: &[u8; 6] = b"$vault";

// #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
// #[cfg_attr(feature = "send-sync-storage", async_trait)]

// TODO: Temporarily do not require future to be send due to lack of Send-Futures in current Stronghold.
#[async_trait(?Send)]
impl Storage for Stronghold {
  async fn did_create(
    &self,
    network: NetworkName,
    fragment: &str,
    private_key: Option<PrivateKey>,
  ) -> Result<(IotaDID, KeyLocation)> {
    // =============================
    // KEY GENERATION/INSERTION
    // =============================

    let tmp_client: Client = Client::default();
    let tmp_location: KeyLocation = random_location(KeyType::Ed25519);

    match private_key {
      Some(private_key) => {
        insert_private_key(&tmp_client, private_key, &tmp_location).await?;
      }
      None => {
        generate_private_key(&tmp_client, &tmp_location).await?;
      }
    }

    let public_key: PublicKey = retrieve_public_key(&tmp_client, &tmp_location).await?;

    let did: IotaDID = IotaDID::new_with_network(public_key.as_ref(), network)
      .map_err(|err| crate::Error::DIDCreationError(err.to_string()))?;

    // =============================
    // ADD DID TO INDEX
    // =============================

    // TODO: Do we have to roll back changes to a client if the entire did_create "transaction"
    // doesn't complete successfully? Otherwise, someone re-trying to create an identity from an existing
    // keypair might fail every time, because the DID was added to the index, but e.g. the client syncing later failed.

    let index_lock: RwLockWriteGuard<'_, _> = self.index_lock.write().await;

    let index_client: Client = self.client(&ClientPath::from(INDEX_CLIENT_PATH)).await?;
    let index_store: Store = index_client.store().await;

    let mut index: BTreeSet<IotaDID> = get_index(&index_store).await?;

    if index.contains(&did) {
      return Err(crate::Error::IdentityAlreadyExists);
    } else {
      index.insert(did.clone());
    }

    set_index(&index_store, index).await?;

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    // =============================
    // CLIENT SYNC & KEY MOVE
    // =============================
    let location: KeyLocation = KeyLocation::new(KeyType::Ed25519, fragment.to_owned(), public_key.as_ref());

    let client_path: ClientPath = ClientPath::from(&did);
    let client: Client = self.client(&client_path).await?;

    let mut sync_config: SyncClientsConfig = SyncClientsConfig::new(MergePolicy::Replace);
    sync_config.sync_selected_vaults(vec![VAULT_PATH.to_vec()]);

    client.sync_with(&tmp_client, sync_config).expect("TODO");
    std::mem::drop(tmp_client);

    move_key(&client, &tmp_location, &location).await?;

    self
      .stronghold
      .write_client(client_path.as_ref())
      .await
      .map_err(|err| StrongholdError::ClientError(ClientOperation::Persist, client_path.clone(), err))?;

    Ok((did, location))
  }

  async fn did_purge(&self, did: &IotaDID) -> Result<bool> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let index_client: Client = self.client(&ClientPath::from(INDEX_CLIENT_PATH)).await?;
    let index_store: Store = index_client.store().await;

    let mut index: BTreeSet<IotaDID> = get_index(&index_store).await?;

    // Remove index entry if present.
    if !index.remove(did) {
      return Ok(false);
    }

    set_index(&index_store, index).await?;
    // Explicitly release the lock early.
    std::mem::drop(index_lock);

    // TODO: Delete the client from stronghold, there should be a function for that?

    Ok(true)
  }

  async fn did_exists(&self, did: &IotaDID) -> Result<bool> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let client: Client = self.client(&ClientPath::from(INDEX_CLIENT_PATH)).await?;
    let store: Store = client.store().await;

    let dids: BTreeSet<IotaDID> = get_index(&store).await?;

    let has_did: bool = dids.contains(did);

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    Ok(has_did)
  }

  async fn did_list(&self) -> Result<Vec<IotaDID>> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let client: Client = self.client(&ClientPath::from(INDEX_CLIENT_PATH)).await?;
    let store: Store = client.store().await;

    let dids: BTreeSet<IotaDID> = get_index(&store).await?;

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    Ok(dids.into_iter().collect())
  }

  async fn key_generate(&self, did: &IotaDID, key_type: KeyType, fragment: &str) -> Result<KeyLocation> {
    self
      .mutate_client(did, |client| async move {
        let tmp_location: KeyLocation = random_location(key_type);

        match key_type {
          KeyType::Ed25519 | KeyType::X25519 => {
            generate_private_key(&client, &tmp_location).await?;
          }
        }

        let public_key: PublicKey = retrieve_public_key(&client, &tmp_location).await?;
        let location: KeyLocation = KeyLocation::new(key_type, fragment.to_owned(), public_key.as_ref());

        move_key(&client, &tmp_location, &location).await?;

        Ok(location)
      })
      .await
  }

  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<()> {
    self
      .mutate_client(did, |client| async move {
        insert_private_key(&client, private_key, location).await
      })
      .await
  }

  async fn key_public(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    let client: Client = self.client(&ClientPath::from(did)).await?;
    retrieve_public_key(&client, location).await
  }

  async fn key_delete(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    self
      .mutate_client(did, |client| async move {
        // TODO: Technically there is a race condition here between existence check and removal.
        // Check if RevokeData returns an error if the record doesn't exist, and if so, ignore it.

        let exists: bool = client
          .record_exists(location.into())
          .await
          .map_err(|err| StrongholdError::VaultError(VaultOperation::RecordExists, err))
          .map_err(crate::Error::from)?;

        client
          .execute_procedure(procedures::RevokeData {
            location: location.into(),
            should_gc: true,
          })
          .await
          .map_err(|err| {
            StrongholdError::ProcedureError(type_name::<procedures::RevokeData>(), vec![location.clone()], err)
          })
          .map_err(crate::Error::from)?;

        Ok(exists)
      })
      .await
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let client: Client = self.client(&ClientPath::from(did)).await?;

    match location.key_type {
      KeyType::Ed25519 => sign_ed25519(&client, data, location).await,
      KeyType::X25519 => Err(identity_did::Error::InvalidMethodType.into()),
    }
  }

  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    let client: Client = self.client(&ClientPath::from(did)).await?;

    client
      .record_exists(location.into())
      .await
      .map_err(|err| StrongholdError::VaultError(VaultOperation::RecordExists, err))
      .map_err(Into::into)
  }

  async fn chain_state_get(&self, did: &IotaDID) -> Result<Option<ChainState>> {
    let client: Client = self.client(&ClientPath::from(did)).await?;
    let store: Store = client.store().await;

    let data: StoreGuard<'_> = store
      .get(CHAIN_STATE_CLIENT_PATH.as_bytes().to_vec())
      .map_err(|err| StrongholdError::StoreError(StoreOperation::Get, err))?;

    match data.deref() {
      None => return Ok(None),
      Some(data) => Ok(Some(ChainState::from_json_slice(data)?)),
    }
  }

  async fn chain_state_set(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()> {
    let json: Vec<u8> = chain_state.to_json_vec()?;

    self
      .mutate_client(did, |client| async move {
        let store: Store = client.store().await;

        store
          .insert(CHAIN_STATE_CLIENT_PATH.as_bytes().to_vec(), json, None)
          .map_err(|err| StrongholdError::StoreError(StoreOperation::Insert, err).into())
      })
      .await
  }

  async fn document_get(&self, did: &IotaDID) -> Result<Option<IotaDocument>> {
    let client: Client = self.client(&ClientPath::from(did)).await?;
    let store: Store = client.store().await;

    let data: StoreGuard<'_> = store
      .get(DOCUMENT_CLIENT_PATH.as_bytes().to_vec())
      .map_err(|err| StrongholdError::StoreError(StoreOperation::Get, err))?;

    match data.deref() {
      None => return Ok(None),
      Some(data) => Ok(Some(IotaDocument::from_json_slice(data)?)),
    }
  }

  async fn document_set(&self, did: &IotaDID, document: &IotaDocument) -> Result<()> {
    let json: Vec<u8> = document.to_json_vec()?;

    self
      .mutate_client(did, |client| async move {
        let store: Store = client.store().await;

        store
          .insert(DOCUMENT_CLIENT_PATH.as_bytes().to_vec(), json, None)
          .map_err(|err| StrongholdError::StoreError(StoreOperation::Insert, err).into())
      })
      .await
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

async fn generate_private_key(client: &Client, location: &KeyLocation) -> Result<()> {
  let generate_key: procedures::GenerateKey = procedures::GenerateKey {
    ty: location_key_type(location),
    output: location.into(),
    hint: default_hint(),
  };

  client.execute_procedure(generate_key).await.map_err(|err| {
    StrongholdError::ProcedureError(type_name::<procedures::GenerateKey>(), vec![location.clone()], err)
  })?;

  Ok(())
}

async fn insert_private_key(client: &Client, mut private_key: PrivateKey, location: &KeyLocation) -> Result<()> {
  let stronghold_location: Location = location.into();

  // TODO: Hopefully this is fixed. Client::vault should only require a vault_path.
  let vault: ClientVault = client.vault(stronghold_location.clone());

  let private_key_vec: Vec<u8> = private_key.as_ref().to_vec();
  private_key.zeroize();

  vault
    .write_secret(stronghold_location, private_key_vec)
    .map_err(|err| StrongholdError::VaultError(VaultOperation::WriteSecret, err))
    .map_err(Into::into)
}

async fn retrieve_public_key(client: &Client, location: &KeyLocation) -> Result<PublicKey> {
  match location.key_type {
    KeyType::Ed25519 | KeyType::X25519 => {
      let public_key: procedures::PublicKey = procedures::PublicKey {
        ty: location_key_type(location),
        private_key: location.into(),
      };

      let public = client.execute_procedure(public_key).await.map_err(|err| {
        StrongholdError::ProcedureError(type_name::<procedures::PublicKey>(), vec![location.clone()], err)
      })?;

      Ok(public.to_vec().into())
    }
  }
}

async fn sign_ed25519(client: &Client, payload: Vec<u8>, location: &KeyLocation) -> Result<Signature> {
  let procedure: procedures::Ed25519Sign = procedures::Ed25519Sign {
    private_key: location.into(),
    msg: payload,
  };

  let signature: [u8; 64] = client.execute_procedure(procedure).await.map_err(|err| {
    StrongholdError::ProcedureError(type_name::<procedures::Ed25519Sign>(), vec![location.clone()], err)
  })?;

  Ok(Signature::new(signature.into()))
}

// Moves a key from one location to another, deleting the old one.
async fn move_key(client: &Client, source: &KeyLocation, target: &KeyLocation) -> Result<()> {
  let source_location: Location = source.into();
  let target_location: Location = target.into();

  let copy_record = procedures::CopyRecord {
    source: source_location.clone(),
    target: target_location,
    hint: default_hint(),
  };

  client.execute_procedure(copy_record).await.map_err(|err| {
    StrongholdError::ProcedureError(
      type_name::<procedures::CopyRecord>(),
      vec![source.clone(), target.clone()],
      err,
    )
  })?;

  let revoke_data = procedures::RevokeData {
    location: source_location,
    should_gc: true,
  };

  client
    .execute_procedure(revoke_data)
    .await
    .map_err(|err| StrongholdError::ProcedureError(type_name::<procedures::RevokeData>(), vec![source.clone()], err))?;

  Ok(())
}

async fn get_index(store: &Store) -> Result<BTreeSet<IotaDID>> {
  let data: StoreGuard<'_> = store
    .get(INDEX_STORE_KEY.as_bytes().to_vec())
    .map_err(|err| StrongholdError::StoreError(StoreOperation::Get, err))?;

  let index: BTreeSet<IotaDID> = match data.deref() {
    Some(index_vec) => BTreeSet::<IotaDID>::from_json_slice(index_vec)?,
    None => BTreeSet::new(),
  };

  Ok(index)
}

async fn set_index(store: &Store, index: BTreeSet<IotaDID>) -> Result<()> {
  let index_vec: Vec<u8> = index.to_json_vec()?;

  store
    .insert(INDEX_STORE_KEY.as_bytes().to_vec(), index_vec, None)
    .map_err(|err| StrongholdError::StoreError(StoreOperation::Insert, err))?;

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
  let mut thread_rng: rand::rngs::ThreadRng = rand::thread_rng();
  let fragment: String = rand::Rng::sample_iter(&mut thread_rng, rand::distributions::Alphanumeric)
    .take(32)
    .map(char::from)
    .collect();
  let public_key: [u8; 32] = rand::Rng::gen(&mut thread_rng);

  KeyLocation::new(key_type, fragment, &public_key)
}

fn default_hint() -> RecordHint {
  // unwrap is okay, the hint is <= 24 bytes
  RecordHint::new([0; 24]).unwrap()
}
