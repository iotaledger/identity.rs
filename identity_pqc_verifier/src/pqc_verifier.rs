
use identity_jose::jwk::Jwk;
use identity_jose::jws::JwsVerifier;
use identity_jose::jws::SignatureVerificationError;
use identity_jose::jws::SignatureVerificationErrorKind;
use identity_jose::jws::VerificationInput;
use oqs::sig::Algorithm;

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
      identity_jose::jws::JwsAlgorithm::ML_DSA_44 => crate::OQSVerifier::verify(input, public_key, Algorithm::Dilithium2),
      #[cfg(feature = "ML_DSA_65")]
      identity_jose::jws::JwsAlgorithm::ML_DSA_65 => crate::OQSVerifier::verify(input, public_key, Algorithm::Dilithium3),
      #[cfg(feature = "ML_DSA_87")]
      identity_jose::jws::JwsAlgorithm::ML_DSA_87 => crate::OQSVerifier::verify(input, public_key, Algorithm::Dilithium5),

      #[cfg(feature = "SLH_DSA_SHA2_128s")]
      identity_jose::jws::JwsAlgorithm::SLH_DSA_SHA2_128s => crate::OQSVerifier::verify(input, public_key, Algorithm::SphincsSha2128sSimple),
      #[cfg(feature = "SLH_DSA_SHA2_128f")]
      identity_jose::jws::JwsAlgorithm::SLH_DSA_SHA2_128f => crate::OQSVerifier::verify(input, public_key, Algorithm::SphincsSha2128fSimple),
      #[cfg(feature = "SLH_DSA_SHAKE_128s")]
      identity_jose::jws::JwsAlgorithm::SLH_DSA_SHAKE_128s => crate::OQSVerifier::verify(input, public_key, Algorithm::SphincsShake128sSimple),
      _ => Err(SignatureVerificationErrorKind::UnsupportedAlg.into()),
    }
  }
}