// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use crate::key_storage::KeyStorageError;
pub use crate::key_storage::KeyStorageErrorKind;

use crate::key_storage::KeyId;
use crate::key_storage::KeyType;
use async_trait::async_trait;
use identity_jose::jwk::Jwk;
use identity_jose::jws::JwsAlgorithm;

use super::key_gen::JwkGenOutput;

/// Result of key storage operations.
pub type KeyStorageResult<T> = Result<T, KeyStorageError>;

#[cfg(not(feature = "send-sync-storage"))]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe {}
  impl<S: super::JwkStorage> StorageSendSyncMaybe for S {}
}

#[cfg(feature = "send-sync-storage")]
mod storage_sub_trait {
  pub trait StorageSendSyncMaybe: Send + Sync {}
  impl<S: Send + Sync + super::JwkStorage> StorageSendSyncMaybe for S {}
}

/// Secure storage for cryptographic keys represented as JWKs.
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait JwkStorage: storage_sub_trait::StorageSendSyncMaybe {
  /// Generate a new key represented as a JSON Web Key.
  ///
  /// It's recommend that the implementer exposes constants for the supported [`KeyType`].
  async fn generate(&self, key_type: KeyType, alg: JwsAlgorithm) -> KeyStorageResult<JwkGenOutput>;

  /// Insert an existing JSON Web Key into the storage.
  ///
  /// All private key components of the `jwk` must be set.
  async fn insert(&self, jwk: Jwk) -> KeyStorageResult<KeyId>;

  /// Sign the provided `data` using the private key identified by `key_id` with the specified `algorithm`.
  async fn sign(&self, key_id: &KeyId, data: &[u8], alg: JwsAlgorithm) -> KeyStorageResult<Vec<u8>>;

  /// Deletes the key identified by `key_id`.
  ///
  /// # Warning
  ///
  /// This operation cannot be undone. The keys are purged permanently.
  async fn delete(&self, key_id: &KeyId) -> KeyStorageResult<()>;

  /// Returns `true` if the key with the given `key_id` exists in storage, `false` otherwise.
  async fn exists(&self, key_id: &KeyId) -> KeyStorageResult<bool>;
}
