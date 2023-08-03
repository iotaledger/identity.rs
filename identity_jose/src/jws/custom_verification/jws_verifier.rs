// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::SignatureVerificationError;
use crate::jwk::Jwk;
use crate::jws::JwsAlgorithm;

#[cfg(any(feature = "eddsa", doc))]
pub use eddsa_verifier::*;

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

#[cfg(any(feature = "eddsa", doc))]
mod eddsa_verifier {
  use std::ops::Deref;

  use super::*;
  use crate::jwk::EdCurve;
  use crate::jwk::JwkParamsOkp;
  use crate::jws::SignatureVerificationErrorKind;

  /// An implementor of [`JwsVerifier`] that can handle the
  /// [`JwsAlgorithm::EdDSA`](crate::jws::JwsAlgorithm::EdDSA) algorithm.
  ///
  /// See [`Self::verify`](EdDSAJwsVerifier::verify).
  ///
  /// NOTE: This type can only be constructed when the `eddsa` feature is enabled.
  #[derive(Debug)]
  #[non_exhaustive]
  pub struct EdDSAJwsVerifier;

  impl EdDSAJwsVerifier {
    /// Verify a JWS signature secured with the [`JwsAlgorithm::EdDSA`](crate::jws::JwsAlgorithm::EdDSA) algorithm.
    /// Only the [`EdCurve::Ed25519`] variant is supported for now. This associated method is only available when the
    /// `eddsa` feature is enabled.
    ///
    /// This function is useful when one is building a [`JwsVerifier`] that handles the
    /// [`JwsAlgorithm::EdDSA`](crate::jws::JwsAlgorithm::EdDSA) in the same manner as the [`EdDSAJwsVerifier`]
    /// hence extending its capabilities.
    ///
    /// # Warning
    /// This function does not check whether `alg = EdDSA` in the protected header. Callers are expected to assert this
    /// prior to calling the function.
    pub fn verify_eddsa(input: VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
      // Obtain an Ed25519 public key
      let params: &JwkParamsOkp = public_key
        .try_okp_params()
        .map_err(|_| SignatureVerificationErrorKind::UnsupportedKeyType)?;

      if params
        .try_ed_curve()
        .ok()
        .filter(|curve_param| *curve_param == EdCurve::Ed25519)
        .is_none()
      {
        return Err(SignatureVerificationErrorKind::UnsupportedKeyParams.into());
      }

      let pk: [u8; crypto::signatures::ed25519::PublicKey::LENGTH] = crate::jwu::decode_b64(params.x.as_str())
        .map_err(|_| {
          SignatureVerificationError::new(SignatureVerificationErrorKind::KeyDecodingFailure)
            .with_custom_message("could not decode x parameter from jwk")
        })
        .and_then(|value| {
          TryInto::try_into(value).map_err(|_| {
            SignatureVerificationError::new(SignatureVerificationErrorKind::KeyDecodingFailure)
              .with_custom_message("invalid public key length")
          })
        })?;

      let public_key_ed25519 = crypto::signatures::ed25519::PublicKey::try_from(pk).map_err(|err| {
        SignatureVerificationError::new(SignatureVerificationErrorKind::KeyDecodingFailure).with_source(err)
      })?;

      let signature_arr =
        <[u8; crypto::signatures::ed25519::Signature::LENGTH]>::try_from(input.decoded_signature.deref())
          .map_err(|_| SignatureVerificationErrorKind::InvalidSignature)?;

      let signature = crypto::signatures::ed25519::Signature::from_bytes(signature_arr);

      if crypto::signatures::ed25519::PublicKey::verify(&public_key_ed25519, &signature, &input.signing_input) {
        Ok(())
      } else {
        Err(SignatureVerificationErrorKind::InvalidSignature.into())
      }
    }
  }

  impl Default for EdDSAJwsVerifier {
    /// Constructs a [`EdDSAJwsVerifier`]. This is the only way to obtain an [`EdDSAJwsVerifier`] and
    /// is only available when the `eddsa` feature is set.
    fn default() -> Self {
      Self
    }
  }

  impl JwsVerifier for EdDSAJwsVerifier {
    /// This implements verification of jws signatures signed with the `EdDSA` algorithm. For now
    /// [`EdDSAJwsVerifier::verify`](EdDSAJwsVerifier::verify) can only handle the `alg = EdDSA` with
    /// `crv = Ed25519`, but the implementation may also support `crv = Ed448` in the future.
    fn verify(
      &self,
      input: VerificationInput,
      public_key: &Jwk,
    ) -> std::result::Result<(), SignatureVerificationError> {
      match input.alg {
        // EdDSA is the only supported algorithm for now, we can consider supporting more by default in the future.
        JwsAlgorithm::EdDSA => EdDSAJwsVerifier::verify_eddsa(input, public_key),
        _ => Err(SignatureVerificationErrorKind::UnsupportedAlg.into()),
      }
    }
  }
}
