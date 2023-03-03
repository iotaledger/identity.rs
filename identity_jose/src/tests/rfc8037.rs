// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519::SecretKey;

use crate::jwk::Jwk;
use crate::jws::Decoder;
#[cfg(feature = "eddsa")]
use crate::jws::EdDSAJwsSignatureVerifier;
use crate::jws::Encoder;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::JwsSignatureVerifierFn;
use crate::jws::Recipient;
use crate::jws::VerificationInput;
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
    let encoded: String = ed25519::encode(&encoder, tv.payload.as_bytes(), secret_key).await;

    assert_eq!(encoded, tv.encoded);

    let jws_verifier = JwsSignatureVerifierFn::from(|input: VerificationInput, key: &Jwk| {
      if input.alg != JwsAlgorithm::EdDSA {
        panic!("invalid algorithm");
      }
      ed25519::verify(input, key)
    });
    let decoder = Decoder::new();
    let token = decoder
      .decode_compact_serialization(encoded.as_bytes(), None)
      .and_then(|decoded| decoded.verify(&jws_verifier, &public))
      .unwrap();

    #[cfg(feature = "eddsa")]
    {
      let decoder = Decoder::default();
      let token_with_default = decoder
        .decode_compact_serialization(encoded.as_bytes(), None)
        .and_then(|decoded| decoded.verify(&EdDSAJwsSignatureVerifier::default(), &public))
        .unwrap();
      assert_eq!(token, token_with_default);
    }
    assert_eq!(token.protected, header);
    assert_eq!(token.claims, tv.payload.as_bytes());
  }
}
