// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::SystemTime;

use crypto::signatures::ed25519::SecretKey;

use crate::jwk::Jwk;
use crate::jwk::JwkParamsOkp;
use crate::jwk::JwkType;
use crate::jws::CompactJwsEncoder;
use crate::jws::Decoder;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::JwsVerifierFn;
use crate::jws::VerificationInput;
use crate::jwt::JwtClaims;
use crate::tests::ed25519;

#[test]
fn test_encoder_decoder_roundtrip() {
  let secret_key = SecretKey::generate().unwrap();
  let public_key = secret_key.public_key();

  let mut header: JwsHeader = JwsHeader::new();
  header.set_alg(JwsAlgorithm::EdDSA);
  let kid = "did:iota:0x123#signing-key";
  header.set_kid(kid);

  let mut claims: JwtClaims<serde_json::Value> = JwtClaims::new();
  claims.set_iss("issuer");
  claims.set_iat(
    SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs() as i64,
  );
  claims.set_custom(serde_json::json!({"num": 42u64}));

  let claims_bytes: Vec<u8> = serde_json::to_vec(&claims).unwrap();

  let encoder: CompactJwsEncoder<'_> = CompactJwsEncoder::new(&claims_bytes, &header).unwrap();
  let signing_input: &[u8] = encoder.signing_input();
  let signature = secret_key.sign(signing_input).to_bytes();
  let jws = encoder.into_jws(&signature);

  let verifier = JwsVerifierFn::from(|input: VerificationInput, key: &Jwk| {
    if input.alg != JwsAlgorithm::EdDSA {
      panic!("invalid algorithm");
    }
    ed25519::verify(input, key)
  });
  let decoder = Decoder::new();
  let mut public_key_jwk = Jwk::new(JwkType::Okp);
  public_key_jwk.set_kid(kid);
  public_key_jwk
    .set_params(JwkParamsOkp {
      crv: "Ed25519".into(),
      x: crate::jwu::encode_b64(public_key.as_slice()),
      d: None,
    })
    .unwrap();

  let token = decoder
    .decode_compact_serialization(jws.as_bytes(), None)
    .and_then(|decoded| decoded.verify(&verifier, &public_key_jwk))
    .unwrap();

  let recovered_claims: JwtClaims<serde_json::Value> = serde_json::from_slice(&token.claims).unwrap();

  assert_eq!(claims, recovered_claims);
}
