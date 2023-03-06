// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

/// Error type for a failed jws signature verification. See [`JwsSignatureVerifier`](super::JwsSignatureVerifier).
#[derive(Debug, thiserror::Error)]
#[error("jws signature verification failed: {kind}")]
pub struct SignatureVerificationError {
  kind: SignatureVerificationErrorKind,
  #[source]
  source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl SignatureVerificationError {
  /// Constructs a new [`SignatureVerificationError`].
  pub fn new(cause: SignatureVerificationErrorKind) -> Self {
    Self {
      kind: cause,
      source: None,
    }
  }

  /// Returns the cause of the [`SignatureVerificationError`].
  pub fn kind(&self) -> &SignatureVerificationErrorKind {
    &self.kind
  }
  /// Updates the `source` of the [`SignatureVerificationError`].
  pub fn with_source(self, source: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
    self._with_source(source.into())
  }

  fn _with_source(mut self, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
    self.source = Some(source);
    self
  }
}

impl From<SignatureVerificationErrorKind> for SignatureVerificationError {
  fn from(value: SignatureVerificationErrorKind) -> Self {
    Self::new(value)
  }
}

/// The cause of a failed jws signature verification.
#[derive(Debug)]
#[non_exhaustive]
pub enum SignatureVerificationErrorKind {
  /// Indicates that the [`JwsSignatureVerifier`](super::JwsSignatureVerifier) implementation is not compatible with
  /// the `alg` extracted from the JOSE header.
  UnsupportedAlg,
  /// Indicates that the [`JwsSignatureVerifier`](super::JwsSignatureVerifier) implementation does not support the
  /// `kty` of the provided [`Jwk`](crate::jwk::Jwk).
  UnsupportedKeyType,
  /// Indicates that the [`JwsSignatureVerifier`](super::JwsSignatureVerifier) implementation does not support the
  /// public key parameters extracted from the provided [`Jwk`](crate::jwk::Jwk).
  UnsupportedKeyParams,
  /// Indicates that the [`JwsSignatureVerifier`](super::JwsSignatureVerifier) implementation failed to decode the
  /// public key extracted from the provided [`Jwk`](crate::jwk::Jwk).
  KeyDecodingFailure,
  /// Indicates that the [`JwsSignatureVerifier`](super::JwsSignatureVerifier) implementation considers the signature
  /// to be invalid.
  InvalidSignature,
  /// Indicates that something went wrong when calling
  /// [`JwsSignatureVerifier::verify`](super::JwsSignatureVerifier::verify), but it is unclear whether the reason
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
