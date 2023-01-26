// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_jose::jwk::Jwk;

#[test]
fn test_rfc7638() {
  struct TestVector {
    jwk_json: &'static str,
    thumbprint_b64: &'static str,
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc7638.rs");

  for tv in TVS {
    let key: Jwk = serde_json::from_str(tv.jwk_json).unwrap();
    let kid: String = key.thumbprint_b64().unwrap();

    assert_eq!(kid, tv.thumbprint_b64);
  }
}