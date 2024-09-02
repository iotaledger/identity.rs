use identity_ecdsa_verifier::EcDSAJwsVerifier;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::verification::{
  jwk::Jwk,
  jws::{JwsAlgorithm, JwsVerifier, SignatureVerificationError, SignatureVerificationErrorKind, VerificationInput},
};

#[derive(Debug, Default)]
pub struct Verifier {
  eddsa: EdDSAJwsVerifier,
  ecdsa: EcDSAJwsVerifier,
}

impl Verifier {
    pub fn new() -> Self {
        Self::default()
    }
}

impl JwsVerifier for Verifier {
  fn verify(&self, input: VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
    match input.alg {
      JwsAlgorithm::EdDSA => self.eddsa.verify(input, public_key),
      JwsAlgorithm::ES256 | JwsAlgorithm::ES256K => self.ecdsa.verify(input, public_key),
      _ => Err(SignatureVerificationError::new(
        SignatureVerificationErrorKind::UnsupportedAlg,
      )),
    }
  }
}
