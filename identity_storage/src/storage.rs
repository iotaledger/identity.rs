// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::BlobStorage;
use crate::KeyId;
use crate::KeyStorage;
use crate::Signature;
use crate::StorageResult;
use async_trait::async_trait;
use identity_core::crypto::PublicKey;

pub struct Storage<K, B>
where
  K: KeyStorage,
  B: BlobStorage,
{
  pub(crate) key_storage: Arc<K>,
  blob_storage: Arc<B>,
}

impl<K, B> Clone for Storage<K, B>
where
  K: KeyStorage,
  B: BlobStorage,
{
  fn clone(&self) -> Self {
    Self {
      key_storage: Arc::clone(&self.key_storage),
      blob_storage: Arc::clone(&self.blob_storage),
    }
  }
}

impl<K, B> Storage<K, B>
where
  K: KeyStorage,
  B: BlobStorage,
{
  /// Combines a key and blob storage implementation to a full [`Storage`].
  pub fn new(key_storage: K, blob_storage: B) -> Self {
    Self {
      key_storage: Arc::new(key_storage),
      blob_storage: Arc::new(blob_storage),
    }
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<K: KeyStorage, B: BlobStorage> BlobStorage for Storage<K, B> {
  async fn store(&self, key: &str, blob: Option<Vec<u8>>) -> StorageResult<()> {
    self.blob_storage.store(key, blob).await
  }

  async fn load(&self, key: &str) -> StorageResult<Option<Vec<u8>>> {
    self.blob_storage.load(key).await
  }

  async fn flush(&self) -> StorageResult<()> {
    self.blob_storage.flush().await
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<K: KeyStorage, B: BlobStorage> KeyStorage for Storage<K, B> {
  type KeyType = K::KeyType;
  type SigningAlgorithm = K::SigningAlgorithm;

  async fn generate(&self, key_type: Self::KeyType) -> StorageResult<KeyId> {
    self.key_storage.generate(key_type).await
  }

  async fn public(&self, private_key: &KeyId) -> StorageResult<PublicKey> {
    self.key_storage.public(private_key).await
  }

  async fn sign<SIG: Send + Into<Self::SigningAlgorithm>>(
    &self,
    private_key: &KeyId,
    signing_algorithm: SIG,
    data: Vec<u8>,
  ) -> StorageResult<Signature> {
    self.key_storage.sign(private_key, signing_algorithm, data).await
  }
}
