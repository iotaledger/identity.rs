// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use async_trait::async_trait;
use crypto::keys::x25519;
use crypto::signatures::ed25519;
use hashbrown::HashMap;
use identity_core::crypto::Ed25519;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_core::crypto::Sign;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use std::convert::TryFrom;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use zeroize::Zeroize;

use crate::error::Error;
use crate::error::Result;
use crate::identity::ChainState;
use crate::storage::Storage;
use crate::types::AccountId;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::Shared;

type MemVault = HashMap<KeyLocation, KeyPair>;

type ChainStates = HashMap<AccountId, ChainState>;
type States = HashMap<AccountId, IotaDocument>;
type Vaults = HashMap<AccountId, MemVault>;
type Index = HashMap<IotaDID, AccountId>;

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
      index: Shared::new(HashMap::new()),
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
  async fn key_generate(&self, account_id: &AccountId, key_type: KeyType, fragment: &str) -> Result<KeyLocation> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.entry(*account_id).or_default();

    let keypair: KeyPair = KeyPair::new(key_type)?;

    let location: KeyLocation = KeyLocation::new(key_type, fragment.to_owned(), keypair.public().as_ref());

    vault.insert(location.clone(), keypair);

    Ok(location)
  }

  async fn key_insert(&self, account_id: &AccountId, location: &KeyLocation, private_key: PrivateKey) -> Result<()> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.entry(*account_id).or_default();

    match location.key_type {
      KeyType::Ed25519 => {
        let mut private_key_bytes: [u8; 32] = <[u8; 32]>::try_from(private_key.as_ref())
          .map_err(|err| Error::InvalidPrivateKey(format!("expected a slice of 32 bytes - {}", err)))?;
        let secret: ed25519::SecretKey = ed25519::SecretKey::from_bytes(private_key_bytes);
        private_key_bytes.zeroize();

        let public: ed25519::PublicKey = secret.public_key();
        let public_key: PublicKey = public.to_bytes().to_vec().into();

        let keypair: KeyPair = KeyPair::from((KeyType::Ed25519, public_key, private_key));

        vault.insert(location.to_owned(), keypair);

        Ok(())
      }
      KeyType::X25519 => {
        let mut private_key_bytes: [u8; 32] = <[u8; 32]>::try_from(private_key.as_ref())
          .map_err(|err| Error::InvalidPrivateKey(format!("expected a slice of 32 bytes - {}", err)))?;
        let secret: x25519::SecretKey = x25519::SecretKey::from_bytes(private_key_bytes);
        private_key_bytes.zeroize();

        let public: x25519::PublicKey = secret.public_key();
        let public_key: PublicKey = public.to_bytes().to_vec().into();

        let keypair: KeyPair = KeyPair::from((KeyType::X25519, public_key, private_key));
        vault.insert(location.to_owned(), keypair);

        Ok(())
      }
    }
  }

  async fn key_exists(&self, account_id: &AccountId, location: &KeyLocation) -> Result<bool> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;

    if let Some(vault) = vaults.get(account_id) {
      return Ok(vault.contains_key(location));
    }

    Ok(false)
  }

  async fn key_public(&self, account_id: &AccountId, location: &KeyLocation) -> Result<PublicKey> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;
    let vault: &MemVault = vaults.get(account_id).ok_or(Error::KeyVaultNotFound)?;
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyNotFound)?;

    Ok(keypair.public().clone())
  }

  async fn key_del(&self, account_id: &AccountId, location: &KeyLocation) -> Result<()> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.get_mut(account_id).ok_or(Error::KeyVaultNotFound)?;

    vault.remove(location);

    Ok(())
  }

  async fn key_sign(&self, account_id: &AccountId, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;
    let vault: &MemVault = vaults.get(account_id).ok_or(Error::KeyVaultNotFound)?;
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

  async fn chain_state_get(&self, account_id: &AccountId) -> Result<Option<ChainState>> {
    self.chain_states.read().map(|states| states.get(account_id).cloned())
  }

  async fn chain_state_set(&self, account_id: &AccountId, chain_state: &ChainState) -> Result<()> {
    self.chain_states.write()?.insert(*account_id, chain_state.clone());

    Ok(())
  }

  async fn document_get(&self, account_id: &AccountId) -> Result<Option<IotaDocument>> {
    self
      .documents
      .read()
      .map(|documents| documents.get(account_id).cloned())
  }

  async fn document_set(&self, account_id: &AccountId, document: &IotaDocument) -> Result<()> {
    self.documents.write()?.insert(*account_id, document.clone());

    Ok(())
  }

  async fn purge(&self, did: &IotaDID) -> Result<()> {
    if let Some(account_id) = self.index_get(did).await? {
      let _ = self.documents.write()?.remove(&account_id);
      let _ = self.vaults.write()?.remove(&account_id);
      let _ = self.chain_states.write()?.remove(&account_id);

      self.index.write()?.remove(did);
    }

    Ok(())
  }

  async fn index_set(&self, did: IotaDID, account_id: AccountId) -> Result<()> {
    let mut index = self.index.write()?;

    index.insert(did, account_id);

    Ok(())
  }

  async fn index_get(&self, did: &IotaDID) -> Result<Option<AccountId>> {
    let index = self.index.read()?;

    Ok(index.get(did).cloned())
  }

  async fn index_keys(&self) -> Result<Vec<IotaDID>> {
    Ok(self.index.read()?.keys().cloned().collect())
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
