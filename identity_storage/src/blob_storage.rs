// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_did::did::CoreDID;

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
  /// Stores an arbitrary blob for the identity specified by `did`.
  ///
  /// Passing `None` means removing all data associated with the specified `did`.
  async fn store(&self, did: &CoreDID, blob: Option<Vec<u8>>) -> StorageResult<()>;

  /// Returns the blob stored by the identity specified by `did`, or `None`
  /// if no blob is stored.
  async fn load(&self, did: &CoreDID) -> StorageResult<Option<Vec<u8>>>;

  /// Persists any unsaved changes. Called before dropping a [`BlobStorage`], if at all.
  async fn flush(&self) -> StorageResult<()>;
}
