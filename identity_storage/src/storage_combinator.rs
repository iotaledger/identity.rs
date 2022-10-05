// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::BlobStorage;
use crate::KeyStorage;
// use async_trait::async_trait;
// use identity_did::did::CoreDID;

/// Combinator for two storage implementations.
#[allow(dead_code)]
pub struct StorageCombinator<K: KeyStorage, B: BlobStorage> {
  key_storage: K,
  blob_storage: B,
}

impl<K: KeyStorage, B: BlobStorage> StorageCombinator<K, B> {
  /// Combines a key and blob storage implementation to a full [`Storage`] implementation.
  pub fn new(key_storage: K, blob_storage: B) -> Self {
    Self {
      key_storage,
      blob_storage,
    }
  }
}

// #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
// #[cfg_attr(feature = "send-sync-storage", async_trait)]
// impl<K: KeyStorage, B: BlobStorage> BlobStorage for StorageCombinator<K, B> {
//   async fn store(&self, did: &CoreDID, blob: Option<Vec<u8>>) -> StorageResult<()> {
//     self.blob_storage.store(did, blob).await
//   }

//   async fn load(&self, did: &CoreDID) -> StorageResult<Option<Vec<u8>>> {
//     self.blob_storage.load(did).await
//   }

//   async fn flush(&self) -> StorageResult<()> {
//     self.blob_storage.flush().await
//   }
// }

// Same for KeyStorage.
// #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
// #[cfg_attr(feature = "send-sync-storage", async_trait)]
// impl<K: KeyStorage, B: BlobStorage> KeyStorage for StorageCombinator<K, B> { ... }

// impl<K: KeyStorage, B: BlobStorage> Storage for StorageCombinator<K, B> { ... }
