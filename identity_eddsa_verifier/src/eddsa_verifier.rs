// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_jose::jwk::Jwk;
use identity_jose::jws::JwsVerifier;
use identity_jose::jws::SignatureVerificationError;
use identity_jose::jws::SignatureVerificationErrorKind;
use identity_jose::jws::VerificationInput;

/// An implementor of [`JwsVerifier`] that can handle the
/// [`JwsAlgorithm::EdDSA`](identity_jose::jws::JwsAlgorithm::EdDSA) algorithm.
#[derive(Debug)]
#[non_exhaustive]
pub struct EdDSAJwsVerifier;

impl Default for EdDSAJwsVerifier {
  /// Constructs an [`EdDSAJwsVerifier`]. This is the only way to obtain an [`EdDSAJwsVerifier`].
  fn default() -> Self {
    Self
  }
}

impl JwsVerifier for EdDSAJwsVerifier {
  /// This implements verification of JWS signatures signed with the
  /// [`JwsAlgorithm::EdDSA`](identity_jose::jws::JwsAlgorithm::EdDSA) algorithm.
  // Allow unused variables in case of no-default-features.
  #[allow(unused_variables)]
  fn verify(&self, input: VerificationInput, public_key: &Jwk) -> std::result::Result<(), SignatureVerificationError> {
    match input.alg {
      #[cfg(feature = "ed25519")]
      identity_jose::jws::JwsAlgorithm::EdDSA => crate::Ed25519Verifier::verify(input, public_key),
      _ => Err(SignatureVerificationErrorKind::UnsupportedAlg.into()),
    }
  }
}
