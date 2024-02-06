// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;
use crypto::signatures::ed25519::PublicKey;
use crypto::signatures::ed25519::SecretKey;
use crypto::signatures::ed25519::Signature;
use identity_jose::jwk::EdCurve;
use identity_jose::jwk::Jwk;
use identity_jose::jwk::JwkParamsOkp;
use identity_jose::jwk::JwkType;
use identity_jose::jws::CompactJwsEncoder;
use identity_jose::jws::Decoder;
use identity_jose::jws::JwsAlgorithm;
use identity_jose::jws::JwsHeader;
use identity_jose::jws::JwsVerifierFn;
use identity_jose::jws::SignatureVerificationError;
use identity_jose::jws::SignatureVerificationErrorKind;
use identity_jose::jws::VerificationInput;
use identity_jose::jwt::JwtClaims;
use identity_jose::jwu;
use std::ops::Deref;
use std::time::SystemTime;

/// Example code demonstrating how to use the [`Encoder`] and [`Decoder`] APIs.
fn encode_then_decode() -> Result<JwtClaims<serde_json::Value>, Box<dyn std::error::Error>> {
  // =============================
  // Generate an Ed25519 key pair
  // =============================

  let secret_key = SecretKey::generate()?;
  let public_key = secret_key.public_key();

  // ====================================
  // Create the header for the recipient
  // ====================================

  let mut header: JwsHeader = JwsHeader::new();
  header.set_alg(JwsAlgorithm::EdDSA);
  let kid = "did:iota:0x123#signing-key";
  header.set_kid(kid);

  // ==================================
  // Create the claims we want to sign
  // ==================================

  let mut claims: JwtClaims<serde_json::Value> = JwtClaims::new();
  claims.set_iss("issuer");
  claims.set_iat(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as i64);
  claims.set_custom(serde_json::json!({"num": 42u64}));

  // ==================
  // Encode the claims
  // ==================

  let claims_bytes: Vec<u8> = serde_json::to_vec(&claims)?;
  let encoder: CompactJwsEncoder<'_> = CompactJwsEncoder::new(&claims_bytes, &header)?;
  // Get the signing input from the encoder
  let signing_input: &[u8] = encoder.signing_input();
  // sign the signing input with secret_key
  let signature: [u8; 64] = secret_key.sign(signing_input).to_bytes();
  // hand the signature over to the encoder so we can complete the JWS creation process
  let token: String = encoder.into_jws(&signature);

  // ===================================
  // Create public key for verification
  // ===================================

  let mut public_key_jwk = Jwk::new(JwkType::Okp);
  public_key_jwk.set_kid(kid);
  public_key_jwk
    .set_params(JwkParamsOkp {
      crv: EdCurve::Ed25519.name().to_owned(),
      x: identity_jose::jwu::encode_b64(public_key.as_slice()),
      d: None,
    })
    .unwrap();
  public_key_jwk.set_alg(JwsAlgorithm::EdDSA.to_string());

  // ==================
  // Decode the claims
  // ==================

  // Set up a verifier that verifies JWS signatures secured with the Ed25519 algorithm
  let verify_fn = JwsVerifierFn::from(
    |verification_input: VerificationInput, jwk: &Jwk| -> Result<(), SignatureVerificationError> {
      if verification_input.alg != JwsAlgorithm::EdDSA {
        return Err(SignatureVerificationErrorKind::UnsupportedAlg.into());
      }

      let params: &JwkParamsOkp = jwk
        .try_okp_params()
        .map_err(|_| SignatureVerificationErrorKind::UnsupportedKeyType)?;

      if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
        return Err(SignatureVerificationErrorKind::UnsupportedKeyParams.into());
      }

      let pk: [u8; PublicKey::LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();

      let public_key = PublicKey::try_from(pk).map_err(|_| SignatureVerificationErrorKind::KeyDecodingFailure)?;
      let signature_arr =
        <[u8; Signature::LENGTH]>::try_from(verification_input.decoded_signature.deref()).map_err(|err| {
          SignatureVerificationError::new(SignatureVerificationErrorKind::InvalidSignature).with_source(err)
        })?;
      let signature = ed25519::Signature::from_bytes(signature_arr);
      if public_key.verify(&signature, &verification_input.signing_input) {
        Ok(())
      } else {
        Err(SignatureVerificationErrorKind::InvalidSignature.into())
      }
    },
  );

  let decoder = Decoder::new();
  // We don't use a detached payload.
  let detached_payload: Option<&[u8]> = None;

  // Decode the encoded token and verify the result to get a cryptographically verified `Token`.
  let token = decoder
    .decode_compact_serialization(token.as_bytes(), detached_payload)
    .and_then(|decoded| decoded.verify(&verify_fn, &public_key_jwk))?;

  // ==================================
  // Assert the claims are as expected
  // ==================================

  let recovered_claims: JwtClaims<serde_json::Value> = serde_json::from_slice(&token.claims)?;
  assert_eq!(claims, recovered_claims);

  Ok(recovered_claims)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let recovered_claims = encode_then_decode()?;
  println!("{recovered_claims:#?}");
  Ok(())
}

#[test]
fn test_example_code() {
  assert!(encode_then_decode().is_ok());
}
