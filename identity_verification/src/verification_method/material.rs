// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jose::jwk::Jwk;
use core::fmt::Debug;
use core::fmt::Formatter;
use identity_core::convert::BaseEncoding;

use crate::error::Error;
use crate::error::Result;

/// Supported verification method data formats.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum MethodData {
  /// Verification Material in multibase encoding.
  PublicKeyMultibase(String),
  /// Verification Material in base58 encoding.
  PublicKeyBase58(String),
  /// Verification Material in the JSON Web Key format.
  PublicKeyJwk(Jwk),
}

impl MethodData {
  /// Creates a new `MethodData` variant with base58-encoded content.
  pub fn new_base58(data: impl AsRef<[u8]>) -> Self {
    Self::PublicKeyBase58(BaseEncoding::encode_base58(&data))
  }

  /// Creates a new `MethodData` variant with [Multibase]-encoded content.
  ///
  /// [Multibase]: https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03
  pub fn new_multibase(data: impl AsRef<[u8]>) -> Self {
    Self::PublicKeyMultibase(BaseEncoding::encode_multibase(&data, None))
  }

  /// Returns a `Vec<u8>` containing the decoded bytes of the `MethodData`.
  ///
  /// This is generally a public key identified by a `MethodType` value.
  ///
  /// # Errors
  /// Decoding will fail if `MethodData` is a [`Jwk`], has invalid content or cannot be
  /// represented as a vector of bytes.
  pub fn try_decode(&self) -> Result<Vec<u8>> {
    match self {
      Self::PublicKeyJwk(_) => Err(Error::InvalidMethodDataTransformation(
        "method data is not base encoded",
      )),
      Self::PublicKeyMultibase(input) => {
        BaseEncoding::decode_multibase(input).map_err(|_| Error::InvalidKeyDataMultibase)
      }
      Self::PublicKeyBase58(input) => BaseEncoding::decode_base58(input).map_err(|_| Error::InvalidKeyDataBase58),
    }
  }

  /// Returns the wrapped `Jwk` if the format is [`MethodData::PublicKeyJwk`].
  pub fn public_key_jwk(&self) -> Option<&Jwk> {
    if let Self::PublicKeyJwk(ref jwk) = self {
      Some(jwk)
    } else {
      None
    }
  }

  /// Fallible version of [`Self::public_key_jwk`](Self::public_key_jwk()).
  pub fn try_public_key_jwk(&self) -> Result<&Jwk> {
    self.public_key_jwk().ok_or(Error::NotPublicKeyJwk)
  }
}

impl Debug for MethodData {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::PublicKeyJwk(inner) => f.write_fmt(format_args!("PublicKeyJwk({inner:#?})")),
      Self::PublicKeyMultibase(inner) => f.write_fmt(format_args!("PublicKeyMultibase({inner})")),
      Self::PublicKeyBase58(inner) => f.write_fmt(format_args!("PublicKeyBase58({inner})")),
    }
  }
}
