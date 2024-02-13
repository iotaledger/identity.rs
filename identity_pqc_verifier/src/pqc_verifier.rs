
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
  /// | [`JwsAlgorithm::ML_DSA_87`](identity_jose::jws::JwsAlgorithm::ML_DSA_87) algorithms.
  // Allow unused variables in case of no-default-features.
  #[allow(unused_variables)]
  fn verify(&self, input: VerificationInput, public_key: &Jwk) -> std::result::Result<(), SignatureVerificationError> {
    match input.alg {
      #[cfg(feature = "ml_dsa_44")]
      identity_jose::jws::JwsAlgorithm::ML_DSA_44 => crate::OQSVerifier::verify(input, public_key, Algorithm::Dilithium2),
      #[cfg(feature = "ml_dsa_65")]
      identity_jose::jws::JwsAlgorithm::ML_DSA_65 => crate::OQSVerifier::verify(input, public_key, Algorithm::Dilithium3),
      #[cfg(feature = "ml_dsa_87")]
      identity_jose::jws::JwsAlgorithm::ML_DSA_87 => crate::OQSVerifier::verify(input, public_key, Algorithm::Dilithium5),
      _ => Err(SignatureVerificationErrorKind::UnsupportedAlg.into()),
    }
  }
}