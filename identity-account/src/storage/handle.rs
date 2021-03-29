// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::PublicKey;
use identity_core::crypto::KeyType;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

use crate::error::Result;
use crate::storage::VaultAdapter;
use crate::types::ToKey;
use crate::types::KeyLocation;
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

  /// Returns a list of deserialized resources.
  pub async fn json_all<T>(&self, type_: ResourceType) -> Result<Vec<T>>
  where
    T: for<'a> Deserialize<'a>,
  {
    self.all(type_).await.and_then(deserialize_list)
  }

  /// Deserializes and returns the resource specified by `key`.
  pub async fn json_get<T>(&self, key: &dyn ToKey) -> Result<T>
  where
    T: for<'a> Deserialize<'a>,
  {
    self.get(key).await.and_then(deserialize)
  }

  /// Serializes and inserts the given `data`.
  pub async fn json_set<T>(&self, key: &dyn ToKey, data: &T) -> Result<()>
  where
    T: Serialize,
  {
    self.set(key, data.to_json_vec()?).await
  }

  // ===========================================================================
  // Storage Adapter
  // ===========================================================================

  /// Returns a list of all resources matching the specified `type_`.
  pub async fn all(&self, type_: ResourceType) -> Result<Vec<Vec<u8>>> {
    self.__lock().await.all(type_).await
  }

  /// Returns the resource specified by `key`.
  pub async fn get(&self, key: &dyn ToKey) -> Result<Vec<u8>> {
    self.__lock().await.get(key).await
  }

  /// Inserts or replaces the resource specified by `key` with `data`.
  pub async fn set(&self, key: &dyn ToKey, data: Vec<u8>) -> Result<()> {
    self.__lock().await.set(key, data).await
  }

  /// Deletes the resource specified by `key`.
  pub async fn del(&self, key: &dyn ToKey) -> Result<()> {
    self.__lock().await.del(key).await
  }

  // ===========================================================================
  // Vault Adapter
  // ===========================================================================

  /// Sets the account password.
  pub async fn set_password(&self, password: EncryptionKey) -> Result<()> {
    self.__lock().await.set_password(password).await
  }

  /// Creates a new keypair at the specified `location`
  pub async fn key_new(&self, type_: KeyType, location: &KeyLocation<'_>) -> Result<PublicKey> {
    self.__lock().await.key_new(type_, location).await
  }

  /// Retrieves the public key at the specified `location`.
  pub async fn key_get(&self, type_: KeyType, location: &KeyLocation<'_>) -> Result<PublicKey> {
    self.__lock().await.key_get(type_, location).await
  }

  /// Deletes the keypair specified by `location`.
  pub async fn key_del(&self, type_: KeyType, location: &KeyLocation<'_>) -> Result<()> {
    self.__lock().await.key_del(type_, location).await
  }

  /// Signs the given `payload` with the private key at the specified `location`.
  pub async fn key_sign(&self, type_: KeyType, location: &KeyLocation<'_>, payload: Vec<u8>) -> Result<Signature> {
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

fn deserialize<T>(data: Vec<u8>) -> Result<T>
where
  T: for<'a> Deserialize<'a>,
{
  T::from_json_slice(&data).map_err(Into::into)
}

fn deserialize_list<T>(data: Vec<Vec<u8>>) -> Result<Vec<T>>
where
  T: for<'a> Deserialize<'a>,
{
  data.into_iter().map(deserialize).collect()
}
