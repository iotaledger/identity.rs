// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::{self};
use crate::jwt::JwtHeaderSet;
use crate::tests::hs256;
use crypto::hashes::sha::SHA256_LEN;

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

    let shared_secret: Vec<u8> = hs256::expand_hmac_jwk(&jwk, SHA256_LEN);

    let verify_fn = move |protected: Option<&JwsHeader>, unprotected: Option<&JwsHeader>, msg: &[u8], sig: &[u8]| {
      let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new().protected(protected).unprotected(unprotected);
      let alg: JwsAlgorithm = header_set.try_alg().map_err(|_| "missing `alg` parameter")?;
      if alg != JwsAlgorithm::HS256 {
        return Err("incompatible `alg` parameter");
      }

      let mut mac: [u8; SHA256_LEN] = Default::default();
      crypto::macs::hmac::HMAC_SHA256(msg, &shared_secret, &mut mac);

      if sig == mac {
        Ok(())
      } else {
        Err("invalid signature")
      }
    };

    let mut decoder = jws::Decoder::new(verify_fn);

    if tv.detach {
      decoder = decoder.payload(tv.payload);
    }

    let decoded: _ = decoder.critical("b64").decode(tv.encoded).unwrap();

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.payload);
  }
}
