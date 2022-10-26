// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_trait::async_trait;
use identity_core::crypto::PublicKey;

use crate::KeyId;
use crate::Signature;
use crate::StorageResult;

#[cfg(not(feature = "send-sync-storage"))]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe {}
  impl<S: super::KeyStorage> StorageSendSyncMaybe for S {}
}

#[cfg(feature = "send-sync-storage")]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe: Send + Sync {}
  impl<S: Send + Sync + super::KeyStorage> StorageSendSyncMaybe for S {}
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
// TODO: Copy docs from legacy `Storage` interface.
pub trait KeyStorage: storage_sub_trait::StorageSendSyncMaybe {
  type KeyType: Send + Sync + 'static;
  type SigningAlgorithm: Send + Sync + 'static;

  async fn generate(&self, key_type: Self::KeyType) -> StorageResult<KeyId>;
  // async fn import(key_type: KeyType, private_key: PrivateKey) -> StorageResult<KeyId>;
  async fn public(&self, private_key: &KeyId) -> StorageResult<PublicKey>;
  // async fn delete(private_key: &KeyId) -> StorageResult<bool>;
  async fn sign<SIG: Send + Into<Self::SigningAlgorithm>>(
    &self,
    private_key: &KeyId,
    signing_algorithm: SIG,
    data: Vec<u8>,
  ) -> StorageResult<Signature>;
  // async fn encrypt(
  //   plaintext: Vec<u8>,
  //   associated_data: Vec<u8>,
  //   encryption_algorithm: &EncryptionAlgorithm,
  //   cek_algorithm: &CekAlgorithm,
  //   public_key: PublicKey,
  // ) -> StorageResult<EncryptedData>;
  // async fn decrypt(
  //   private_key: &KeyId,
  //   data: EncryptedData,
  //   encryption_algorithm: &EncryptionAlgorithm,
  //   cek_algorithm: &CekAlgorithm,
  // ) -> Vec<u8>;
  // async fn flush(&self) -> StorageResult<()>;
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<T> KeyStorage for Arc<T>
where
  T: KeyStorage,
  T::KeyType: Send,
{
  type KeyType = T::KeyType;
  type SigningAlgorithm = T::SigningAlgorithm;

  async fn generate(&self, key_type: Self::KeyType) -> StorageResult<KeyId> {
    T::generate(self, key_type).await
  }

  async fn public(&self, private_key: &KeyId) -> StorageResult<PublicKey> {
    T::public(self, private_key).await
  }

  async fn sign<SIG: Send + Into<Self::SigningAlgorithm>>(
    &self,
    private_key: &KeyId,
    signing_algorithm: SIG,
    data: Vec<u8>,
  ) -> StorageResult<Signature> {
    T::sign(self, private_key, signing_algorithm, data).await
  }
}
