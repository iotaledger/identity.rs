// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::convert::ToJson;
use identity_core::crypto::PublicKey;
use identity_did::verification::MethodType;
use serde::Deserialize;
use std::collections::HashSet;
use serde::Serialize;
use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

use crate::chain::ChainId;
use crate::error::Result;
use crate::storage::VaultAdapter;
use crate::utils::EncryptionKey;
use crate::types::KeyLocation;
use crate::types::Resource;
use crate::types::Signature;
use crate::utils;

/// A thread-safe wrapper around a [VaultAdapter] implementation.
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

  /// Returns a list of deserialized resources.
  pub async fn json_all<T>(&self, resource: Resource) -> Result<Vec<T>>
  where
    T: for<'a> Deserialize<'a>,
  {
    self.all(resource).await.and_then(utils::deserialize_list)
  }

  /// Deserializes and returns the resource specified by `key`.
  pub async fn json_get<T>(&self, resource: Resource, key: &[u8]) -> Result<T>
  where
    T: for<'a> Deserialize<'a>,
  {
    self.get(resource, key).await.and_then(utils::deserialize)
  }

  /// Serializes and inserts the given `data`.
  pub async fn json_set<T>(&self, resource: Resource, key: &[u8], data: &T) -> Result<()>
  where
    T: Serialize,
  {
    self.set(resource, key, data.to_json_vec()?).await
  }

  // ===========================================================================
  // Storage Adapter
  // ===========================================================================

  /// Returns a list of all resources matching the specified `type_`.
  pub async fn all(&self, resource: Resource) -> Result<Vec<Vec<u8>>> {
    self.__lock().await.all(resource).await
  }

  /// Returns the resource specified by `key`.
  pub async fn get(&self, resource: Resource, key: &[u8]) -> Result<Vec<u8>> {
    self.__lock().await.get(resource, key).await
  }

  /// Inserts or replaces the resource specified by `key` with `data`.
  pub async fn set(&self, resource: Resource, key: &[u8], data: Vec<u8>) -> Result<()> {
    self.__lock().await.set(resource, key, data).await
  }

  /// Deletes the resource specified by `key`.
  pub async fn del(&self, resource: Resource, key: &[u8]) -> Result<()> {
    self.__lock().await.del(resource, key).await
  }

  pub async fn storage_path(&self) -> PathBuf {
    self.__lock().await.storage_path().to_path_buf()
  }

  pub async fn storage_root(&self) -> PathBuf {
    self.__lock().await.storage_root().to_path_buf()
  }

  // ===========================================================================
  // Vault Adapter
  // ===========================================================================

  /// Sets the account password.
  pub async fn set_password(&self, password: EncryptionKey) -> Result<()> {
    self.__lock().await.set_password(password).await
  }

  /// Creates a new keypair at the specified `location`
  pub async fn key_new(&self, type_: MethodType, location: &KeyLocation) -> Result<PublicKey> {
    self.__lock().await.key_new(type_, location).await
  }

  /// Retrieves the public key at the specified `location`.
  pub async fn key_get(&self, type_: MethodType, location: &KeyLocation) -> Result<PublicKey> {
    self.__lock().await.key_get(type_, location).await
  }

  /// Deletes the keypair specified by `location`.
  pub async fn key_del(&self, type_: MethodType, location: &KeyLocation) -> Result<()> {
    self.__lock().await.key_del(type_, location).await
  }

  /// Signs the given `payload` with the private key at the specified `location`.
  pub async fn key_sign(&self, type_: MethodType, location: &KeyLocation, payload: Vec<u8>) -> Result<Signature> {
    self.__lock().await.key_sign(type_, location, payload).await
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
