// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDID;

use crate::error::Result;
use crate::identity::ChainState;
use crate::identity::IdentityState;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::EncryptionKey;

/// An interface for Identity Account storage implementations.
///
/// See [MemStore][crate::storage::MemStore] for a test/example implementation.
#[async_trait::async_trait]
pub trait Storage: Debug + Send + Sync + 'static {
  /// Sets the account password.
  async fn set_password(&self, password: EncryptionKey) -> Result<()>;

  /// Write any unsaved changes to disk.
  async fn flush_changes(&self) -> Result<()>;

  /// Creates a new keypair at the specified `location`
  async fn key_new(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey>;

  /// Inserts a private key at the specified `location`.
  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<PublicKey>;

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey>;

  /// Deletes the keypair specified by `location`.
  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> Result<()>;

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature>;

  /// Returns `true` if a keypair exists at the specified `location`.
  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool>;

  /// Returns the last generation that has been published to the tangle for the given `did`.
  async fn published_generation(&self, did: &IotaDID) -> Result<Option<Generation>>;

  /// Sets the last generation that has been published to the tangle for the given `did`.
  async fn set_published_generation(&self, did: &IotaDID, index: Generation) -> Result<()>;

  /// Returns the chain state of the identity specified by `did`.
  async fn chain_state(&self, did: &IotaDID) -> Result<Option<ChainState>>;

  /// Set the chain state of the identity specified by `did`.
  async fn set_chain_state(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()>;

  /// Returns the state of the identity specified by `did`.
  async fn state(&self, did: &IotaDID) -> Result<Option<IdentityState>>;

  /// Sets a new state for the identity specified by `did`.
  async fn set_state(&self, did: &IotaDID, state: &IdentityState) -> Result<()>;

  /// Removes the keys and any state for the identity specified by `did`.
  async fn purge(&self, did: &IotaDID) -> Result<()>;
}

#[async_trait::async_trait]
impl Storage for Box<dyn Storage> {
  async fn set_password(&self, password: EncryptionKey) -> Result<()> {
    (**self).set_password(password).await
  }

  async fn flush_changes(&self) -> Result<()> {
    (**self).flush_changes().await
  }

  async fn key_new(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    (**self).key_new(did, location).await
  }

  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<PublicKey> {
    (**self).key_insert(did, location, private_key).await
  }

  async fn key_get(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey> {
    (**self).key_get(did, location).await
  }

  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> Result<()> {
    (**self).key_del(did, location).await
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature> {
    (**self).key_sign(did, location, data).await
  }

  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool> {
    (**self).key_exists(did, location).await
  }

  async fn chain_state(&self, did: &IotaDID) -> Result<Option<ChainState>> {
    (**self).chain_state(did).await
  }

  async fn set_chain_state(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()> {
    (**self).set_chain_state(did, chain_state).await
  }

  async fn state(&self, did: &IotaDID) -> Result<Option<IdentityState>> {
    (**self).state(did).await
  }

  async fn set_state(&self, did: &IotaDID, state: &IdentityState) -> Result<()> {
    (**self).set_state(did, state).await
  }

  async fn purge(&self, did: &IotaDID) -> Result<()> {
    (**self).purge(did).await
  }

  async fn published_generation(&self, did: &IotaDID) -> Result<Option<Generation>> {
    (**self).published_generation(did).await
  }

  async fn set_published_generation(&self, did: &IotaDID, index: Generation) -> Result<()> {
    (**self).set_published_generation(did, index).await
  }
}
