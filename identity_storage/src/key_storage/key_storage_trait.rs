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

#[async_trait(?Send)]
pub trait KeyStorage {
  /// Generate a new key represented as a JSON Web Key.
  ///
  /// It's recommend that the implementer exposes constants for the supported [`KeyType`].
  async fn generate_jwk(&self, key_type: KeyType) -> KeyStorageResult<JwkGenOutput>;

  /// Sign the provided `data` using the private key identified by `key_id` with the specified `algorithm`.
  ///
  /// It's recommend that the implementer exposes constants for the supported [`SignatureAlgorithm`].
  async fn sign(&self, key_id: &KeyId, algorithm: SignatureAlgorithm, data: Vec<u8>) -> KeyStorageResult<Vec<u8>>;

  /// Returns the public key identified by `key_id` as a JSON Web Key.
  async fn public_jwk(&self, key_id: &KeyId) -> KeyStorageResult<Jwk>;

  /// Deletes the key identified by `key_id`.
  ///
  /// # Warning
  ///
  /// This operation cannot be undone. The keys are purged permanently.
  async fn delete(&self, key_id: &KeyId) -> KeyStorageResult<()>;
}
