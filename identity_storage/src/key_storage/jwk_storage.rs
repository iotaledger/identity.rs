// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_storage::KeyId;
use crate::key_storage::KeyStorageError;
use crate::key_storage::KeyType;
use async_trait::async_trait;
use identity_verification::jose::jwk::Jwk;
use identity_verification::jose::jws::JwsAlgorithm;

use super::jwk_gen_output::JwkGenOutput;

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
  /// It is recommended that the implementer exposes constants for the supported [`KeyType`].
  async fn generate(&self, key_type: KeyType, alg: JwsAlgorithm) -> KeyStorageResult<JwkGenOutput>;

  /// Insert an existing JSON Web Key into the storage.
  ///
  /// All private key components of the `jwk` must be set.
  async fn insert(&self, jwk: Jwk) -> KeyStorageResult<KeyId>;

  /// Sign the provided `data` using the private key identified by `key_id` according to the requirements of
  /// the corresponding `public_key` (see [`Jwk::alg`](Jwk::alg()) etc.).
  ///
  /// # Note
  ///
  /// High level methods from this library calling this method are designed to always pass a `public_key` that
  /// corresponds to `key_id` and additional checks for this in the `sign` implementation are normally not required.
  /// This is however based on the expectation that the key material associated with a given [`KeyId`] is immutable.  
  async fn sign(&self, key_id: &KeyId, data: &[u8], public_key: &Jwk) -> KeyStorageResult<Vec<u8>>;

  /// Deletes the key identified by `key_id`.
  ///
  /// If the corresponding key does not exist in storage, a [`KeyStorageError`] with kind
  /// [`KeyNotFound`](crate::key_storage::KeyStorageErrorKind::KeyNotFound) must be returned.
  ///
  /// # Warning
  ///
  /// This operation cannot be undone. The keys are purged permanently.
  async fn delete(&self, key_id: &KeyId) -> KeyStorageResult<()>;

  /// Returns `true` if the key with the given `key_id` exists in storage, `false` otherwise.
  async fn exists(&self, key_id: &KeyId) -> KeyStorageResult<bool>;
}
