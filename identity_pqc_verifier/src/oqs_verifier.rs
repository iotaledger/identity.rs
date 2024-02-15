
use std::ops::Deref;
use identity_jose::jwk::Jwk;
use identity_jose::jwk::JwkParamsPQ;
use identity_jose::jws::SignatureVerificationError;
use identity_jose::jws::SignatureVerificationErrorKind;
use identity_jose::jws::VerificationInput;
use oqs::sig::Algorithm;
use oqs::sig::Sig;

/// A verifier that can handle the [`Algorithm`] PQC algorithms.
#[derive(Debug)]
#[non_exhaustive]
pub struct OQSVerifier;

impl OQSVerifier {

  /// Verify a JWS signature secured with the on the [`Algorithm`] defined in liboqs.
  pub fn verify(input: VerificationInput, public_key: &Jwk, alg: Algorithm) -> Result<(), SignatureVerificationError> {

    // Obtain an ML-DSA-44 public key.
    let params: &JwkParamsPQ = public_key
      .try_pq_params()
      .map_err(|_| SignatureVerificationErrorKind::UnsupportedKeyType)?;


    let pk = 
    identity_jose::jwu::decode_b64(params.public.as_str())
      .map_err(|_| {
        SignatureVerificationError::new(SignatureVerificationErrorKind::KeyDecodingFailure)
          .with_custom_message("could not decode 'pub' parameter from jwk")
    })?;


    oqs::init(); //TODO: check what this function does

    let scheme = Sig::new(alg)
    .map_err(|_| {
        SignatureVerificationError::new(SignatureVerificationErrorKind::Unspecified)
        .with_custom_message("signature scheme init failed")
    })?;

    let public_key = scheme.public_key_from_bytes(&pk).ok_or(
            SignatureVerificationError::new(SignatureVerificationErrorKind::KeyDecodingFailure)
    )?;

    let signature = scheme.signature_from_bytes(input.decoded_signature.deref())
        .ok_or(SignatureVerificationErrorKind::InvalidSignature)?;

    Ok(scheme.verify(&input.signing_input, signature, public_key)
        .map_err(|_| SignatureVerificationErrorKind::InvalidSignature)?)

  }
}