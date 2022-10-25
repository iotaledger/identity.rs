// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::StorageResult;

#[cfg(not(feature = "send-sync-storage"))]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe {}
  impl<S: super::BlobStorage> StorageSendSyncMaybe for S {}
}

#[cfg(feature = "send-sync-storage")]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe: Send + Sync {}
  impl<S: Send + Sync + super::BlobStorage> StorageSendSyncMaybe for S {}
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait BlobStorage: storage_sub_trait::StorageSendSyncMaybe {
  /// Stores an arbitrary blob for the `key`.
  ///
  /// Passing `None` removes all data associated with the specified `key`.
  async fn store(&self, key: &str, blob: Option<Vec<u8>>) -> StorageResult<()>;

  /// Returns the blob stored at `key`, or `None` if no blob is stored.
  async fn load(&self, key: &str) -> StorageResult<Option<Vec<u8>>>;

  /// Persists any unsaved changes. Called before dropping a [`BlobStorage`], if at all.
  async fn flush(&self) -> StorageResult<()>;
}
