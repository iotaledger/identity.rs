// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use crate::shared::Shared;
use crate::BlobStorage;
use crate::StorageResult;

pub struct MemBlobStore {
  store: Shared<HashMap<String, Vec<u8>>>,
}

impl MemBlobStore {
  pub fn new() -> Self {
    Self {
      store: Shared::new(HashMap::new()),
    }
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl BlobStorage for MemBlobStore {
  async fn store(&self, key: &str, blob: Option<Vec<u8>>) -> StorageResult<()> {
    let mut store: RwLockWriteGuard<'_, _> = self.store.write().await;

    match blob {
      Some(bytes) => store.insert(key.to_owned(), bytes),
      None => store.remove(key),
    };

    Ok(())
  }

  async fn load(&self, key: &str) -> StorageResult<Option<Vec<u8>>> {
    let store: RwLockReadGuard<'_, _> = self.store.read().await;
    Ok(store.get(key).map(ToOwned::to_owned))
  }

  async fn flush(&self) -> StorageResult<()> {
    Ok(())
  }
}
