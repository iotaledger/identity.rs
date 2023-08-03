// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_id_storage::key_id_storage::KeyIdStorage;
use crate::key_id_storage::key_id_storage_error::KeyIdStorageError;
use crate::key_id_storage::key_id_storage_error::KeyIdStorageErrorKind;
use crate::key_storage::shared::Shared;
use crate::key_storage::KeyId;
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use super::key_id_storage::KeyIdStorageResult;
use super::method_digest::MethodDigest;

type KeyIdStore = HashMap<MethodDigest, KeyId>;

/// An insecure, in-memory [`KeyIdStorage`] implementation that serves as an example and may be used in tests.
#[derive(Debug)]
pub struct KeyIdMemstore {
  key_id_store: Shared<KeyIdStore>,
}

impl KeyIdMemstore {
  /// Creates a new, empty `KeyIdMemstore` instance.
  pub fn new() -> Self {
    Self {
      key_id_store: Shared::new(HashMap::new()),
    }
  }

  /// Returns the number of items contained in the [`KeyIdMemstore`].
  pub async fn count(&self) -> usize {
    self.key_id_store.read().await.keys().count()
  }
}

impl Default for KeyIdMemstore {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(? Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl KeyIdStorage for KeyIdMemstore {
  async fn insert_key_id(&self, key: MethodDigest, value: KeyId) -> KeyIdStorageResult<()> {
    let mut key_id_store: RwLockWriteGuard<'_, KeyIdStore> = self.key_id_store.write().await;
    if key_id_store.contains_key(&key) {
      return Err(KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdAlreadyExists));
    }
    key_id_store.insert(key, value);
    Ok(())
  }

  async fn get_key_id(&self, key: &MethodDigest) -> KeyIdStorageResult<KeyId> {
    let key_id_store: RwLockReadGuard<'_, KeyIdStore> = self.key_id_store.read().await;
    Ok(
      key_id_store
        .get(key)
        .ok_or_else(|| KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdNotFound))?
        .clone(),
    )
  }

  async fn delete_key_id(&self, key: &MethodDigest) -> KeyIdStorageResult<()> {
    let mut key_id_store: RwLockWriteGuard<'_, KeyIdStore> = self.key_id_store.write().await;
    key_id_store
      .remove(key)
      .ok_or_else(|| KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdNotFound))?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::key_id_storage::key_id_storage::KeyIdStorage;
  use crate::key_id_storage::memstore::KeyIdMemstore;
  use crate::key_id_storage::method_digest::MethodDigest;
  use crate::key_id_storage::KeyIdStorageError;
  use crate::key_id_storage::KeyIdStorageErrorKind;
  use crate::key_storage::KeyId;
  use identity_verification::VerificationMethod;

  #[tokio::test]
  async fn memstore_operations() {
    let verification_method: VerificationMethod = crate::storage::tests::test_utils::create_verification_method();

    // Test insertion.
    let memstore: KeyIdMemstore = KeyIdMemstore::new();
    let key_id_1 = KeyId::new("keyid");
    let method_digest: MethodDigest = MethodDigest::new(&verification_method).unwrap();
    memstore
      .insert_key_id(method_digest.clone(), key_id_1.clone())
      .await
      .expect("inserting into memstore failed");

    // Double insertion.
    let insertion_result = memstore.insert_key_id(method_digest.clone(), key_id_1.clone()).await;
    let _expected_error: KeyIdStorageError = KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdAlreadyExists);
    assert!(matches!(insertion_result.unwrap_err(), _expected_error));

    // Test retrieving.
    let key_id: KeyId = memstore.get_key_id(&method_digest).await.unwrap();
    assert_eq!(key_id_1, key_id);

    // Test deletion.
    memstore.delete_key_id(&method_digest).await.expect("deletion failed");

    let repeat_deletion_result: Result<(), KeyIdStorageError> = memstore.delete_key_id(&method_digest).await;
    let _expected_error: KeyIdStorageError = KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdNotFound);
    assert!(matches!(repeat_deletion_result.unwrap_err(), _expected_error));
  }
}
