// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use crate::key_storage::KeyStorageError;
pub use crate::key_storage::KeyStorageErrorKind;
use crate::key_storage::SignatureAlgorithm;

use crate::key_storage::KeyId;
use crate::key_storage::KeyType;
use async_trait::async_trait;
use identity_jose::jwk::Jwk;

use super::key_gen::JwkGenOutput;

pub type KeyStorageResult<T> = Result<T, KeyStorageError>;

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
pub trait KeyStorage: storage_sub_trait::StorageSendSyncMaybe {
  /// Generate a new key represented as a JSON Web Key.
  ///
  /// It's recommend that the implementer exposes constants for the supported [`KeyType`].
  async fn generate_jwk(&self, key_type: KeyType) -> KeyStorageResult<JwkGenOutput>;

  /// Insert an existing JSON Web Key into the storage.
  ///
  /// All private key components of the `jwk` must be set.
  async fn insert_jwk(&self, jwk: Jwk) -> KeyStorageResult<KeyId>;

  /// Sign the provided `data` using the private key identified by `key_id` with the specified `algorithm`.
  ///
  /// It's recommend that the implementer exposes constants for the supported [`SignatureAlgorithm`].
  async fn sign(&self, key_id: &KeyId, algorithm: SignatureAlgorithm, data: Vec<u8>) -> KeyStorageResult<Vec<u8>>;

  /// Returns the public key identified by `key_id` as a JSON Web Key.
  async fn public_jwk(&self, key_id: &KeyId) -> KeyStorageResult<Jwk>;

  /// Deletes the key identified by `key_id`.
  ///
  /// This operation is idempotent: it does not fail if the key does not exist.
  /// Storages should return `true` if the key existed and was deleted, and `false` otherwise.
  ///
  /// # Warning
  ///
  /// This operation cannot be undone. The keys are purged permanently.
  async fn delete(&self, key_id: &KeyId) -> KeyStorageResult<bool>;

  /// Returns `true` if the key with the given `key_id` exists in storage, `false` otherwise.
  async fn exists(&self, key_id: &KeyId) -> KeyStorageResult<bool>;
}
