// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::crypto::PublicKey;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

use crate::error::Result;
use crate::storage::VaultAdapter;
use crate::types::KeyLocation;
use crate::types::ResourceId;
use crate::types::ResourceType;
use crate::types::Signature;
use crate::utils::EncryptionKey;

/// A thread-safe wrapper around a [`VaultAdapter`] implementation.
#[derive(Clone)]
pub struct StorageHandle {
  data: Arc<Mutex<dyn VaultAdapter>>,
}

impl StorageHandle {
  /// Creates a new [`StorageHandle`].
  pub fn new(storage: impl VaultAdapter) -> Self {
    Self {
      data: Arc::new(Mutex::new(storage)),
    }
  }

  // ===========================================================================
  // Storage Adapter
  // ===========================================================================

  /// Returns a list of all resources matching the specified `type_`.
  pub async fn all(&self, type_: ResourceType) -> Result<Vec<Vec<u8>>> {
    self.__lock().await.all(type_).await
  }

  /// Returns the resource specified by `id`.
  pub async fn get(&self, id: ResourceId<'_>) -> Result<Vec<u8>> {
    self.__lock().await.get(id).await
  }

  /// Inserts or replaces the resource specified by `id` with `data`.
  pub async fn set(&self, id: ResourceId<'_>, data: Vec<u8>) -> Result<()> {
    self.__lock().await.set(id, data).await
  }

  /// Deletes the resource specified by `id`.
  pub async fn del(&self, id: ResourceId<'_>) -> Result<()> {
    self.__lock().await.del(id).await
  }

  // ===========================================================================
  // Vault Adapter
  // ===========================================================================

  pub async fn set_password(&self, password: EncryptionKey) -> Result<()> {
    self.__lock().await.set_password(password).await
  }

  pub async fn key_new(&self, location: &KeyLocation<'_>) -> Result<PublicKey> {
    self.__lock().await.key_new(location).await
  }

  pub async fn key_get(&self, location: &KeyLocation<'_>) -> Result<PublicKey> {
    self.__lock().await.key_get(location).await
  }

  pub async fn key_del(&self, location: &KeyLocation<'_>) -> Result<()> {
    self.__lock().await.key_del(location).await
  }

  pub async fn key_sign(&self, location: &KeyLocation<'_>, payload: Vec<u8>) -> Result<Signature> {
    self.__lock().await.key_sign(location, payload).await
  }

  // ===========================================================================
  // Private
  // ===========================================================================

  async fn __lock(&self) -> MutexGuard<'_, dyn VaultAdapter> {
    self.data.lock().await
  }
}

impl Debug for StorageHandle {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("StorageHandle")
  }
}
