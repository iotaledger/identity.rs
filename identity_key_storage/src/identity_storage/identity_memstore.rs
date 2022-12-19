// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use crate::identifiers::KeyId;
use crate::identifiers::MethodId;
use crate::key_storage::shared::Shared;

use super::IdentityStorage;
use super::IdentityStorageError;
use super::IdentityStorageErrorKind;
use super::IdentityStorageResult;

pub struct MemIdentityStore {
  key_ids: Shared<HashMap<MethodId, KeyId>>,
}

impl MemIdentityStore {
  pub fn new() -> Self {
    Self {
      key_ids: Shared::new(HashMap::new()),
    }
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl IdentityStorage for MemIdentityStore {
  async fn store_key_id(&self, idx: MethodId, key_id: KeyId) -> IdentityStorageResult<()> {
    let mut key_ids: RwLockWriteGuard<'_, _> = self.key_ids.write().await;

    match key_ids.entry(idx) {
      Entry::Occupied(_) => {
        return Err(IdentityStorageError::new(
          IdentityStorageErrorKind::MethodIdxAlreadyExists,
        ));
      }
      Entry::Vacant(entry) => entry.insert(key_id),
    };

    Ok(())
  }

  async fn load_key_id(&self, idx: &MethodId) -> IdentityStorageResult<KeyId> {
    let key_ids: RwLockReadGuard<'_, _> = self.key_ids.read().await;
    key_ids
      .get(idx)
      .map(ToOwned::to_owned)
      .ok_or_else(|| IdentityStorageError::new(IdentityStorageErrorKind::MethodIdxNotFound))
  }

  async fn delete_key_id(&self, method_idx: &MethodId) -> IdentityStorageResult<()> {
    let mut key_ids: RwLockWriteGuard<'_, _> = self.key_ids.write().await;
    key_ids.remove(method_idx);

    Ok(())
  }

  // async fn flush(&self) -> StorageResult<()> {
  //   Ok(())
  // }
}

impl Default for MemIdentityStore {
  fn default() -> Self {
    Self::new()
  }
}
