// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;
use std::path::Path;

use crate::error::Result;
use crate::storage::KeyLocation;
use crate::storage::Signature;
use crate::utils::fs;
use crate::utils::EncryptionKey;

#[async_trait::async_trait]
pub trait StorageAdapter: Send + Sync {
  async fn all(&mut self) -> Result<Vec<Vec<u8>>>;

  async fn get(&mut self, resource_id: &[u8]) -> Result<Vec<u8>>;

  async fn set(&mut self, resource_id: &[u8], resource: &[u8]) -> Result<()>;

  async fn del(&mut self, resource_id: &[u8]) -> Result<()>;

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
  async fn set_password(&self, password: EncryptionKey) -> Result<()>;

  async fn generate_public_key(&mut self, location: KeyLocation<'_>) -> Result<PublicKey>;

  async fn retrieve_public_key(&mut self, location: KeyLocation<'_>) -> Result<PublicKey>;

  async fn generate_signature(&mut self, payload: Vec<u8>, location: KeyLocation<'_>) -> Result<Signature>;
}

macro_rules! impl_storage_deref {
  ($trait:ident) => {
    #[async_trait::async_trait]
    impl StorageAdapter for Box<dyn $trait> {
      async fn all(&mut self) -> Result<Vec<Vec<u8>>> {
        (**self).all().await
      }

      async fn get(&mut self, resource_id: &[u8]) -> Result<Vec<u8>> {
        (**self).get(resource_id).await
      }

      async fn set(&mut self, resource_id: &[u8], resource: &[u8]) -> Result<()> {
        (**self).set(resource_id, resource).await
      }

      async fn del(&mut self, resource_id: &[u8]) -> Result<()> {
        (**self).del(resource_id).await
      }

      fn storage_path(&self) -> &Path {
        (**self).storage_path()
      }
    }
  };
}
impl_storage_deref!(StorageAdapter);
impl_storage_deref!(VaultAdapter);

macro_rules! impl_vault_deref {
  ($trait:ident) => {
    #[async_trait::async_trait]
    impl VaultAdapter for Box<dyn $trait> {
      async fn set_password(&self, password: EncryptionKey) -> Result<()> {
        (**self).set_password(password).await
      }

      async fn generate_public_key(&mut self, location: KeyLocation<'_>) -> Result<PublicKey> {
        (**self).generate_public_key(location).await
      }

      async fn retrieve_public_key(&mut self, location: KeyLocation<'_>) -> Result<PublicKey> {
        (**self).retrieve_public_key(location).await
      }

      async fn generate_signature(&mut self, payload: Vec<u8>, location: KeyLocation<'_>) -> Result<Signature> {
        (**self).generate_signature(payload, location).await
      }
    }
  };
}
impl_vault_deref!(VaultAdapter);
