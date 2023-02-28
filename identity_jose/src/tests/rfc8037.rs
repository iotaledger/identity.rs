// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519::SecretKey;

use crate::jwk::Jwk;
use crate::jws::Decoder;
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

    let jws_verifier = JwsSignatureVerifierFn::from(|input: &VerificationInput, key: &Jwk| {
      if input
        .jose_header()
        .alg()
        .filter(|value| *value == JwsAlgorithm::EdDSA)
        .is_none()
      {
        panic!("invalid algorithm");
      }
      ed25519::verify(input, key)
    });
    let decoder = Decoder::new().jwk_must_have_alg(false);
    let decoded = decoder
      .decode(encoded.as_bytes(), &jws_verifier, |_| Some(&public), None)
      .unwrap();

    #[cfg(feature = "default-jws-signature-verifier")]
    {
      let decoder = Decoder::default().jwk_must_have_alg(false);
      let decoded_with_default = decoder
        .decode_default(encoded.as_bytes(), |_| Some(&public), None)
        .unwrap();
      assert_eq!(decoded, decoded_with_default);
    }
    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.payload.as_bytes());
  }
}
