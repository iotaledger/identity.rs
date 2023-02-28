// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;
use crate::jws::Decoder;
use crate::jws::JwsDecoderConfig;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::JwsSignatureVerifierFn;
use crate::jws::VerificationInput;
use crate::tests::hs256;

#[test]
fn test_rfc7797() {
  struct TestVector {
    detach: bool,
    header: &'static [u8],
    encoded: &'static [u8],
    payload: &'static [u8],
    public_key: &'static str,
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc7797.rs");

  for tv in TVS {
    let header: JwsHeader = serde_json::from_slice(tv.header).unwrap();
    let jwk: Jwk = serde_json::from_str(tv.public_key).unwrap();

    let verifier = JwsSignatureVerifierFn::from(|input: &VerificationInput, key: &Jwk| {
      if input
        .jose_header()
        .alg()
        .filter(|value| *value == JwsAlgorithm::HS256)
        .is_none()
      {
        panic!("unsupported algorithm");
      }
      hs256::verify(input, key)
    });
    let decoder =
      Decoder::new(verifier).config(JwsDecoderConfig::default().critical("b64").jwk_must_have_alg(false));

    let decoded = decoder
      .decode(tv.encoded, |_| Some(&jwk), tv.detach.then_some(tv.payload))
      .unwrap();

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.payload);
  }
}
