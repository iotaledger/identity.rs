// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519::PublicKey;
use crypto::signatures::ed25519::SecretKey;

use crate::jwk::Jwk;
use crate::jws::Decoder;
use crate::jws::Encoder;
use crate::jws::JwsHeader;
use crate::jws::Recipient;
use crate::jws::{self};
use crate::tests::ed25519;

#[tokio::test]
async fn test_rfc8037_ed25519() {
  struct TestVector {
    private_jwk: &'static str,
    public_jwk: &'static str,
    thumbprint_b64: &'static str,
    header: &'static str,
    payload: &'static str,
    encoded: &'static str,
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc8037_ed25519.rs");

  for tv in TVS {
    let secret: Jwk = serde_json::from_str(tv.private_jwk).unwrap();
    let public: Jwk = serde_json::from_str(tv.public_jwk).unwrap();

    assert_eq!(secret.thumbprint_b64().unwrap(), tv.thumbprint_b64);
    assert_eq!(public.thumbprint_b64().unwrap(), tv.thumbprint_b64);

    let header: JwsHeader = serde_json::from_str(tv.header).unwrap();
    let encoder: Encoder = Encoder::new().recipient(Recipient::new().protected(&header));

    let secret_key: SecretKey = ed25519::expand_secret_jwk(&secret);
    let public_key: PublicKey = ed25519::expand_public_jwk(&public);
    let encoded: String = ed25519::encode(&encoder, tv.payload.as_bytes(), secret_key).await;

    assert_eq!(encoded, tv.encoded);

    let decoder: Decoder = Decoder::new();
    let decoded: jws::Token = ed25519::decode(&decoder, encoded.as_bytes(), public_key);

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.payload.as_bytes());
  }
}
