// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;
use crate::jwk::JwkSet;
use serde_json::Value;

#[test]
fn test_rfc7517() {
  enum TestVector {
    KeySet { json: &'static str },
    Key { json: &'static str },
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc7517.rs");

  for tv in TVS {
    match tv {
      TestVector::KeySet { json } => {
        let value: Value = serde_json::from_str(json).unwrap();
        let jwks: JwkSet = serde_json::from_str(json).unwrap();

        for (index, jwk) in jwks.iter().enumerate() {
          let ser: Value = serde_json::to_value(jwk).unwrap();
          assert_eq!(ser, value["keys"][index]);
        }
      }
      TestVector::Key { json } => {
        let value: Value = serde_json::from_str(json).unwrap();
        let jwk: Jwk = serde_json::from_str(json).unwrap();
        let ser: Value = serde_json::to_value(&jwk).unwrap();

        assert_eq!(ser, value);
      }
    }
  }
}
