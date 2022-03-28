// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use futures::executor;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use iota_stronghold::procedures;
use iota_stronghold::Location;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use crate::error::Result;
use crate::identity::ChainState;
use crate::storage::Storage;
use crate::stronghold::default_hint;
use crate::stronghold::Context;
use crate::stronghold::Snapshot;
use crate::stronghold::Store;
use crate::stronghold::StrongholdError;
use crate::stronghold::Vault;
use crate::types::AccountId;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::derive_encryption_key;

// The name of the stronghold client used for indexing, which is global for a storage instance.
static INDEX_PATH: &str = "$index";
static CHAIN_STATE_PATH: &str = "$chain_state";
static DOCUMENT_PATH: &str = "$document";

#[derive(Debug)]
pub struct Stronghold {
  snapshot: Arc<Snapshot>,
  // Used to prevent race conditions when updating the index concurrently.
  index_lock: RwLock<()>,
  dropsave: bool,
}

impl Stronghold {
  pub async fn new<'a, T, U>(snapshot: &T, password: U, dropsave: Option<bool>) -> Result<Self>
  where
    T: AsRef<Path> + ?Sized,
    U: Into<Option<&'a str>>,
  {
    let snapshot: Snapshot = Snapshot::new(snapshot);

    if let Some(password) = password.into() {
      snapshot.load(derive_encryption_key(password)).await?;
    }

    Ok(Self {
      snapshot: Arc::new(snapshot),
      index_lock: RwLock::new(()),
      dropsave: dropsave.unwrap_or(true),
    })
  }

  fn store(&self, name: &str) -> Store<'_> {
    self.snapshot.store(name, &[])
  }

  fn vault(&self, id: &AccountId) -> Vault<'_> {
    self.snapshot.vault(&fmt_account_id(id), &[])
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
  async fn key_generate(&self, account_id: &AccountId, key_type: KeyType, fragment: &str) -> Result<KeyLocation> {
    let vault: Vault<'_> = self.vault(account_id);

    let tmp_location: KeyLocation = KeyLocation::random(key_type);

    match key_type {
      KeyType::Ed25519 | KeyType::X25519 => {
        generate_private_key(&vault, &tmp_location).await?;
      }
    }

    let public_key: PublicKey = self.key_public(account_id, &tmp_location).await?;

    let location: KeyLocation = KeyLocation::new(key_type, fragment.to_owned(), public_key.as_ref());
    move_key(&vault, (&tmp_location).into(), (&location).into()).await?;

    Ok(location)
  }

  async fn key_insert(&self, account_id: &AccountId, location: &KeyLocation, private_key: PrivateKey) -> Result<()> {
    let vault = self.vault(account_id);

    let stronghold_location: Location = location.into();

    vault
      .insert(stronghold_location, private_key.as_ref(), default_hint(), &[])
      .await?;

    Ok(())
  }

  async fn key_public(&self, account_id: &AccountId, location: &KeyLocation) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(account_id);

    match location.key_type {
      KeyType::Ed25519 | KeyType::X25519 => retrieve_public_key(&vault, location).await,
    }
  }

  async fn key_delete(&self, account_id: &AccountId, location: &KeyLocation) -> Result<bool> {
    // Explicitly implemented to hold the lock across the existence check & removal.
    let context: _ = Context::scope(self.snapshot.path(), fmt_account_id(account_id).as_ref(), &[]).await?;

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

  async fn key_sign(&self, account_id: &AccountId, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vault: Vault<'_> = self.vault(account_id);

    match location.key_type {
      KeyType::Ed25519 => sign_ed25519(&vault, data, location).await,
      KeyType::X25519 => Err(identity_did::Error::InvalidMethodType.into()),
    }
  }

  async fn key_exists(&self, account_id: &AccountId, location: &KeyLocation) -> Result<bool> {
    self.vault(account_id).exists(location.into()).await.map_err(Into::into)
  }

  async fn chain_state_get(&self, account_id: &AccountId) -> Result<Option<ChainState>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_account_id(account_id));
    let data: Option<Vec<u8>> = store.get(CHAIN_STATE_PATH).await?;

    match data {
      None => return Ok(None),
      Some(data) => Ok(Some(ChainState::from_json_slice(&data)?)),
    }
  }

  async fn chain_state_set(&self, account_id: &AccountId, chain_state: &ChainState) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_account_id(account_id));

    let json: Vec<u8> = chain_state.to_json_vec()?;

    store.set(CHAIN_STATE_PATH, json, None).await?;

    Ok(())
  }

  async fn document_get(&self, account_id: &AccountId) -> Result<Option<IotaDocument>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_account_id(account_id));

    // Read the state from the stronghold snapshot
    let data: Option<Vec<u8>> = store.get(DOCUMENT_PATH).await?;

    match data {
      None => return Ok(None),
      Some(data) => Ok(Some(IotaDocument::from_json_slice(&data)?)),
    }
  }

  async fn document_set(&self, account_id: &AccountId, document: &IotaDocument) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_account_id(account_id));

    // Serialize the state
    let json: Vec<u8> = document.to_json_vec()?;

    // Write the state to the stronghold snapshot
    store.set(DOCUMENT_PATH, json, None).await?;

    Ok(())
  }

  async fn purge(&self, did: &IotaDID) -> Result<bool> {
    let account_id: AccountId = match self.index_get(did).await? {
      Some(account_id) => account_id,
      None => return Ok(false),
    };

    // Remove index entry.
    let store: Store<'_> = self.store(INDEX_PATH);
    let index_lock: RwLockWriteGuard<'_, _> = self.index_lock.write().await;
    let mut index: HashMap<IotaDID, AccountId> = get_index(&store).await?;
    if index.remove(did).is_none() {
      return Ok(false);
    }
    set_index(&store, index).await?;
    // Explicitly release the lock early.
    std::mem::drop(index_lock);

    let store: Store<'_> = self.store(&fmt_account_id(&account_id));

    store.del(DOCUMENT_PATH).await?;
    store.del(CHAIN_STATE_PATH).await?;

    // TODO: Remove leftover keys.

    Ok(true)
  }

  async fn index_set(&self, did: IotaDID, account_id: AccountId) -> Result<()> {
    let index_lock: RwLockWriteGuard<'_, _> = self.index_lock.write().await;

    let store: Store<'_> = self.store(INDEX_PATH);

    let mut index: HashMap<IotaDID, AccountId> = get_index(&store).await?;

    index.insert(did, account_id);

    set_index(&store, index).await?;

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    Ok(())
  }

  async fn index_get(&self, did: &IotaDID) -> Result<Option<AccountId>> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let store: Store<'_> = self.store(INDEX_PATH);

    let index: HashMap<IotaDID, AccountId> = get_index(&store).await?;

    let lookup: Option<AccountId> = index.get(did).cloned();

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    Ok(lookup)
  }

  async fn index_keys(&self) -> Result<Vec<IotaDID>> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let store: Store<'_> = self.store(INDEX_PATH);

    let index: HashMap<IotaDID, AccountId> = get_index(&store).await?;

    let dids: Vec<IotaDID> = index.keys().cloned().collect();

    // Explicitly drop the lock so it's not considered unused.
    std::mem::drop(index_lock);

    Ok(dids)
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

async fn get_index(store: &Store<'_>) -> Result<HashMap<IotaDID, AccountId>> {
  let index: HashMap<IotaDID, AccountId> = match store.get(INDEX_PATH).await? {
    Some(index_vec) => HashMap::<IotaDID, AccountId>::from_json_slice(&index_vec)?,
    None => HashMap::new(),
  };

  Ok(index)
}

async fn set_index(store: &Store<'_>, index: HashMap<IotaDID, AccountId>) -> Result<()> {
  let index_vec: Vec<u8> = index.to_json_vec()?;

  store.set(INDEX_PATH, index_vec, None).await?;

  Ok(())
}

fn fmt_account_id(account_id: &AccountId) -> String {
  format!("$identity:{}", account_id)
}

impl From<&KeyLocation> for Location {
  fn from(key_location: &KeyLocation) -> Self {
    Location::generic(b"$vault".to_vec(), key_location.to_string().into_bytes())
  }
}

fn location_key_type(location: &KeyLocation) -> procedures::KeyType {
  match location.key_type {
    KeyType::Ed25519 => procedures::KeyType::Ed25519,
    KeyType::X25519 => procedures::KeyType::X25519,
  }
}
