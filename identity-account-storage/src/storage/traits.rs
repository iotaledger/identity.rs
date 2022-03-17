// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;

use async_trait::async_trait;

use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_did::verification::MethodType;
use identity_iota_core::did::IotaDID;

use crate::error::Result;
use crate::identity::ChainState;
use crate::identity::IdentityState;
use crate::types::KeyLocation;
use crate::types::KeyLocation2;
use crate::types::Signature;
use crate::utils::EncryptionKey;

#[cfg(not(feature = "send-sync-storage"))]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe {}
  impl<S: super::Storage> StorageSendSyncMaybe for S {}
}

#[cfg(feature = "send-sync-storage")]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe: Send + Sync {}
  impl<S: Send + Sync + super::Storage> StorageSendSyncMaybe for S {}
}

/// An interface for Identity Account storage implementations.
///
/// See [MemStore][crate::storage::MemStore] for a test/example implementation.
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait Storage: storage_sub_trait::StorageSendSyncMaybe + Debug {
  /// Sets the account password.
  async fn set_password(&self, password: EncryptionKey) -> Result<()>;

  /// Write any unsaved changes to disk.
  async fn flush_changes(&self) -> Result<()>;

  /// Creates a new keypair for the given `did` and returns its location.
  async fn key_new(&self, did: &IotaDID, fragment: &str, method_type: MethodType) -> Result<KeyLocation2>;

  /// Inserts a private key at the specified `location`.
  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation2, private_key: PrivateKey) -> Result<()>;

  /// Moves a key from one did-location pair to another.
  ///
  /// The key at the source location will be removed. If a key at the target exists, it will be overwritten.
  async fn key_move(&self, did: &IotaDID, source: &KeyLocation2, target: &KeyLocation2) -> Result<()>;

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&self, did: &IotaDID, location: &KeyLocation2) -> Result<PublicKey>;

  /// Deletes the keypair specified by `location`.
  async fn key_del(&self, did: &IotaDID, location: &KeyLocation2) -> Result<()>;

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation2, data: Vec<u8>) -> Result<Signature>;

  /// Returns `true` if a keypair exists at the specified `location`.
  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation2) -> Result<bool>;

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
