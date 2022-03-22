// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;

use async_trait::async_trait;

use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use uuid::Uuid;

use crate::error::Result;
use crate::identity::ChainState;
use crate::types::KeyLocation;
use crate::types::Signature;

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

/// An interface for Account storage implementations.
///
/// The [`Storage`] interface is used for secure operations on keys, such as generation and signing,
/// as well as key-value like storage of data structures, such as DID documents.
///
/// This interface works with [`AccountId`] and [`IotaDID`] as the top-level identifiers.
/// `AccountId` is intended as a partition-key, i.e. everything related to the account
/// can be stored in its own partition. Keys are identified by [`KeyLocation`]s in that partition.
/// An `IotaDID` can be resolved to an `AccountId` using an index, which is a global data structure.
/// Therefore, index operations need to be carefully synchronized per [`Storage`] instance, while other operations
/// only need to be synchronized within the same `AccountId`.
/// For example, two `key_generate` operations with different account ids can be executed concurrently.
///
/// See [`MemStore`][crate::storage::MemStore] for a test/example implementation.
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait Storage: storage_sub_trait::StorageSendSyncMaybe + Debug {
  /// Creates a new keypair for the given `account_id` at the given `location`.
  ///
  /// If a key at `location` exists, it is overwritten.
  async fn key_generate(&self, account_id: &AccountId, location: &KeyLocation) -> Result<()>;

  /// Inserts a private key at the specified `location`.
  ///
  /// If a key at `location` exists, it is overwritten.
  async fn key_insert(&self, account_id: &AccountId, location: &KeyLocation, private_key: PrivateKey) -> Result<()>;

  /// Moves a key from `source` to `target`.
  ///
  /// The key at the source location is removed. If a key at the target exists, it is overwritten.
  async fn key_move(&self, account_id: &AccountId, source: &KeyLocation, target: &KeyLocation) -> Result<()>;

  /// Retrieves the public key from `location`.
  async fn key_public(&self, account_id: &AccountId, location: &KeyLocation) -> Result<PublicKey>;

  /// Deletes the key at `location`.
  ///
  /// This operation is idempotent: it does not fail if the key does not exist.
  async fn key_del(&self, account_id: &AccountId, location: &KeyLocation) -> Result<()>;

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, account_id: &AccountId, location: &KeyLocation, data: Vec<u8>) -> Result<Signature>;

  /// Returns `true` if a key exists at the specified `location`.
  async fn key_exists(&self, account_id: &AccountId, location: &KeyLocation) -> Result<bool>;

  /// Returns the chain state of the identity specified by `did`.
  async fn chain_state(&self, account_id: &AccountId) -> Result<Option<ChainState>>;

  /// Set the chain state of the identity specified by `did`.
  async fn set_chain_state(&self, account_id: &AccountId, chain_state: &ChainState) -> Result<()>;

  /// Returns the [`IotaDocument`] of the identity specified by `did`.
  async fn document(&self, account_id: &AccountId) -> Result<Option<IotaDocument>>;

  /// Sets a new state for the identity specified by `did`.
  async fn set_document(&self, account_id: &AccountId, state: &IotaDocument) -> Result<()>;

  /// Removes the keys and any state for the identity specified by `did`.
  ///
  /// This operation is idempotent: it does not fail if the given `did` does not exist.
  async fn purge(&self, did: &IotaDID) -> Result<()>;

  /// Adds the did -> account_id mapping to the index.
  ///
  /// Note that his operation needs to be synchronized globally for a given storage instance,
  /// in order to prevent race conditions when inserting index entries concurrently.
  async fn index_set(&self, did: IotaDID, account_id: AccountId) -> Result<()>;

  // Looks up the [`AccountId`] of the given `did`.
  async fn index_get(&self, did: &IotaDID) -> Result<Option<AccountId>>;

  // Retrieve the list of DIDs stored in the index.
  async fn index(&self) -> Result<Vec<IotaDID>>;

  /// Write any unsaved changes to disk.
  async fn flush_changes(&self) -> Result<()>;
}
