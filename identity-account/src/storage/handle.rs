// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::crypto::PublicKey;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::Result;
use crate::storage::KeyLocation;
use crate::storage::Signature;
use crate::storage::VaultAdapter;

/// A thread-safe wrapper around a [`VaultAdapter`] implementation.
#[derive(Clone)]
pub struct StorageHandle {
  data: Arc<Mutex<dyn VaultAdapter>>,
}

impl StorageHandle {
  /// Creates a new [`StorageHandle`].
  pub fn new(storage: Box<dyn VaultAdapter>) -> Self {
    Self {
      data: Arc::new(Mutex::new(storage)),
    }
  }

  pub async fn all(&self) -> Result<Vec<Vec<u8>>> {
    self.data.lock().await.all().await
  }

  pub async fn get(&self, resource_id: &[u8]) -> Result<Vec<u8>> {
    self.data.lock().await.get(resource_id).await
  }

  pub async fn set(&self, resource_id: &[u8], resource: &[u8]) -> Result<()> {
    self.data.lock().await.set(resource_id, resource).await
  }

  pub async fn del(&self, resource_id: &[u8]) -> Result<()> {
    self.data.lock().await.del(resource_id).await
  }

  /// Generates a new public key at the specified location.
  pub async fn generate_public_key(&self, location: KeyLocation<'_>) -> Result<PublicKey> {
    self.data.lock().await.generate_public_key(location).await
  }

  /// Retrieves the public key at the specified location.
  pub async fn retrieve_public_key(&mut self, location: KeyLocation<'_>) -> Result<PublicKey> {
    self.data.lock().await.retrieve_public_key(location).await
  }

  /// Signs the given `payload` with the private key at the specified location.
  pub async fn generate_signature(&mut self, payload: Vec<u8>, location: KeyLocation<'_>) -> Result<Signature> {
    self.data.lock().await.generate_signature(payload, location).await
  }
}

impl Debug for StorageHandle {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("StorageHandle")
  }
}
