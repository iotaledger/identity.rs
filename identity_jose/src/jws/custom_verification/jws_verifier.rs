// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;

#[cfg(feature = "eddsa")]
use crate::jwk::EdCurve;
#[cfg(feature = "eddsa")]
use crate::jwk::JwkParamsOkp;
#[cfg(feature = "default-jws-signature-verifier")]
use crate::jws::JwsAlgorithm;

use super::SignatureVerificationError;
use super::SignatureVerificationErrorKind;

/// Input a [`JwsSignatureVerifier`] verifies.
pub struct VerificationInput<'a> {
  /// The `alg` parsed from the protected header.
  pub alg: JwsAlgorithm,
  /// The signing input.
  ///
  /// See [RFC 7515: section 5.2 part 8.](https://www.rfc-editor.org/rfc/rfc7515#section-5.2) and
  /// [RFC 7797 section 3](https://www.rfc-editor.org/rfc/rfc7797#section-3).
  pub signing_input: &'a [u8],
  /// The decoded signature to validate the `signing_input` against in the manner defined by the `alg` field.
  pub decoded_signature: &'a [u8],
}

/// Trait for cryptographically verifying a JWS signature.
///
/// Any type implementing this trait can be passed to
/// [`JwsValidationItem::verify`](`crate::jws::JwsValidationItem::verify`) which is intended as the most convenient way
/// to verify a decoded JWS.
///
/// [`JwsValidationItem::verify`](crate::jws::JwsValidationItem::verify)
///
/// ## Implementation
/// Implementors are expected to provide a procedure for step 8 of [RFC 7515 section 5.2](https://www.rfc-editor.org/rfc/rfc7515#section-5.2) for
/// the JWS signature algorithms they want to support.
///
/// Custom implementations can be constructed inline by converting a suitable closure into a [`JwsSignatureVerifierFn`]
/// using the [`From`] trait.
///
/// ## Default implementation
/// When the `default-jws-signature-verifier` feature is enabled one can construct a default implementor
/// provided by the IOTA Identity library. See
/// [`DefaultJwsSignatureVerifier::verify`](DefaultJwsSignatureVerifier::verify).
pub trait JwsSignatureVerifier {
  /// Validate the `decoded_signature` against the `signing_input` in the manner defined by `alg` using the
  /// `public_key`.
  ///
  /// Implementors may decide to error with
  /// [`SignatureVerificationErrorKind::UnsupportedAlg`](SignatureVerificationErrorKind::UnsupportedAlg) if they
  /// are not interested in supporting a given algorithm.
  fn verify(&self, input: VerificationInput<'_>, public_key: &Jwk) -> Result<(), SignatureVerificationError>;
}

// =================================================================================================================
// Implementation
// ================================================================================================================
/// Simple wrapper around a closure capable of verifying a JWS signature. This wrapper implements
/// [`JwsSignatureVerifier`].
///
/// Note: One can convert a closure to this wrapper using the [`From`] trait.
pub struct JwsSignatureVerifierFn<F>(F);
impl<F> From<F> for JwsSignatureVerifierFn<F>
where
  for<'a> F: Fn(VerificationInput<'a>, &Jwk) -> Result<(), SignatureVerificationError>,
{
  fn from(value: F) -> Self {
    Self(value)
  }
}

impl<F> JwsSignatureVerifier for JwsSignatureVerifierFn<F>
where
  for<'a> F: Fn(VerificationInput<'a>, &Jwk) -> Result<(), SignatureVerificationError>,
{
  fn verify(&self, input: VerificationInput<'_>, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
    self.0(input, public_key)
  }
}

/// The default implementor of [`JwsSignatureVerifier`] provided by the IOTA Identity library.
///
/// NOTE: This type can only be constructed when the `default-jws-signature-verifier` feature is enabled.
pub struct DefaultJwsSignatureVerifier {
  _internal: (),
}

impl DefaultJwsSignatureVerifier {
  /// Verify a JWS signature secured with the [`JwsAlgorithm::EdDSA`](crate::jws::JwsAlgorithm::EdDSA) algorithm.
  /// Only the [`EdCurve::Ed25519`] variant is supported for now. This associated method is only available when the
  /// `eddsa` feature is enabled.
  ///
  /// This function is useful when one is building a [`JwsSignatureVerifier`] that handles the
  /// [`JwsAlgoritm::EdDSA`](crate::jws::JwsAlgorithm::EdDSA) in the same manner as the [`DefaultJwsSignatureVerifier`]
  /// hence extending its capabilities.
  #[cfg(any(feature = "eddsa", doc))]
  pub fn verify_eddsa_jws_prechecked_alg(
    input: VerificationInput<'_>,
    public_key: &Jwk,
  ) -> Result<(), SignatureVerificationError> {
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

    let pk: [u8; crypto::signatures::ed25519::PUBLIC_KEY_LENGTH] = crate::jwu::decode_b64(params.x.as_str())
      .map_err(|_| SignatureVerificationErrorKind::KeyDecodingFailure)
      .and_then(|value| TryInto::try_into(value).map_err(|_| SignatureVerificationErrorKind::KeyDecodingFailure))?;

    let public_key_ed25519 = crypto::signatures::ed25519::PublicKey::try_from(pk)
      .map_err(|_| SignatureVerificationErrorKind::KeyDecodingFailure)?;

    let signature_arr = <[u8; crypto::signatures::ed25519::SIGNATURE_LENGTH]>::try_from(input.decoded_signature)
      .map_err(|_| SignatureVerificationErrorKind::InvalidSignature)?;

    let signature = crypto::signatures::ed25519::Signature::from_bytes(signature_arr);

    if crypto::signatures::ed25519::PublicKey::verify(&public_key_ed25519, &signature, input.signing_input) {
      Ok(())
    } else {
      Err(SignatureVerificationErrorKind::InvalidSignature.into())
    }
  }
}

#[cfg(any(feature = "default-jws-signature-verifier", doc))]
impl Default for DefaultJwsSignatureVerifier {
  /// Constructs a [`DefaultJwsSignatureVerifier`]. This is the only way to obtain a `DefaultJwsVerifier` and is only
  /// available when the `default-jws-signature-verifier` feature is set.
  fn default() -> Self {
    Self { _internal: () }
  }
}

impl JwsSignatureVerifier for DefaultJwsSignatureVerifier {
  /// Default implementer of [`JwsSignatureVerifier`]. For now `DefaultJwsVerifier::verify` can only handle the `alg =
  /// EdDSA` with `crv = Ed25519`, but the implementation may support more algorithms in the future.
  #[cfg(feature = "default-jws-signature-verifier")]
  fn verify(
    &self,
    input: VerificationInput<'_>,
    public_key: &Jwk,
  ) -> std::result::Result<(), SignatureVerificationError> {
    match input.alg {
      // EdDSA is the only supported algorithm for now, we can consider supporting more by default in the future.
      JwsAlgorithm::EdDSA => DefaultJwsSignatureVerifier::verify_eddsa_jws_prechecked_alg(input, public_key),
      _ => Err(SignatureVerificationErrorKind::UnsupportedAlg.into()),
    }
  }

  // This method can never be called because it is impossible to construct the `DefaultJwsVerifier` without enabling
  // the `default-jws-signature-verifier` feature. It is still implemented in case we need to parameterize a struct with
  // DefaultJwsSignatureVerifier as a default also in the case where the `default-jws-signature-verifier` feature is
  // disabled.
  #[cfg(not(feature = "default-jws-signature-verifier"))]
  fn verify(
    &self,
    _input: VerificationInput<'_>,
    _public_key: &Jwk,
  ) -> std::result::Result<(), SignatureVerificationError> {
    panic!("it should not be possible to construct a DefaultJwsVerifier without the 'default-jws-signature-verifier' feature. We encourage you to report this bug at: https://github.com/iotaledger/identity.rs/issues");
  }
}
