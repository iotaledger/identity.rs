// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;
use std::path::Path;

use crate::error::Result;
use crate::storage::KeyLocation;
use crate::storage::ResourceId;
use crate::storage::ResourceType;
use crate::types::Signature;
use crate::utils::fs;
use crate::utils::EncryptionKey;

#[async_trait::async_trait]
pub trait StorageAdapter: Send + Sync + 'static {
  /// Returns a list of all resources matching the specified `type_`.
  async fn all(&mut self, type_: ResourceType) -> Result<Vec<Vec<u8>>>;

  /// Returns the resource specified by `id`.
  async fn get(&mut self, id: ResourceId<'_>) -> Result<Vec<u8>>;

  /// Inserts or replaces the resource specified by `id` with `data`.
  async fn set(&mut self, id: ResourceId<'_>, data: Vec<u8>) -> Result<()>;

  /// Deletes the resource specified by `id`.
  async fn del(&mut self, id: ResourceId<'_>) -> Result<()>;

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

#[async_trait::async_trait]
pub trait VaultAdapter: StorageAdapter {
  /// Sets the account password.
  async fn set_password(&mut self, password: EncryptionKey) -> Result<()>;

  /// Creates a new keypair at the specified `location`
  async fn key_new(&mut self, location: &KeyLocation<'_>) -> Result<PublicKey>;

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&mut self, location: &KeyLocation<'_>) -> Result<PublicKey>;

  /// Deletes the keypair specified by `location`.
  async fn key_del(&mut self, location: &KeyLocation<'_>) -> Result<()>;

  /// Signs the given `payload` with the private key at the specified `location`.
  async fn key_sign(&mut self, location: &KeyLocation<'_>, payload: Vec<u8>) -> Result<Signature>;
}
