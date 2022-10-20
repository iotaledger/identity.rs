// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::{BlobStorage, StorageResult};

pub struct MemBlobStore {}

impl MemBlobStore {
  pub fn new() -> Self {
    Self {}
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl BlobStorage for MemBlobStore {
  async fn store(&self, key: &str, blob: Option<Vec<u8>>) -> StorageResult<()> {
    todo!()
  }

  async fn load(&self, key: &str) -> StorageResult<Option<Vec<u8>>> {
    todo!()
  }

  async fn flush(&self) -> StorageResult<()> {
    todo!()
  }
}
