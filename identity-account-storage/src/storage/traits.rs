// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;

use async_trait::async_trait;

use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::tangle::NetworkName;

use crate::error::Result;
use crate::identity::ChainState;
use crate::types::KeyLocation;
use crate::types::Signature;

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
/// `AccountId` is intended as a partition-key, i.e. everything related to an account
/// can be stored in its own partition. Keys belonging to an account are identified by [`KeyLocation`]s
/// in that partition.
/// An `IotaDID` can be resolved to an `AccountId` using the index, which is a global data structure.
/// Therefore, index operations need to be carefully synchronized per [`Storage`] instance.
/// Other operations don't need to be synchronized globally, as it is a user error to create
/// more than one `Account` for the same identity.
///
/// See [`MemStore`][crate::storage::MemStore] for a test/example implementation.
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait Storage: storage_sub_trait::StorageSendSyncMaybe + Debug {
  /// Creates a new identity for the given `network`.
  ///
  /// - Uses the given Ed25519 `private_key` or generates a new key if it's `None`.
  /// - Returns an [`IdentityAlreadyExists`](crate::Error::IdentityAlreadyExists) error if the DID already exists.
  /// - Adds the newly created DID to an index which can be accessed via [`Storage::index`].
  ///
  /// Returns the generated DID and the location at which the key was stored.
  async fn did_create(
    &self,
    network: NetworkName,
    fragment: &str,
    private_key: Option<PrivateKey>,
  ) -> Result<(IotaDID, KeyLocation)>;

  /// Removes the keys and any other state for the given `did`.
  ///
  /// This operation is idempotent: it does not fail if the given `did` does not exist.
  ///
  /// Returns `true` if the did was removed, false if nothing was done.
  async fn did_purge(&self, did: &IotaDID) -> Result<bool>;

  /// Generates a new key for the given `account_id` with the given `key_type` and `fragment` identifier
  /// and returns the location of the newly generated key.
  async fn key_generate(&self, did: &IotaDID, key_type: KeyType, fragment: &str) -> Result<KeyLocation>;

  /// Inserts a private key at the specified `location`.
  ///
  /// If a key at `location` exists, it is overwritten.
  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result<()>;

  /// Retrieves the public key from `location`.
  async fn key_public(&self, did: &IotaDID, location: &KeyLocation) -> Result<PublicKey>;

  /// Deletes the key at `location`.
  ///
  /// This operation is idempotent: it does not fail if the key does not exist.
  /// Returns true if it removed the key, false if it did nothing.
  async fn key_delete(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool>;

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result<Signature>;

  /// Returns `true` if a key exists at the specified `location`.
  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result<bool>;

  /// Returns the chain state of the identity specified by `account_id`.
  async fn chain_state_get(&self, did: &IotaDID) -> Result<Option<ChainState>>;

  /// Set the chain state of the identity specified by `account_id`.
  async fn chain_state_set(&self, did: &IotaDID, chain_state: &ChainState) -> Result<()>;

  /// Returns the [`IotaDocument`] of the identity specified by `account_id`.
  async fn document_get(&self, did: &IotaDID) -> Result<Option<IotaDocument>>;

  /// Sets a new state for the identity specified by `account_id`.
  async fn document_set(&self, did: &IotaDID, state: &IotaDocument) -> Result<()>;

  /// Returns `true` if the index contains the given `did`.
  async fn index_has(&self, did: &IotaDID) -> Result<bool>;

  /// Retrieves the list of [`IotaDID`]s stored in the index.
  async fn index(&self) -> Result<Vec<IotaDID>>;

  /// Persists any unsaved changes.
  async fn flush_changes(&self) -> Result<()>;
}
