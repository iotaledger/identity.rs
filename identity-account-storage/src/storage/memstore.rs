// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use std::collections::HashSet;

use async_trait::async_trait;
use hashbrown::HashMap;
use identity_core::crypto::Ed25519;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_core::crypto::Sign;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::tangle::NetworkName;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use zeroize::Zeroize;

use crate::error::Error;
use crate::error::Result;
use crate::identity::ChainState;
use crate::storage::Storage;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::Shared;

type MemVault = HashMap<KeyLocation, KeyPair>;

type ChainStates = HashMap<IotaDID, ChainState>;
type States = HashMap<IotaDID, IotaDocument>;
type Vaults = HashMap<IotaDID, MemVault>;
type Index = HashSet<IotaDID>;

pub struct MemStore {
  expand: bool,
  chain_states: Shared<ChainStates>,
  documents: Shared<States>,
  vaults: Shared<Vaults>,
  index: Shared<Index>,
}

impl MemStore {
  pub fn new() -> Self {
    Self {
      expand: false,
      chain_states: Shared::new(HashMap::new()),
      documents: Shared::new(HashMap::new()),
      vaults: Shared::new(HashMap::new()),
      index: Shared::new(HashSet::new()),
    }
  }

  pub fn expand(&self) -> bool {
    self.expand
  }

  pub fn set_expand(&mut self, value: bool) {
    self.expand = value;
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl Storage for MemStore {
  async fn did_create(
    &self,
    network: NetworkName,
    fragment: &str,
    private_key: Option<PrivateKey>,
  ) -> Result<(IotaDID, KeyLocation)> {
    let keypair: KeyPair = match private_key {
      Some(private_key) => KeyPair::try_from_private_key_bytes(KeyType::Ed25519, private_key.as_ref())?,
      None => KeyPair::new(KeyType::Ed25519)?,
    };

    let location: KeyLocation = KeyLocation::new(KeyType::Ed25519, fragment.to_owned(), keypair.public().as_ref());

    let did: IotaDID = IotaDID::new_with_network(keypair.public().as_ref(), network)
      .map_err(|err| crate::Error::DIDCreationError(err.to_string()))?;

    let mut index: RwLockWriteGuard<'_, _> = self.index.write()?;

    if index.contains(&did) {
      return Err(Error::IdentityAlreadyExists);
    } else {
      index.insert(did.clone());
    }

    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;

    let vault: &mut MemVault = vaults.entry(did.clone()).or_default();

    vault.insert(location.clone(), keypair);

    Ok((did, location))
  }

  async fn did_purge(&self, did: &IotaDID) -> Result<bool> {
    if self.index.write()?.remove(did) {
      let _ = self.documents.write()?.remove(did);
      let _ = self.vaults.write()?.remove(did);
      let _ = self.chain_states.write()?.remove(did);

      Ok(true)
    } else {
      Ok(false)
    }
  }

  async fn did_exists(&self, did: &IotaDID) -> Result<bool> {
    Ok(self.index.read()?.contains(did))
  }

  async fn did_list(&self) -> Result<Vec<IotaDID>> {
    Ok(self.index.read()?.iter().cloned().collect())
  }

  async fn key_generate(&self, did: &IotaDID, key_type: KeyType, fragment: &str) -> Result<KeyLocation> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.entry(did.clone()).or_default();

    let keypair: KeyPair = KeyPair::new(key_type)?;

    let location: KeyLocation = KeyLocation::new(key_type, fragment.to_owned(), keypair.public().as_ref());

    vault.insert(location.clone(), keypair);

    Ok(location)
  }

  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, mut private_key: PrivateKey) -> Result<()> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.entry(did.clone()).or_default();

    match location.key_type {
      KeyType::Ed25519 => {
        let keypair: KeyPair = KeyPair::try_from_private_key_bytes(KeyType::Ed25519, private_key.as_ref())
          .map_err(|err| Error::InvalidPrivateKey(err.to_string()))?;
        private_key.zeroize();

        vault.insert(location.to_owned(), keypair);

        Ok(())
      }
      KeyType::X25519 => {
        let keypair: KeyPair = KeyPair::try_from_private_key_bytes(KeyType::X25519, private_key.as_ref())
          .map_err(|err| Error::InvalidPrivateKey(err.to_string()))?;
        private_key.zeroize();

        vault.insert(location.to_owned(), keypair);

        Ok(())
      }
    }
  }

  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;

    if let Some(vault) = vaults.get(did) {
      return Ok(vault.contains_key(location));
    }

    Ok(false)
  }

  async fn key_public(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;
    let vault: &MemVault = vaults.get(did).ok_or(Error::KeyVaultNotFound)?;
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyNotFound)?;

    Ok(keypair.public().clone())
  }

  async fn key_delete(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.get_mut(did).ok_or(Error::KeyVaultNotFound)?;

    Ok(vault.remove(location).is_some())
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;
    let vault: &MemVault = vaults.get(did).ok_or(Error::KeyVaultNotFound)?;
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyNotFound)?;

    match location.key_type {
      KeyType::Ed25519 => {
        assert_eq!(keypair.type_(), KeyType::Ed25519);

        let signature: [u8; 64] = Ed25519::sign(&data, keypair.private())?;
        let signature: Signature = Signature::new(signature.to_vec());
        Ok(signature)
      }
      KeyType::X25519 => {
        return Err(identity_did::Error::InvalidMethodType.into());
      }
    }
  }

  async fn chain_state_get(&self, did: &IotaDID) -> Result<Option<ChainState>> {
    self.chain_states.read().map(|states| states.get(did).cloned())
  }

  async fn chain_state_set(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()> {
    self.chain_states.write()?.insert(did.clone(), chain_state.clone());

    Ok(())
  }

  async fn document_get(&self, did: &IotaDID) -> Result<Option<IotaDocument>> {
    self.documents.read().map(|documents| documents.get(did).cloned())
  }

  async fn document_set(&self, did: &IotaDID, document: &IotaDocument) -> Result<()> {
    self.documents.write()?.insert(did.clone(), document.clone());

    Ok(())
  }

  async fn flush_changes(&self) -> Result<()> {
    Ok(())
  }
}

impl Debug for MemStore {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    if self.expand {
      f.debug_struct("MemStore")
        .field("chain_states", &self.chain_states)
        .field("states", &self.documents)
        .field("vaults", &self.vaults)
        .field("index", &self.index)
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

#[cfg(test)]
mod tests {
  use crate::storage::Storage;
  use crate::storage::StorageTestSuite;

  use super::MemStore;

  fn test_memstore() -> impl Storage {
    MemStore::new()
  }

  #[tokio::test]
  async fn test_memstore_did_create_with_private_key() {
    StorageTestSuite::did_create_private_key_test(test_memstore())
      .await
      .unwrap()
  }

  #[tokio::test]
  async fn test_memstore_did_create_generate_key() {
    StorageTestSuite::did_create_generate_key_test(test_memstore())
      .await
      .unwrap()
  }

  #[tokio::test]
  async fn test_memstore_key_generate() {
    StorageTestSuite::key_generate_test(test_memstore()).await.unwrap()
  }

  #[tokio::test]
  async fn test_memstore_key_delete() {
    StorageTestSuite::key_delete_test(test_memstore()).await.unwrap()
  }

  #[tokio::test]
  async fn test_memstore_did_list() {
    StorageTestSuite::did_list_test(test_memstore()).await.unwrap()
  }

  #[tokio::test]
  async fn test_memstore_key_insert() {
    StorageTestSuite::key_insert_test(test_memstore()).await.unwrap()
  }

  #[tokio::test]
  async fn test_memstore_key_sign_ed25519() {
    StorageTestSuite::key_sign_ed25519_test(test_memstore()).await.unwrap()
  }

  #[tokio::test]
  async fn test_memstore_key_value_store() {
    StorageTestSuite::key_value_store_test(test_memstore()).await.unwrap()
  }

  #[tokio::test]
  async fn test_memstore_did_purge() {
    StorageTestSuite::did_purge_test(test_memstore()).await.unwrap()
  }
}
