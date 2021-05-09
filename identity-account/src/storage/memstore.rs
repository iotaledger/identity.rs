// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use futures::stream;
use futures::stream::BoxStream;
use futures::StreamExt;
use hashbrown::HashMap;
use identity_core::crypto::Ed25519;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use identity_core::crypto::Sign;
use identity_did::verification::MethodType;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use crate::error::Error;
use crate::error::Result;
use crate::events::Commit;
use crate::identity::IdentityId;
use crate::identity::IdentityIndex;
use crate::identity::IdentitySnapshot;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::EncryptionKey;
use crate::utils::Shared;

type MemVault = HashMap<KeyLocation, KeyPair>;

type Events = HashMap<IdentityId, Vec<Commit>>;
type States = HashMap<IdentityId, IdentitySnapshot>;
type Vaults = HashMap<IdentityId, MemVault>;

pub struct MemStore {
  expand: bool,
  index: Shared<IdentityIndex>,
  events: Shared<Events>,
  states: Shared<States>,
  vaults: Shared<Vaults>,
}

impl MemStore {
  pub fn new() -> Self {
    Self {
      expand: false,
      index: Shared::new(IdentityIndex::new()),
      events: Shared::new(HashMap::new()),
      states: Shared::new(HashMap::new()),
      vaults: Shared::new(HashMap::new()),
    }
  }

  pub fn expand(&self) -> bool {
    self.expand
  }

  pub fn set_expand(&mut self, value: bool) {
    self.expand = value;
  }

  pub fn index(&self) -> Result<IdentityIndex> {
    self.index.read().map(|data| data.clone())
  }

  pub fn events(&self) -> Result<Events> {
    self.events.read().map(|data| data.clone())
  }

  pub fn states(&self) -> Result<States> {
    self.states.read().map(|data| data.clone())
  }

  pub fn vaults(&self) -> Result<Vaults> {
    self.vaults.read().map(|data| data.clone())
  }
}

#[async_trait::async_trait]
impl Storage for MemStore {
  async fn set_password(&self, _password: EncryptionKey) -> Result<()> {
    Ok(())
  }

  async fn flush_changes(&self) -> Result<()> {
    Ok(())
  }

  async fn key_new(&self, id: IdentityId, location: &KeyLocation) -> Result<PublicKey> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.entry(id).or_default();

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => {
        let keypair: KeyPair = KeyPair::new_ed25519()?;
        let public: PublicKey = keypair.public().clone();

        vault.insert(location.clone(), keypair);

        Ok(public)
      }
      MethodType::MerkleKeyCollection2021 => {
        todo!("[MemStore::key_new] Handle MerkleKeyCollection2021")
      }
    }
  }

  async fn key_exists(&self, id: IdentityId, location: &KeyLocation) -> Result<bool> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;

    if let Some(vault) = vaults.get(&id) {
      return Ok(vault.contains_key(location));
    }

    Ok(false)
  }

  async fn key_get(&self, id: IdentityId, location: &KeyLocation) -> Result<PublicKey> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;
    let vault: &MemVault = vaults.get(&id).ok_or(Error::KeyVaultNotFound)?;
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyPairNotFound)?;

    Ok(keypair.public().clone())
  }

  async fn key_del(&self, id: IdentityId, location: &KeyLocation) -> Result<()> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.get_mut(&id).ok_or(Error::KeyVaultNotFound)?;

    vault.remove(location);

    Ok(())
  }

  async fn key_sign(&self, id: IdentityId, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;
    let vault: &MemVault = vaults.get(&id).ok_or(Error::KeyVaultNotFound)?;
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyPairNotFound)?;

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => {
        assert_eq!(keypair.type_(), KeyType::Ed25519);

        let public: PublicKey = keypair.public().clone();
        let signature: [u8; 64] = Ed25519::sign(&data, keypair.secret())?;
        let signature: Signature = Signature::new(public, signature.to_vec());

        Ok(signature)
      }
      MethodType::MerkleKeyCollection2021 => {
        todo!("[MemStore::key_sign] Handle MerkleKeyCollection2021")
      }
    }
  }

  async fn index(&self) -> Result<IdentityIndex> {
    self.index.read().map(|index| index.clone())
  }

  async fn set_index(&self, index: &IdentityIndex) -> Result<()> {
    *self.index.write()? = index.clone();

    Ok(())
  }

  async fn snapshot(&self, id: IdentityId) -> Result<Option<IdentitySnapshot>> {
    self.states.read().map(|states| states.get(&id).cloned())
  }

  async fn set_snapshot(&self, id: IdentityId, snapshot: &IdentitySnapshot) -> Result<()> {
    self.states.write()?.insert(id, snapshot.clone());

    Ok(())
  }

  async fn append(&self, id: IdentityId, commits: &[Commit]) -> Result<()> {
    let mut state: RwLockWriteGuard<'_, _> = self.events.write()?;
    let queue: &mut Vec<Commit> = state.entry(id).or_default();

    for commit in commits {
      queue.push(commit.clone());
    }

    Ok(())
  }

  async fn stream(&self, id: IdentityId, index: Generation) -> Result<BoxStream<'_, Result<Commit>>> {
    let state: RwLockReadGuard<'_, _> = self.events.read()?;
    let queue: Vec<Commit> = state.get(&id).cloned().unwrap_or_default();
    let index: usize = index.to_u32() as usize;

    Ok(stream::iter(queue.into_iter().skip(index)).map(Ok).boxed())
  }

  async fn purge(&self, id: IdentityId) -> Result<()> {
    let _ = self.events.write()?.remove(&id);
    let _ = self.states.write()?.remove(&id);
    let _ = self.vaults.write()?.remove(&id);

    Ok(())
  }
}

impl Debug for MemStore {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if self.expand {
      f.debug_struct("MemStore")
        .field("index", &self.index)
        .field("events", &self.events)
        .field("states", &self.states)
        .field("vaults", &self.vaults)
        .finish()
    } else {
      f.write_str("MemStore")
    }
  }
}

impl Default for MemStore {
  fn default() -> Self {
    Self::new()
  }
}
