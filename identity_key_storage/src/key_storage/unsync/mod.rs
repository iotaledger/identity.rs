// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_trait::async_trait;
use identity_data_integrity_types::verification_material::PublicKeyMultibase;

use crate::key_generation::KeyId;
use crate::key_generation::MultikeyOutput;
use crate::key_generation::MultikeySchema;
use crate::signature::Signature;

type KeyStorageResult<T> = Result<T, crate::key_storage::KeyStorageError>;
#[async_trait(?Send)]
trait KeyStorage {
  /// Signing algorithms supported by the `KeyStorage`.
  /// This will typically be an enum of various cryptographic
  /// signature algorithms supported by the key storage.
  type SigningAlgorithm: std::fmt::Display + FromStr + TryFrom<String> + AsRef<str>;

  /// Generate and store a private, public key pair
  /// in compliance with the Multikey specification.
  /// The implementation must return a Multibase encoding of the public key, and the `KeyId`
  /// corresponding to the private key.
  async fn generate_multikey(schema: &MultikeySchema) -> KeyStorageResult<MultikeyOutput>;

  /// Sign the provided data using the private key corresponding to the given `key_id` with the specified `algorithm`. 
  async fn sign(data: &[u8], key_id: &KeyId, algorithm: &Self::SigningAlgorithm) -> KeyStorageResult<Signature>; 
}
