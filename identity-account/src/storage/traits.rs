// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;
use identity_core::crypto::KeyType;
use std::path::Path;

use crate::error::Result;
use crate::types::KeyLocation;
use crate::types::ResourceType;
use crate::types::Signature;
use crate::types::ToKey;
use crate::utils::fs;
use crate::utils::EncryptionKey;

#[async_trait::async_trait(?Send)]
pub trait StorageAdapter: Send + Sync + 'static {
  /// Returns a list of all resources matching the specified `type_`.
  async fn all(&mut self, type_: ResourceType) -> Result<Vec<Vec<u8>>>;

  /// Returns the resource specified by `key`.
  async fn get(&mut self, key: &dyn ToKey) -> Result<Vec<u8>>;

  /// Inserts or replaces the resource specified by `key` with `data`.
  async fn set(&mut self, key: &dyn ToKey, data: Vec<u8>) -> Result<()>;

  /// Deletes the resource specified by `key`.
  async fn del(&mut self, key: &dyn ToKey) -> Result<()>;

  fn storage_path(&self) -> &Path;

  fn storage_root(&self) -> &Path {
    let path: &Path = self.storage_path();

    if fs::maybe_file(path) {
      path.parent().unwrap_or(path)
    } else {
      path
    }
  }
}

#[async_trait::async_trait(?Send)]
pub trait VaultAdapter: StorageAdapter {
  /// Sets the account password.
  async fn set_password(&mut self, password: EncryptionKey) -> Result<()>;

  /// Creates a new keypair at the specified `location`
  async fn key_new(&mut self, type_: KeyType, location: &KeyLocation<'_>) -> Result<PublicKey>;

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&mut self, type_: KeyType, location: &KeyLocation<'_>) -> Result<PublicKey>;

  /// Deletes the keypair specified by `location`.
  async fn key_del(&mut self, type_: KeyType, location: &KeyLocation<'_>) -> Result<()>;

  /// Signs the given `payload` with the private key at the specified `location`.
  async fn key_sign(&mut self, type_: KeyType, location: &KeyLocation<'_>, payload: Vec<u8>) -> Result<Signature>;
}
