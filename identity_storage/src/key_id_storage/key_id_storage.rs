// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_storage::KeyId;
use async_trait::async_trait;

use super::key_id_storage_error::KeyIdStorageError;
use super::method_digest::MethodDigest;

/// Result of key id storage operations.
pub type KeyIdStorageResult<T> = Result<T, KeyIdStorageError>;

/// Key value Storage for [`KeyId`] under [`MethodDigest`].
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait KeyIdStorage: storage_sub_trait::StorageSendSyncMaybe {
  /// Insert a [`KeyId`] into the [`KeyIdStorage`] under the given [`MethodDigest`].
  ///
  /// If an entry for `key` already exists in the storage an error must be returned
  /// immediately without altering the state of the storage.
  async fn insert_key_id(&self, method_digest: MethodDigest, key_id: KeyId) -> KeyIdStorageResult<()>;

  /// Obtain the [`KeyId`] associated with the given [`MethodDigest`].
  async fn get_key_id(&self, method_digest: &MethodDigest) -> KeyIdStorageResult<KeyId>;

  /// Delete the [`KeyId`] associated with the given [`MethodDigest`] from the [`KeyIdStorage`].
  ///
  /// If `key` is not found in storage, an Error must be returned.
  async fn delete_key_id(&self, method_digest: &MethodDigest) -> KeyIdStorageResult<()>;
}

#[cfg(not(feature = "send-sync-storage"))]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe {}
  impl<S: super::KeyIdStorage> StorageSendSyncMaybe for S {}
}

#[cfg(feature = "send-sync-storage")]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe: Send + Sync {}
  impl<S: Send + Sync + super::KeyIdStorage> StorageSendSyncMaybe for S {}
}
