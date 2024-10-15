// Copyright 2020-2024 IOTA Stiftung, Filancore GmbH
// SPDX-License-Identifier: Apache-2.0

use identity_verification::jwk::Jwk;
use identity_verification::jws::SignatureVerificationError;
use identity_verification::jws::VerificationInput;
use k256::ecdsa::VerifyingKey;
use signature::hazmat::PrehashVerifier;
use signature::Verifier;

use crate::common;

/// A verifier that can handle the
/// [`JwsAlgorithm::ES256K`](identity_verification::jws::JwsAlgorithm::ES256K)
/// algorithm.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Secp256K1Verifier {}

impl Secp256K1Verifier {
  /// Verify a JWS signature secured with the
  /// [`JwsAlgorithm::ES256K`](identity_verification::jws::JwsAlgorithm::ES256K)
  /// algorithm.
  ///
  /// This function is useful when one is building a
  /// [`JwsVerifier`](identity_verification::jws::JwsVerifier) that
  /// handles the
  /// [`JwsAlgorithm::ES256K`](identity_verification::jws::JwsAlgorithm::ES256K)
  /// in the same manner as the [`Secp256K1Verifier`] hence extending its
  /// capabilities.
  ///
  /// # Warning
  ///
  /// This function does not check whether `alg = ES256K` is in the protected
  /// header. Callers are expected to assert this prior to calling the
  /// function.
  pub fn verify(input: &VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
    common::verify_signature(input, public_key, VerifyingKey::verify)
  }

  /// Pre-hashed variant of [`Secp256K1Verifier::verify`].
  /// # Warning
  /// `input.signing_input` **MUST** be the result of a cryptographically secure hashing algorithm.
  pub fn verify_prehashed(input: &VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
    common::verify_signature(input, public_key, VerifyingKey::verify_prehash)
  }
}
