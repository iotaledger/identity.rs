// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::tests::es256;
use crate::tests::hs256;

struct TestVector {
  deterministic: bool,
  header: &'static str,
  claims: &'static [u8],
  encoded: &'static [u8],
  private_key: &'static str,
}

#[tokio::test]
async fn test_rfc7515() {
  static TVS: &[TestVector] = &include!("fixtures/rfc7515.rs");

  for tv in TVS {
    let header: JwsHeader = serde_json::from_str(tv.header).unwrap();
    let jwk: Jwk = serde_json::from_str(tv.private_key).unwrap();

    if tv.deterministic {
      let encoded: String = match header.alg() {
        JwsAlgorithm::HS256 => hs256::encode(tv.claims, &header, &jwk).await,
        JwsAlgorithm::ES256 => es256::encode(tv.claims, &header, &jwk).await,
        other => unimplemented!("{other}"),
      };

      assert_eq!(encoded.as_bytes(), tv.encoded);
    }

    let decoded: _ = match header.alg() {
      JwsAlgorithm::HS256 => hs256::decode(tv.encoded, &jwk),
      JwsAlgorithm::ES256 => es256::decode(tv.encoded, &jwk),
      other => unimplemented!("{other}"),
    };

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.claims);
  }
}
