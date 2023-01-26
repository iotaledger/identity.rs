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

    if tv.deterministic {
      let encoded: String = match header.alg() {
        JwsAlgorithm::HS256 => hs256::encode(tv, &header, &jwk).await,
        JwsAlgorithm::ES256 => es256::encode(tv, &header, &jwk).await,
        other => unimplemented!("{other}"),
      };

      assert_eq!(encoded.as_bytes(), tv.encoded);
    }

    let decoded: _ = match header.alg() {
      JwsAlgorithm::HS256 => hs256::decode(tv, &jwk),
      JwsAlgorithm::ES256 => es256::decode(tv, &jwk),
      other => unimplemented!("{other}"),
    };

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.claims);
  }
}

mod hs256 {
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

  pub(crate) async fn encode(tv: &TestVector, header: &JwsHeader, jwk: &Jwk) -> String {
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

  pub(crate) fn decode(tv: &TestVector, jwk: &Jwk) -> Token<'static> {
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

mod es256 {
  use identity_jose::jwk::EcCurve;
  use identity_jose::jwk::Jwk;
  use identity_jose::jwk::JwkParamsEc;
  use identity_jose::jws;
  use identity_jose::jws::JwsAlgorithm;
  use identity_jose::jws::JwsHeader;
  use identity_jose::jws::Recipient;
  use identity_jose::jws::Token;
  use identity_jose::jwt::JwtHeaderSet;
  use identity_jose::jwu;
  use p256::ecdsa::Signature;
  use p256::ecdsa::SigningKey;
  use p256::ecdsa::VerifyingKey;
  use p256::PublicKey;
  use p256::SecretKey;

  use crate::TestVector;

  pub(crate) fn expand_p256_jwk(jwk: &Jwk) -> (SecretKey, PublicKey) {
    let params: &JwkParamsEc = jwk.try_ec_params().unwrap();

    if params.try_ec_curve().unwrap() != EcCurve::P256 {
      panic!("expected a P256 curve");
    }

    let sk_bytes = params.d.as_ref().map(jwu::decode_b64).unwrap().unwrap();
    let sk = SecretKey::from_be_bytes(&sk_bytes).unwrap();

    // Transformation according to section 2.3.3 from http://www.secg.org/sec1-v2.pdf.
    let pk_bytes: Vec<u8> = [0x04]
      .into_iter()
      .chain(jwu::decode_b64(&params.x).unwrap().into_iter())
      .chain(jwu::decode_b64(&params.y).unwrap().into_iter())
      .collect();

    let pk = PublicKey::from_sec1_bytes(&pk_bytes).unwrap();

    assert_eq!(sk.public_key(), pk);

    (sk, pk)
  }

  pub(crate) async fn encode(tv: &TestVector, header: &JwsHeader, jwk: &Jwk) -> String {
    let (secret_key, _) = expand_p256_jwk(jwk);

    let signing_key = SigningKey::from(secret_key);

    let sign_fn = move |protected: Option<JwsHeader>, unprotected: Option<JwsHeader>, msg: Vec<u8>| {
      let sk = signing_key.clone();
      async move {
        let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new().protected(&protected).unprotected(&unprotected);
        if header_set.try_alg().map_err(|_| "missing `alg` parameter".to_owned())? != JwsAlgorithm::ES256 {
          return Err("incompatible `alg` parameter".to_owned());
        }
        let signature: Signature = signature::Signer::sign(&sk, &msg);
        let b64 = jwu::encode_b64(signature.to_bytes());
        Ok(b64)
      }
    };

    jws::Encoder::new(sign_fn)
      .recipient(Recipient::new().protected(header))
      .encode(tv.claims)
      .await
      .unwrap()
  }

  pub(crate) fn decode(tv: &TestVector, jwk: &Jwk) -> Token<'static> {
    let (_, public_key) = expand_p256_jwk(jwk);

    let verifying_key = VerifyingKey::from(public_key);

    let verify_fn = move |protected: Option<&JwsHeader>, unprotected: Option<&JwsHeader>, msg: &[u8], sig: &[u8]| {
      let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new().protected(protected).unprotected(unprotected);
      let alg: JwsAlgorithm = header_set.try_alg().map_err(|_| "missing `alg` parameter".to_owned())?;
      if alg != JwsAlgorithm::ES256 {
        return Err("incompatible `alg` parameter".to_owned());
      }

      let signature = Signature::try_from(sig).unwrap();

      match signature::Verifier::verify(&verifying_key, msg, &signature) {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string().to_owned()),
      }
    };

    jws::Decoder::new(verify_fn).decode(tv.encoded).unwrap()
  }
}
