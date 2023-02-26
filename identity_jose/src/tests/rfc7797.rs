// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;
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

    let mut decoder = jws::Decoder::new();

    let decoder = decoder.critical("b64");

    let decoded: _ = hs256::decode(&decoder, tv.encoded, tv.detach.then_some(tv.payload), &jwk);

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.payload);
  }
}
