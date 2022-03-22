// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Allow this even thought we implement Hash while deriving PartialEq.
// A test ensures compatibility between the two.
#![allow(clippy::derive_hash_xor_eq)]

use core::fmt::Debug;
use core::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use identity_core::utils::decode_b58;
use identity_core::utils::decode_multibase;
use identity_core::utils::encode_b58;
use identity_core::utils::encode_multibase;

use crate::error::Error;
use crate::error::Result;

/// Supported verification method data formats.
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum MethodData {
  PublicKeyMultibase(String),
  PublicKeyBase58(String),
}

impl MethodData {
  /// Creates a new `MethodData` variant with base58-encoded content.
  pub fn new_base58(data: impl AsRef<[u8]>) -> Self {
    Self::PublicKeyBase58(encode_b58(&data))
  }

  /// Creates a new `MethodData` variant with [Multibase]-encoded content.
  ///
  /// [Multibase]: https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03
  pub fn new_multibase(data: impl AsRef<[u8]>) -> Self {
    Self::PublicKeyMultibase(encode_multibase(&data, None))
  }

  /// Returns a `Vec<u8>` containing the decoded bytes of the `MethodData`.
  ///
  /// This is generally a public key identified by a `MethodType` value.
  ///
  /// # Errors
  ///
  /// Decoding can fail if `MethodData` has invalid content or cannot be
  /// represented as a vector of bytes.
  pub fn try_decode(&self) -> Result<Vec<u8>> {
    match self {
      Self::PublicKeyMultibase(input) => decode_multibase(input).map_err(|_| Error::InvalidKeyDataMultibase),
      Self::PublicKeyBase58(input) => decode_b58(input).map_err(|_| Error::InvalidKeyDataBase58),
    }
  }
}

impl Debug for MethodData {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::PublicKeyMultibase(inner) => f.write_fmt(format_args!("PublicKeyMultibase({})", inner)),
      Self::PublicKeyBase58(inner) => f.write_fmt(format_args!("PublicKeyBase58({})", inner)),
    }
  }
}

impl Hash for MethodData {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      MethodData::PublicKeyBase58(ref string) => {
        std::mem::discriminant(self).hash(state);
        string.hash(state);
      }
      MethodData::PublicKeyMultibase(ref string) => {
        std::mem::discriminant(self).hash(state);
        string.hash(state);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::hash_map::DefaultHasher;
  use std::hash::Hash;
  use std::hash::Hasher;

  use super::MethodData;

  #[test]
  fn test_partial_eq_and_hash_impls_agree() {
    let data1 = MethodData::PublicKeyBase58("public_key".to_owned());
    let data2 = MethodData::PublicKeyMultibase("public_key".to_owned());

    assert_eq!(data1, data1);
    let mut data_hasher1 = DefaultHasher::new();
    data1.hash(&mut data_hasher1);
    let mut data_hasher2 = DefaultHasher::new();
    data1.hash(&mut data_hasher2);
    assert_eq!(data_hasher1.finish(), data_hasher2.finish());

    assert_eq!(data2, data2);
    let mut data_hasher1 = DefaultHasher::new();
    data2.hash(&mut data_hasher1);
    let mut data_hasher2 = DefaultHasher::new();
    data2.hash(&mut data_hasher2);
    assert_eq!(data_hasher1.finish(), data_hasher2.finish());

    assert_ne!(data1, data2);
    let mut data_hasher1 = DefaultHasher::new();
    data1.hash(&mut data_hasher1);
    let mut data_hasher2 = DefaultHasher::new();
    data2.hash(&mut data_hasher2);
    assert_ne!(data_hasher1.finish(), data_hasher2.finish());
  }
}
