// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::SystemTime;

use crypto::signatures::ed25519::SecretKey;

use crate::jws::Decoder;
use crate::jws::Encoder;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::Recipient;
use crate::jwt::JwtClaims;
use crate::tests::ed25519;

#[tokio::test]
async fn test_encoder_decoder_roundtrip() {
  let secret_key = SecretKey::generate().unwrap();
  let public_key = secret_key.public_key();

  let mut header: JwsHeader = JwsHeader::new(JwsAlgorithm::EdDSA);
  header.set_kid("did:iota:0x123#signing-key");

  let mut claims: JwtClaims<serde_json::Value> = JwtClaims::new();
  claims.set_iss("issuer");
  claims.set_iat(
    SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs() as i64,
  );
  claims.set_custom(serde_json::json!({"num": 42u64}));

  let encoder: Encoder = Encoder::new().recipient(Recipient::new().protected(&header));
  let claims_bytes: Vec<u8> = serde_json::to_vec(&claims).unwrap();

  let token: String = ed25519::encode(&encoder, &claims_bytes, secret_key).await;

  let decoder: Decoder = Decoder::new();
  let token: _ = ed25519::decode(&decoder, token.as_bytes(), public_key);

  let recovered_claims: JwtClaims<serde_json::Value> = serde_json::from_slice(&token.claims).unwrap();

  assert_eq!(claims, recovered_claims);
}
