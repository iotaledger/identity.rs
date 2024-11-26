// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use identity_jose::jwk::Jwk;
use identity_jose::jwk::JwkParamsPQ;
use identity_jose::jws::SignatureVerificationError;
use identity_jose::jws::SignatureVerificationErrorKind;
use identity_jose::jws::VerificationInput;
use pqcrypto::sign::mldsa44::{PublicKey, DetachedSignature, verify_detached_signature};
use pqcrypto::traits::sign::{PublicKey as PkTrait, DetachedSignature as DTSTrait};


/// A verifier that can handle the PQC algorithms.
#[derive(Debug)]
#[non_exhaustive]
pub struct PQCleanVerifier;

impl PQCleanVerifier {
  /// Verify a JWS signature secured with the on the ML-DSA-44 defined in pqcrypto.
  pub fn verify(input: VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
    
    // Obtain an ML-DSA-44 public key.
    let params: &JwkParamsPQ = public_key
      .try_pq_params()
      .map_err(|_| SignatureVerificationErrorKind::UnsupportedKeyType)?;
    let pk = identity_jose::jwu::decode_b64(params.public.as_str()).map_err(|_| {
      SignatureVerificationError::new(SignatureVerificationErrorKind::KeyDecodingFailure)
        .with_custom_message("could not decode 'pub' parameter from jwk")
    })?;

    let public_key = PublicKey::from_bytes(&pk)
    .map_err(|_| SignatureVerificationError::new(
      SignatureVerificationErrorKind::KeyDecodingFailure,
    ))?;

    let signature = DetachedSignature::from_bytes(input.decoded_signature.deref())
    .map_err(|_| SignatureVerificationErrorKind::InvalidSignature)?;

    verify_detached_signature(&signature, &input.signing_input, &public_key)
    .map_err(|_|SignatureVerificationErrorKind::InvalidSignature)?;
    
    Ok(())

}
}

#[cfg(test)]
mod tests {

}
 
