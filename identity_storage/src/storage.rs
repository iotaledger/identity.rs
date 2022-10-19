// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::IdentityState;
use crate::KeyAlias;
use crate::KeyStorage;
use crate::Signature;
use crate::StorageResult;
use async_trait::async_trait;
use identity_core::crypto::PublicKey;

pub struct Storage<K>
where
  K: KeyStorage,
{
  key_storage: K,
  identity_state: IdentityState,
}

impl<K> Storage<K>
where
  K: KeyStorage,
{
  /// Combines a key and blob storage implementation to a full [`Storage`].
  pub fn new(key_storage: K, identity_state: IdentityState) -> Self {
    Self {
      key_storage,
      identity_state,
    }
  }
}

// #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
// #[cfg_attr(feature = "send-sync-storage", async_trait)]
// impl<K: KeyStorage, B: BlobStorage> BlobStorage for Storage<K, B> {
//   async fn store(&self, did: &CoreDID, blob: Option<Vec<u8>>) -> StorageResult<()> {
//     self.blob_storage.store(did, blob).await
//   }

//   async fn load(&self, did: &CoreDID) -> StorageResult<Option<Vec<u8>>> {
//     self.blob_storage.load(did).await
//   }

//   async fn flush(&self) -> StorageResult<()> {
//     self.blob_storage.flush().await
//   }
// }

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<K: KeyStorage> KeyStorage for Storage<K> {
  type KeyType = K::KeyType;
  type SigningAlgorithm = K::SigningAlgorithm;

  async fn generate(&self, key_type: Self::KeyType) -> StorageResult<KeyAlias> {
    self.key_storage.generate(key_type).await
  }

  async fn public(&self, private_key: &KeyAlias) -> StorageResult<PublicKey> {
    self.key_storage.public(private_key).await
  }

  async fn sign<SIG: Send + Into<Self::SigningAlgorithm>>(
    &self,
    private_key: &KeyAlias,
    signing_algorithm: SIG,
    data: Vec<u8>,
  ) -> StorageResult<Signature> {
    self.key_storage.sign(private_key, signing_algorithm, data).await
  }
}
