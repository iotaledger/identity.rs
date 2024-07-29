use identity_jose::jwk::Jwk;
use identity_jose::jws::JwsAlgorithm;
use identity_jose::jws::JwsVerifier;
use identity_jose::jws::SignatureVerificationError;
use identity_jose::jws::SignatureVerificationErrorKind;
use identity_jose::jws::VerificationInput;
use oqs::sig::Algorithm;

use crate::OQSVerifier;

/// An implementor of [`JwsVerifier`] that can handle the
/// [`JwsAlgorithm::ML_DSA_44`](identity_jose::jws::JwsAlgorithm::ML_DSA_44)
/// | [`JwsAlgorithm::ML_DSA_65`](identity_jose::jws::JwsAlgorithm::ML_DSA_65)
/// | [`JwsAlgorithm::ML_DSA_87`](identity_jose::jws::JwsAlgorithm::ML_DSA_87) algorithms.
#[derive(Debug)]
#[non_exhaustive]
pub struct PQCJwsVerifier;

impl Default for PQCJwsVerifier {
  /// Constructs an [`MLDSAJwsVerifier`]. This is the only way to obtain an [`MLDSAJwsVerifier`].
  fn default() -> Self {
    Self
  }
}

impl JwsVerifier for PQCJwsVerifier {
  /// This implements verification of JWS signatures signed with the
  /// [`JwsAlgorithm::ML_DSA_44`](identity_jose::jws::JwsAlgorithm::ML_DSA_44)
  /// | [`JwsAlgorithm::ML_DSA_65`](identity_jose::jws::JwsAlgorithm::ML_DSA_65)
  /// | [`JwsAlgorithm::ML_DSA_87`](identity_jose::jws::JwsAlgorithm::ML_DSA_87)
  /// | [`JwsAlgorithm::SLH_DSA_SHA2_128s`](identity_jose::jws::JwsAlgorithm::SLH_DSA_SHA2_128s)
  /// | [`JwsAlgorithm::SLH_DSA_SHA2_128f`](identity_jose::jws::JwsAlgorithm::SLH_DSA_SHA2_128f)
  /// | [`JwsAlgorithm::SLH_DSA_SHAKE_128s`](identity_jose::jws::JwsAlgorithm::SLH_DSA_SHAKE_128s) algorithms.
  // Allow unused variables in case of no-default-features.
  #[allow(unused_variables)]
  fn verify(&self, input: VerificationInput, public_key: &Jwk) -> std::result::Result<(), SignatureVerificationError> {
    match input.alg {
      #[cfg(feature = "ML_DSA_44")]
      JwsAlgorithm::ML_DSA_44 => OQSVerifier::verify(input, public_key, Algorithm::Dilithium2),
      #[cfg(feature = "ML_DSA_65")]
      JwsAlgorithm::ML_DSA_65 => OQSVerifier::verify(input, public_key, Algorithm::Dilithium3),
      #[cfg(feature = "ML_DSA_87")]
      JwsAlgorithm::ML_DSA_87 => OQSVerifier::verify(input, public_key, Algorithm::Dilithium5),

      #[cfg(feature = "SLH_DSA_SHA2_128s")]
      JwsAlgorithm::SLH_DSA_SHA2_128s => OQSVerifier::verify(input, public_key, Algorithm::SphincsSha2128sSimple),
      #[cfg(feature = "SLH_DSA_SHAKE_128s")]
      JwsAlgorithm::SLH_DSA_SHAKE_128s => OQSVerifier::verify(input, public_key, Algorithm::SphincsShake128sSimple),
      #[cfg(feature = "SLH_DSA_SHA2_128f")]
      JwsAlgorithm::SLH_DSA_SHA2_128f => OQSVerifier::verify(input, public_key, Algorithm::SphincsSha2128fSimple),

      #[cfg(feature = "SLH_DSA_SHAKE_128f")]
      JwsAlgorithm::SLH_DSA_SHAKE_128f => OQSVerifier::verify(input, public_key, Algorithm::SphincsShake128fSimple),
      #[cfg(feature = "SLH_DSA_SHA2_192s")]
      JwsAlgorithm::SLH_DSA_SHA2_192s => OQSVerifier::verify(input, public_key, Algorithm::SphincsSha2192sSimple),
      #[cfg(feature = "SLH_DSA_SHAKE_192s")]
      JwsAlgorithm::SLH_DSA_SHAKE_192s => OQSVerifier::verify(input, public_key, Algorithm::SphincsShake192sSimple),
      #[cfg(feature = "SLH_DSA_SHA2_192f")]
      JwsAlgorithm::SLH_DSA_SHA2_192f => OQSVerifier::verify(input, public_key, Algorithm::SphincsSha2192fSimple),
      #[cfg(feature = "SLH_DSA_SHAKE_192f")]
      JwsAlgorithm::SLH_DSA_SHAKE_192f => OQSVerifier::verify(input, public_key, Algorithm::SphincsShake192fSimple),
      #[cfg(feature = "SLH_DSA_SHA2_256s")]
      JwsAlgorithm::SLH_DSA_SHA2_256s => OQSVerifier::verify(input, public_key, Algorithm::SphincsSha2256sSimple),
      #[cfg(feature = "SLH_DSA_SHAKE_256s")]
      JwsAlgorithm::SLH_DSA_SHAKE_256s => OQSVerifier::verify(input, public_key, Algorithm::SphincsShake256sSimple),
      #[cfg(feature = "SLH_DSA_SHA2_256f")]
      JwsAlgorithm::SLH_DSA_SHA2_256f => OQSVerifier::verify(input, public_key, Algorithm::SphincsSha2256fSimple),
      #[cfg(feature = "SLH_DSA_SHAKE_256f")]
      JwsAlgorithm::SLH_DSA_SHAKE_256f => OQSVerifier::verify(input, public_key, Algorithm::SphincsShake256fSimple),

      #[cfg(feature = "FALCON512")]
      JwsAlgorithm::FALCON512 => OQSVerifier::verify(input, public_key, Algorithm::Falcon512),
      #[cfg(feature = "FALCON1024")]
      JwsAlgorithm::FALCON1024 => OQSVerifier::verify(input, public_key, Algorithm::Falcon1024),
      _ => Err(SignatureVerificationErrorKind::UnsupportedAlg.into()),
    }
  }
}
