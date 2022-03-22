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
use crate::stronghold::Snapshot;
use crate::stronghold::Store;
use crate::stronghold::Vault;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::derive_encryption_key;

use super::AccountId;
use super::StoreKey;

// The name of the stronghold client used for indexing, which is global for a storage instance.
static INDEX_PATH: &str = "$index";

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
  async fn key_generate(&self, account_id: &AccountId, location: &KeyLocation) -> Result<()> {
    let vault: Vault<'_> = self.vault(account_id);

    match location.key_type {
      KeyType::Ed25519 => generate_ed25519(&vault, location).await?,
    }

    Ok(())
  }

  // TODO: Let take reference to account id?
  async fn key_insert(&self, account_id: &AccountId, location: &KeyLocation, private_key: PrivateKey) -> Result<()> {
    let vault = self.vault(account_id);

    let stronghold_location: Location = location.into();

    vault
      .insert(stronghold_location, private_key.as_ref(), default_hint(), &[])
      .await?;

    Ok(())
  }

  async fn key_move(&self, account_id: &AccountId, source: &KeyLocation, target: &KeyLocation) -> Result<()> {
    let vault: Vault<'_> = self.vault(account_id);

    move_key(&vault, source.into(), target.into()).await
  }

  async fn key_public(&self, account_id: &AccountId, location: &KeyLocation) -> Result<PublicKey> {
    let vault: Vault<'_> = self.vault(account_id);

    match location.key_type {
      KeyType::Ed25519 => retrieve_ed25519(&vault, location.into()).await,
    }
  }

  async fn key_del(&self, account_id: &AccountId, location: &KeyLocation) -> Result<()> {
    let vault: Vault<'_> = self.vault(account_id);

    match location.key_type {
      KeyType::Ed25519 => {
        vault.delete(location.into(), true).await?;
      }
    }

    Ok(())
  }

  async fn key_sign(&self, account_id: &AccountId, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vault: Vault<'_> = self.vault(account_id);

    match location.key_type {
      KeyType::Ed25519 => sign_ed25519(&vault, data, location.into()).await,
    }
  }

  async fn key_exists(&self, account_id: &AccountId, location: &KeyLocation) -> Result<bool> {
    self.vault(account_id).exists(location.into()).await.map_err(Into::into)
  }

  async fn chain_state(&self, account_id: &AccountId) -> Result<Option<ChainState>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_account_id(account_id));
    let data: Option<Vec<u8>> = store.get(location_chain_state()).await?;

    match data {
      None => return Ok(None),
      Some(data) => Ok(Some(ChainState::from_json_slice(&data)?)),
    }
  }

  async fn set_chain_state(&self, account_id: &AccountId, chain_state: &ChainState) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_account_id(account_id));

    let json: Vec<u8> = chain_state.to_json_vec()?;

    store.set(location_chain_state(), json, None).await?;

    Ok(())
  }

  async fn document(&self, account_id: &AccountId) -> Result<Option<IotaDocument>> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_account_id(account_id));

    // Read the state from the stronghold snapshot
    let data: Option<Vec<u8>> = store.get(location_document()).await?;

    match data {
      None => return Ok(None),
      Some(data) => Ok(Some(IotaDocument::from_json_slice(&data)?)),
    }
  }

  async fn set_document(&self, account_id: &AccountId, document: &IotaDocument) -> Result<()> {
    // Load the chain-specific store
    let store: Store<'_> = self.store(&fmt_account_id(account_id));

    // Serialize the state
    let json: Vec<u8> = document.to_json_vec()?;

    // Write the state to the stronghold snapshot
    store.set(location_document(), json, None).await?;

    Ok(())
  }

  async fn purge(&self, did: &IotaDID) -> Result<()> {
    let account_id: AccountId = match self.index_get(did).await? {
      Some(account_id) => account_id,
      None => return Ok(()),
    };

    // Remove index entry.
    let store: Store<'_> = self.store(INDEX_PATH);
    let index_lock: RwLockWriteGuard<'_, _> = self.index_lock.write().await;
    let mut index = get_index(&store).await?;
    index.remove(did);
    set_index(&store, index).await?;
    std::mem::drop(index_lock);

    let store: Store<'_> = self.store(&fmt_account_id(&account_id));

    store.del(location_document()).await?;
    store.del(location_chain_state()).await?;

    // TODO: Remove leftover keys.

    Ok(())
  }

  async fn index_set(&self, did: IotaDID, account_id: AccountId) -> Result<()> {
    let index_lock: RwLockWriteGuard<'_, _> = self.index_lock.write().await;

    let store: Store<'_> = self.store(INDEX_PATH);

    let mut index = get_index(&store).await?;

    index.insert(did, account_id);

    set_index(&store, index).await?;

    std::mem::drop(index_lock);

    Ok(())
  }

  async fn index_get(&self, did: &IotaDID) -> Result<Option<AccountId>> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let store: Store<'_> = self.store(INDEX_PATH);

    let index = get_index(&store).await?;

    let lookup: Option<AccountId> = index.get(did).cloned();

    // Explicitly drop the lock so it's not considered unused,
    // and possibly optimized away.
    std::mem::drop(index_lock);

    Ok(lookup)
  }

  async fn index(&self) -> Result<Vec<IotaDID>> {
    let index_lock: RwLockReadGuard<'_, _> = self.index_lock.read().await;

    let store: Store<'_> = self.store(INDEX_PATH);

    let index = get_index(&store).await?;

    let dids: Vec<IotaDID> = index.keys().cloned().collect();

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

async fn generate_ed25519(vault: &Vault<'_>, location: &KeyLocation) -> Result<()> {
  let location: Location = location.into();

  let generate_key: procedures::GenerateKey = procedures::GenerateKey {
    ty: procedures::KeyType::Ed25519,
    output: location,
    hint: default_hint(),
  };

  vault.execute(generate_key).await?;

  Ok(())
}

async fn retrieve_ed25519(vault: &Vault<'_>, location: Location) -> Result<PublicKey> {
  let procedure: procedures::PublicKey = procedures::PublicKey {
    ty: procedures::KeyType::Ed25519,
    private_key: location,
  };
  Ok(vault.execute(procedure).await.map(|public| public.to_vec().into())?)
}

async fn sign_ed25519(vault: &Vault<'_>, payload: Vec<u8>, location: Location) -> Result<Signature> {
  let public_key: PublicKey = retrieve_ed25519(vault, location.clone()).await?;
  let procedure: procedures::Ed25519Sign = procedures::Ed25519Sign {
    private_key: location,
    msg: payload,
  };
  let signature: [u8; 64] = vault.execute(procedure).await?;

  Ok(Signature::new(public_key, signature.into()))
}

// Moves a key from one location to another, thus deleting the old one.
async fn move_key(vault: &Vault<'_>, from: Location, to: Location) -> Result<()> {
  let copy_record = procedures::CopyRecord {
    source: from.clone(),
    target: to,
    hint: default_hint(),
  };

  vault.execute(copy_record).await?;

  let revoke_data = procedures::RevokeData {
    location: from,
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

fn location_chain_state() -> StoreKey {
  "$chain_state".to_owned()
}

// TODO: Turn static.
fn location_document() -> StoreKey {
  "$document".to_owned()
}

fn fmt_account_id(account_id: &AccountId) -> String {
  format!("$identity:{}", account_id)
}

impl From<&KeyLocation> for Location {
  fn from(key_location: &KeyLocation) -> Self {
    let bytes: Vec<u8> = format!("{}:{}", key_location.fragment, key_location.key_hash).into_bytes();
    Location::generic("$vault".as_bytes().to_vec(), bytes)
  }
}
