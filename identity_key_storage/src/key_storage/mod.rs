// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod error;
pub use error::KeyStorageError;
pub use error::KeyStorageErrorKind;

use std::str::FromStr;

use async_trait::async_trait;
use identity_data_integrity::verification_material::VerificationMaterial;

use crate::identifiers::KeyId;
use crate::key_generation::MultikeyOutput;
use crate::key_generation::MultikeySchema;
use crate::signature::Signature;

pub type KeyStorageResult<T> = Result<T, crate::key_storage::KeyStorageError>;

#[async_trait(?Send)]
pub trait KeyStorage {
  /// Signing algorithms supported by the `KeyStorage`.
  /// This will typically be an enum of various cryptographic
  /// signature algorithms supported by the key storage.
  type SigningAlgorithm: std::fmt::Display + FromStr + TryFrom<String> + AsRef<str>;

  /// Generate and store a private, public key pair
  /// in compliance with the Multikey specification.
  /// The implementation must return a Multibase encoding of the public key, and the `KeyId`
  /// corresponding to the private key.
  async fn generate_multikey(&self, schema: &MultikeySchema) -> KeyStorageResult<MultikeyOutput>;

  /// Sign the provided data using the private key corresponding to the given `key_id` with the specified `algorithm`.
  async fn sign(
    data: Vec<u8>,
    key_identifier: &KeyId,
    algorithm: &Self::SigningAlgorithm,
  ) -> KeyStorageResult<Signature>;

  /// Returns the public key associated with the provided `key_identifier`.
  async fn public(key_identifier: &KeyId) -> KeyStorageResult<VerificationMaterial>;

  /// Deletes the secured keys associated with the provided `key_identifier`.
  ///
  /// # Warning
  /// This operation cannot be undone. The keys associated with `key_identifier` will be lost forever.
  async fn delete(&self, key_identifier: &KeyId) -> KeyStorageResult<()>;
}
