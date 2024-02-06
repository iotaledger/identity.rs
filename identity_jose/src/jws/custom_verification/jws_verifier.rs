// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::SignatureVerificationError;
use crate::jwk::Jwk;
use crate::jws::JwsAlgorithm;

/// Input a [`JwsVerifier`] verifies.
pub struct VerificationInput {
  /// The `alg` parsed from the protected header.
  pub alg: JwsAlgorithm,
  /// The signing input.
  ///
  /// See [RFC 7515: section 5.2 part 8.](https://www.rfc-editor.org/rfc/rfc7515#section-5.2) and
  /// [RFC 7797 section 3](https://www.rfc-editor.org/rfc/rfc7797#section-3).
  pub signing_input: Box<[u8]>,
  /// The decoded signature to validate the `signing_input` against in the manner defined by the `alg` field.
  pub decoded_signature: Box<[u8]>,
}

/// Trait for cryptographically verifying a JWS signature.
///
/// Any type implementing this trait can be passed to
/// [`JwsValidationItem::verify`](`crate::jws::JwsValidationItem::verify`) which is intended
/// as the most convenient way to verify a decoded JWS.
///
/// [`JwsValidationItem::verify`](crate::jws::JwsValidationItem::verify)
///
/// ## Implementation
///
/// Implementers are expected to provide a procedure for step 8 of
/// [RFC 7515 section 5.2](https://www.rfc-editor.org/rfc/rfc7515#section-5.2) for
/// the JWS signature algorithms they want to support.
///
/// Custom implementations can be constructed inline by converting a suitable closure into a [`JwsVerifierFn`]
/// using the [`From`] trait.
///
/// ## Default implementation
///
/// When the `eddsa` feature is enabled one can construct an implementor
/// provided by the IOTA Identity library. See
/// [`EdDSAJwsVerifier::verify`](EdDSAJwsVerifier::verify).
pub trait JwsVerifier {
  /// Validate the `decoded_signature` against the `signing_input` in the manner defined by `alg` using the
  /// `public_key`.
  ///
  /// Implementors may decide to error with
  /// [`SignatureVerificationErrorKind::UnsupportedAlg`](crate::jws::SignatureVerificationErrorKind::UnsupportedAlg) if
  /// they are not interested in supporting a given algorithm.
  fn verify(&self, input: VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError>;
}

impl JwsVerifier for Box<dyn JwsVerifier> {
  fn verify(&self, input: VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
    <dyn JwsVerifier>::verify(self, input, public_key)
  }
}

// =================================================================================================================
// Implementation
// ================================================================================================================

/// Simple wrapper around a closure capable of verifying a JWS signature. This wrapper implements
/// [`JwsVerifier`].
///
/// Note: One can convert a closure to this wrapper using the [`From`] trait.
pub struct JwsVerifierFn<F>(F);
impl<F> From<F> for JwsVerifierFn<F>
where
  F: Fn(VerificationInput, &Jwk) -> Result<(), SignatureVerificationError>,
{
  fn from(value: F) -> Self {
    Self(value)
  }
}

impl<F> JwsVerifier for JwsVerifierFn<F>
where
  F: Fn(VerificationInput, &Jwk) -> Result<(), SignatureVerificationError>,
{
  fn verify(&self, input: VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
    self.0(input, public_key)
  }
}
