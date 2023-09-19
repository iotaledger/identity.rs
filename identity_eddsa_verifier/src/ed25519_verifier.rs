// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use identity_jose::jwk::EdCurve;
use identity_jose::jwk::Jwk;
use identity_jose::jwk::JwkParamsOkp;
use identity_jose::jws::SignatureVerificationError;
use identity_jose::jws::SignatureVerificationErrorKind;
use identity_jose::jws::VerificationInput;

/// A verifier that can handle the [`JwsAlgorithm::EdDSA`](identity_jose::jws::JwsAlgorithm::EdDSA) algorithm with curve
/// [`EdCurve::Ed25519`].
#[derive(Debug)]
#[non_exhaustive]
pub struct Ed25519Verifier;

impl Ed25519Verifier {
  /// Verify a JWS signature secured with the [`JwsAlgorithm::EdDSA`](identity_jose::jws::JwsAlgorithm::EdDSA)
  /// algorithm and curve [`EdCurve::Ed25519`]. This associated method is only available when the
  /// `ed25519` feature is enabled.
  ///
  /// This function is useful when one is composing a [`JwsVerifier`](identity_jose::jws::JwsVerifier) that delegates
  /// [`JwsAlgorithm::EdDSA`](identity_jose::jws::JwsAlgorithm::EdDSA) verification with
  /// curve [`EdCurve::Ed25519`] to this function.
  ///
  /// # Warning
  ///
  /// This function does not check whether `alg = EdDSA` in the protected header. Callers are expected to assert this
  /// prior to calling the function.
  pub fn verify(input: VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
    // Obtain an Ed25519 public key.
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

    let pk: [u8; crypto::signatures::ed25519::PublicKey::LENGTH] = identity_jose::jwu::decode_b64(params.x.as_str())
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
