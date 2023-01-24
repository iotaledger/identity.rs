// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_jose::jwk::Jwk;
use identity_jose::jws::JwsAlgorithm;
use identity_jose::jws::JwsHeader;

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

    let (encode, decode) = match header.alg() {
      JwsAlgorithm::HS256 => (hmac::hmac_256_encode, hmac::hmac_256_decode),
      other => unimplemented!("{other}"),
    };

    if tv.deterministic {
      let encoded: String = encode(tv, &header, &jwk).await;
      assert_eq!(encoded.as_bytes(), tv.encoded);
    }

    let decoded = decode(tv, &jwk);

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.claims);
  }
}

mod hmac {
  use super::TestVector;
  use crypto::hashes::sha::SHA256_LEN;
  use identity_jose::jwk::Jwk;
  use identity_jose::jwk::JwkParamsOct;
  use identity_jose::jws;
  use identity_jose::jws::JwsAlgorithm;
  use identity_jose::jws::JwsHeader;
  use identity_jose::jws::Recipient;
  use identity_jose::jws::Token;
  use identity_jose::jwt::JwtHeaderSet;
  use identity_jose::jwu;

  fn expand_hmac_jwk(jwk: &Jwk, key_len: usize) -> Vec<u8> {
    let params: &JwkParamsOct = jwk.try_oct_params().unwrap();
    let k: Vec<u8> = jwu::decode_b64(&params.k).unwrap();

    if k.len() >= key_len {
      k
    } else {
      panic!("expected different key length");
    }
  }

  pub(crate) async fn hmac_256_encode(tv: &TestVector, header: &JwsHeader, jwk: &Jwk) -> String {
    let shared_secret: Vec<u8> = expand_hmac_jwk(jwk, SHA256_LEN);

    let sign_fn = move |protected: Option<JwsHeader>, unprotected: Option<JwsHeader>, msg: Vec<u8>| {
      let sk = shared_secret.clone();
      async move {
        let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new().protected(&protected).unprotected(&unprotected);
        if header_set.try_alg().map_err(|_| "missing `alg` parameter".to_owned())? != JwsAlgorithm::HS256 {
          return Err("incompatible `alg` parameter".to_owned());
        }
        let mut mac: [u8; SHA256_LEN] = Default::default();
        crypto::macs::hmac::HMAC_SHA256(&msg, &sk, &mut mac);
        Ok(jwu::encode_b64(mac))
      }
    };

    jws::Encoder::new(sign_fn)
      .recipient(Recipient::new().protected(header))
      .encode(tv.claims)
      .await
      .unwrap()
  }

  pub(crate) fn hmac_256_decode(tv: &TestVector, jwk: &Jwk) -> Token<'static> {
    let shared_secret: Vec<u8> = expand_hmac_jwk(jwk, SHA256_LEN);

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

    jws::Decoder::new(verify_fn).decode(tv.encoded).unwrap()
  }
}
