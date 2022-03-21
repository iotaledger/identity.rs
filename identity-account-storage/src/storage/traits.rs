// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;

use async_trait::async_trait;

use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use uuid::Uuid;

use crate::error::Result;
use crate::identity::ChainState;
use crate::types::KeyLocation;
use crate::types::Signature;
use crate::utils::EncryptionKey;

pub type StoreKey = String;
pub type AccountId = Uuid;

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
  async fn key_generate(&self, did: &IotaDID, fragment: &str, key_type: KeyType) -> Result<KeyLocation>;

  /// Inserts a private key at the specified `location`.
  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<()>;

  /// Moves a key from one did-location pair to another.
  ///
  /// The key at the source location will be removed. If a key at the target exists, it will be overwritten.
  async fn key_move(&self, did: &IotaDID, source: &KeyLocation, target: &KeyLocation) -> Result<()>;

  /// Retrieves the public key at the specified `location`.
  async fn key_public(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey>;

  /// Deletes the keypair specified by `location`.
  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> Result<()>;

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature>;

  /// Returns `true` if a keypair exists at the specified `location`.
  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool>;

  /// Returns the chain state of the identity specified by `did`.
  async fn chain_state(&self, did: &IotaDID) -> Result<Option<ChainState>>;

  /// Set the chain state of the identity specified by `did`.
  async fn set_chain_state(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()>;

  /// Returns the [`IotaDocument`] of the identity specified by `did`.
  async fn document(&self, did: &IotaDID) -> Result<Option<IotaDocument>>;

  /// Sets a new state for the identity specified by `did`.
  async fn set_document(&self, did: &IotaDID, state: &IotaDocument) -> Result<()>;

  /// Removes the keys and any state for the identity specified by `did`.
  async fn purge(&self, did: &IotaDID) -> Result<()>;

  /// Adds the did -> account_id mapping to the index.
  ///
  /// Note that his operation needs to be synchronized globally for a given storage instance,
  /// in order to prevent race conditions when inserting index entries concurrently.
  async fn index_set(&self, did: IotaDID, account_id: AccountId) -> Result<()>;

  // Looks up the [`AccountId`] of the given `did`.
  async fn index_get(&self, did: &IotaDID) -> Result<Option<AccountId>>;
}
