// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;
use crate::jws::JWSValidationConfig;
use crate::jws::JwsHeader;
use crate::jws::{self};
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

    let verifier = JwsSignatureVerifierFn::from(|input, key| {
      if input.alg().filter(|value| value == JwsAlgorithm::HS256).is_none() {
        panic!("unsupported algorithm"); 
      }
      hs256::verify(input, key)
    });
    let decoder = Decoder::new(verifier).config(JWSValidationConfig::default().critical("b64"));  

    let decoded = decoder.decode(
      tv.encoded,
      || Some(&jwk), 
      tv.detach.then_some(tv.payload),
    ).unwrap();

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.payload);
  }
}
