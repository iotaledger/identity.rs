// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use async_trait::async_trait;
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
use identity_iota_core::document::IotaVerificationMethod;
use std::convert::TryFrom;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use zeroize::Zeroize;

use crate::error::Error;
use crate::error::Result;
use crate::identity::ChainState;
use crate::storage::Storage;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::EncryptionKey;
use crate::utils::Shared;

use super::AccountId;

type MemVault = HashMap<KeyLocation, KeyPair>;

type ChainStates = HashMap<IotaDID, ChainState>;
type States = HashMap<IotaDID, IotaDocument>;
type Vaults = HashMap<IotaDID, MemVault>;
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
  async fn set_password(&self, _password: EncryptionKey) -> Result<()> {
    Ok(())
  }

  async fn flush_changes(&self) -> Result<()> {
    Ok(())
  }

  async fn key_generate(&self, did: &IotaDID, fragment: &str, key_type: KeyType) -> Result<KeyLocation> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.entry(did.clone()).or_default();

    let keypair: KeyPair = KeyPair::new(key_type)?;
    let public: PublicKey = keypair.public().clone();

    let method = IotaVerificationMethod::new(did.clone(), KeyType::Ed25519, &public, fragment)?;

    let location = KeyLocation::new(key_type, fragment.to_owned(), method.key_data());

    vault.insert(location.clone(), keypair);

    Ok(location)
  }

  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<()> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.entry(did.clone()).or_default();

    match location.key_type {
      KeyType::Ed25519 => {
        let mut private_key_bytes: [u8; 32] = <[u8; 32]>::try_from(private_key.as_ref())
          .map_err(|err| Error::InvalidPrivateKey(format!("expected a slice of 32 bytes - {}", err)))?;

        let secret: ed25519::SecretKey = ed25519::SecretKey::from_bytes(private_key_bytes);
        private_key_bytes.zeroize();

        let public: ed25519::PublicKey = secret.public_key();

        let public_key: PublicKey = public.to_bytes().to_vec().into();

        let keypair: KeyPair = KeyPair::from((KeyType::Ed25519, public_key, private_key));

        vault.insert(location.clone(), keypair);

        Ok(())
      }
    }
  }

  async fn key_move(&self, did: &IotaDID, source: &KeyLocation, target: &KeyLocation) -> Result<()> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;

    if let Some(vault) = vaults.get_mut(did) {
      match vault.remove(source) {
        Some(key) => {
          vault.insert(target.clone(), key);
          Ok(())
        }
        None => Err(Error::KeyNotFound),
      }
    } else {
      Err(Error::KeyVaultNotFound)
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

  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> Result<()> {
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    let vault: &mut MemVault = vaults.get_mut(did).ok_or(Error::KeyVaultNotFound)?;

    vault.remove(location);

    Ok(())
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;
    let vault: &MemVault = vaults.get(did).ok_or(Error::KeyVaultNotFound)?;
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyNotFound)?;

    match location.key_type {
      KeyType::Ed25519 => {
        assert_eq!(keypair.type_(), KeyType::Ed25519);

        let public: PublicKey = keypair.public().clone();
        let signature: [u8; 64] = Ed25519::sign(&data, keypair.private())?;
        let signature: Signature = Signature::new(public, signature.to_vec());

        Ok(signature)
      }
    }
  }

  async fn chain_state(&self, did: &IotaDID) -> Result<Option<ChainState>> {
    self.chain_states.read().map(|states| states.get(did).cloned())
  }

  async fn set_chain_state(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()> {
    self.chain_states.write()?.insert(did.clone(), chain_state.clone());

    Ok(())
  }

  async fn document(&self, did: &IotaDID) -> Result<Option<IotaDocument>> {
    self.documents.read().map(|documents| documents.get(did).cloned())
  }

  async fn set_document(&self, did: &IotaDID, document: &IotaDocument) -> Result<()> {
    self.documents.write()?.insert(did.clone(), document.clone());

    Ok(())
  }

  async fn purge(&self, did: &IotaDID) -> Result<()> {
    let _ = self.documents.write()?.remove(did);
    let _ = self.vaults.write()?.remove(did);
    let _ = self.chain_states.write()?.remove(did);

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
}

impl Debug for MemStore {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    if self.expand {
      f.debug_struct("MemStore")
        .field("chain_states", &self.chain_states)
        .field("states", &self.documents)
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
