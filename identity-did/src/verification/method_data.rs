// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::common::Object;
use identity_core::utils::decode_b16;
use identity_core::utils::decode_b58;
use identity_core::utils::encode_b16;
use identity_core::utils::encode_b58;

use crate::error::Error;
use crate::error::Result;

/// Supported verification method data formats.
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum MethodData {
  PublicKeyBase58(String),
  PublicKeyHex(String),
  PublicKeyJwk(Object),
}

impl MethodData {
  /// Creates a new `MethodData` variant with base16-encoded content.
  pub fn new_b16(data: impl AsRef<[u8]>) -> Self {
    Self::PublicKeyHex(encode_b16(&data))
  }

  /// Creates a new `MethodData` variant with base58-encoded content.
  pub fn new_b58(data: impl AsRef<[u8]>) -> Self {
    Self::PublicKeyBase58(encode_b58(&data))
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
      Self::PublicKeyBase58(input) => decode_b58(input).map_err(|_| Error::InvalidKeyDataBase58),
      Self::PublicKeyHex(input) => decode_b16(input).map_err(|_| Error::InvalidKeyDataBase16),
      Self::PublicKeyJwk(_) => Err(Error::InvalidKeyData),
    }
  }
}

impl Debug for MethodData {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::PublicKeyBase58(inner) => f.write_fmt(format_args!("PublicKeyBase58({})", inner)),
      Self::PublicKeyHex(inner) => f.write_fmt(format_args!("PublicKeyHex({})", inner)),
      Self::PublicKeyJwk(inner) => f.debug_tuple("PublicKeyJwk").field(inner).finish(),
    }
  }
}
