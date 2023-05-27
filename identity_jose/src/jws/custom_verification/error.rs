// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

/// Error type for a failed jws signature verification. See [`JwsVerifier`](super::JwsVerifier).
pub type SignatureVerificationError = identity_core::common::SingleStructError<SignatureVerificationErrorKind>;

/// The cause of a failed jws signature verification.
#[derive(Debug)]
#[non_exhaustive]
pub enum SignatureVerificationErrorKind {
  /// Indicates that the [`JwsVerifier`](super::JwsVerifier) implementation is not compatible with
  /// the `alg` extracted from the JOSE header.
  UnsupportedAlg,
  /// Indicates that the [`JwsVerifier`](super::JwsVerifier) implementation does not support the
  /// `kty` of the provided [`Jwk`](crate::jwk::Jwk).
  UnsupportedKeyType,
  /// Indicates that the [`JwsVerifier`](super::JwsVerifier) implementation does not support the
  /// public key parameters extracted from the provided [`Jwk`](crate::jwk::Jwk).
  UnsupportedKeyParams,
  /// Indicates that the [`JwsVerifier`](super::JwsVerifier) implementation failed to decode the
  /// public key extracted from the provided [`Jwk`](crate::jwk::Jwk).
  KeyDecodingFailure,
  /// Indicates that the [`JwsVerifier`](super::JwsVerifier) implementation considers the signature
  /// to be invalid.
  InvalidSignature,
  /// Indicates that something went wrong when calling
  /// [`JwsVerifier::verify`](super::JwsVerifier::verify), but it is unclear whether the reason
  /// matches any of the other variants.
  Unspecified,
}

impl SignatureVerificationErrorKind {
  const fn as_str(&self) -> &str {
    match self {
      Self::UnsupportedAlg => "unsupported alg",
      Self::UnsupportedKeyType => "unsupported key type",
      Self::UnsupportedKeyParams => "unsupported key parameters",
      Self::KeyDecodingFailure => "key decoding failure",
      Self::InvalidSignature => "invalid signature",
      Self::Unspecified => "unspecified failure",
    }
  }
}

impl Display for SignatureVerificationErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}
