// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;
use crate::jws::Decoder;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::JwsVerifierFn;
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

    let verifier = JwsVerifierFn::from(|input: VerificationInput, key: &Jwk| {
      if input.alg != JwsAlgorithm::HS256 {
        panic!("unsupported algorithm");
      }
      hs256::verify(input, key)
    });
    let decoder = Decoder::new();

    let token = decoder
      .decode_compact_serialization(tv.encoded, tv.detach.then_some(tv.payload))
      .and_then(|decoded| decoded.verify(&verifier, &jwk))
      .unwrap();

    assert_eq!(token.protected, header);
    assert_eq!(token.claims, tv.payload);
  }
}
