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

// The map from DIDs to chain states.
type ChainStates = HashMap<IotaDID, ChainState>;
// The map from DIDs to DID documents.
type States = HashMap<IotaDID, IotaDocument>;
// The map from DIDs to vaults.
type Vaults = HashMap<IotaDID, MemVault>;
// The set of DIDs stored in this storage.
type Index = HashSet<IotaDID>;
// The map from key locations to key pairs, that lives within a DID partition.
type MemVault = HashMap<KeyLocation, KeyPair>;

/// An insecure, in-memory [`Storage`] implementation that serves as an example and is used in tests.
pub struct MemStore {
  // Controls whether to print the storages content when debugging.
  expand: bool,
  // The `Shared<T>` type is simply a light wrapper around `Rwlock<T>`.
  chain_states: Shared<ChainStates>,
  documents: Shared<States>,
  vaults: Shared<Vaults>,
  index: Shared<Index>,
}

impl MemStore {
  /// Creates a new, empty `MemStore` instance.
  pub fn new() -> Self {
    Self {
      expand: false,
      chain_states: Shared::new(HashMap::new()),
      documents: Shared::new(HashMap::new()),
      vaults: Shared::new(HashMap::new()),
      index: Shared::new(HashSet::new()),
    }
  }

  /// Returns whether to expand the debug representation.
  pub fn expand(&self) -> bool {
    self.expand
  }

  /// Sets whether to expand the debug representation.
  pub fn set_expand(&mut self, value: bool) {
    self.expand = value;
  }
}

// Refer to the `Storage` interface docs for high-level documentation of the individual methods.
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl Storage for MemStore {
  async fn did_create(
    &self,
    network: NetworkName,
    fragment: &str,
    private_key: Option<PrivateKey>,
  ) -> Result<(IotaDID, KeyLocation)> {
    // Extract a `KeyPair` from the passed private key or generate a new one.
    // For `did_create` we can assume the `KeyType` to be `Ed25519` because
    // that is the only currently available signature type.
    let keypair: KeyPair = match private_key {
      Some(private_key) => KeyPair::try_from_private_key_bytes(KeyType::Ed25519, private_key.as_ref())?,
      None => KeyPair::new(KeyType::Ed25519)?,
    };

    // We create the location at which the key pair will be stored.
    // Most notably, this uses the public key as an input.
    let location: KeyLocation = KeyLocation::new(KeyType::Ed25519, fragment.to_owned(), keypair.public().as_ref());

    // Next we use the public key to derive the initial DID.
    let did: IotaDID = IotaDID::new_with_network(keypair.public().as_ref(), network)
      .map_err(|err| crate::Error::DIDCreationError(err.to_string()))?;

    // Since storages can be called from different threads or tasks, we need to obtain
    // mutually exclusive access to the index, before writing.
    let mut index: RwLockWriteGuard<'_, _> = self.index.write()?;

    // If the DID already exists in our index, we need to return an error.
    // We don't want to overwrite an existing DID.
    // Otherwise we add it to the index.
    if index.contains(&did) {
      return Err(Error::IdentityAlreadyExists);
    } else {
      index.insert(did.clone());
    }

    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;

    // Obtain the exiting mem vault or create a new one.
    let vault: &mut MemVault = vaults.entry(did.clone()).or_default();

    // Insert the key pair at the previously created location.
    vault.insert(location.clone(), keypair);

    // Return did and location.
    Ok((did, location))
  }

  async fn did_purge(&self, did: &IotaDID) -> Result<bool> {
    // This method is supposed to be idempotent,
    // so we only need to do work if the DID still exists.
    // The return value signals whether the DID was actually removed.
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
    // Note that any failure to get access to the storage and do the actual existence check
    // should result in an error rather than returning `false`.
    Ok(self.index.read()?.contains(did))
  }

  async fn did_list(&self) -> Result<Vec<IotaDID>> {
    Ok(self.index.read()?.iter().cloned().collect())
  }

  async fn key_generate(&self, did: &IotaDID, key_type: KeyType, fragment: &str) -> Result<KeyLocation> {
    // Obtain exclusive access to the vaults.
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    // Get or insert the MemVault.
    let vault: &mut MemVault = vaults.entry(did.clone()).or_default();

    // Generate a new key pair for the given `key_type`.
    let keypair: KeyPair = KeyPair::new(key_type)?;

    // Derive the key location from the fragmnent and public key and set the `KeyType` of the location.
    let location: KeyLocation = KeyLocation::new(key_type, fragment.to_owned(), keypair.public().as_ref());

    vault.insert(location.clone(), keypair);

    Ok(location)
  }

  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, mut private_key: PrivateKey) -> Result<()> {
    // Obtain exclusive access to the vaults.
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    // Get or insert the MemVault.
    let vault: &mut MemVault = vaults.entry(did.clone()).or_default();

    // Reconstruct the key pair from the given private key by inspecting the location for its key type.
    // Then insert the key at the given location.
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
    // Obtain read access to the vaults.
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;

    // Within the DID partition of vaults, check for existence of the given location.
    if let Some(vault) = vaults.get(did) {
      return Ok(vault.contains_key(location));
    }

    Ok(false)
  }

  async fn key_public(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    // Obtain read access to the vaults.
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;
    // Lookup the vault for the given DID.
    let vault: &MemVault = vaults.get(did).ok_or(Error::KeyVaultNotFound)?;
    // Lookup the key pair within the vault.
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyNotFound)?;

    // Return the public key.
    Ok(keypair.public().clone())
  }

  async fn key_delete(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    // Obtain read access to the vaults.
    let mut vaults: RwLockWriteGuard<'_, _> = self.vaults.write()?;
    // Lookup the vault for the given DID.
    let vault: &mut MemVault = vaults.get_mut(did).ok_or(Error::KeyVaultNotFound)?;

    // Remove the key and return whether it existed.
    Ok(vault.remove(location).is_some())
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    // Obtain read access to the vaults.
    let vaults: RwLockReadGuard<'_, _> = self.vaults.read()?;
    // Lookup the vault for the given DID.
    let vault: &MemVault = vaults.get(did).ok_or(Error::KeyVaultNotFound)?;
    // Lookup the key pair within the vault.
    let keypair: &KeyPair = vault.get(location).ok_or(Error::KeyNotFound)?;

    match location.key_type {
      KeyType::Ed25519 => {
        assert_eq!(keypair.type_(), KeyType::Ed25519);

        // Use the `Ed25519` API to sign the given data with the private key.
        let signature: [u8; 64] = Ed25519::sign(&data, keypair.private())?;
        // Construct a new `Signature` wrapper with the returned signature bytes.
        let signature: Signature = Signature::new(signature.to_vec());
        Ok(signature)
      }
      KeyType::X25519 => {
        // Calling key_sign on key types that cannot be signed with should return an error.
        return Err(identity_did::Error::InvalidMethodType.into());
      }
    }
  }

  async fn chain_state_get(&self, did: &IotaDID) -> Result<Option<ChainState>> {
    // Lookup the chain state of the given DID.
    self.chain_states.read().map(|states| states.get(did).cloned())
  }

  async fn chain_state_set(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()> {
    // Set the chain state of the given DID.
    self.chain_states.write()?.insert(did.clone(), chain_state.clone());

    Ok(())
  }

  async fn document_get(&self, did: &IotaDID) -> Result<Option<IotaDocument>> {
    // Lookup the DID document of the given DID.
    self.documents.read().map(|documents| documents.get(did).cloned())
  }

  async fn document_set(&self, did: &IotaDID, document: &IotaDocument) -> Result<()> {
    // Set the DID document of the given DID.
    self.documents.write()?.insert(did.clone(), document.clone());

    Ok(())
  }

  async fn flush_changes(&self) -> Result<()> {
    // The MemStore doesn't need to flush changes to disk or any other persistent store,
    // which is why this function does nothing.
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
