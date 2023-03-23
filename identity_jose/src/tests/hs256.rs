// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use crate::jwk::Jwk;
use crate::jwk::JwkParamsOct;
use crate::jws::Encoder;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::SignatureVerificationError;
use crate::jws::SignatureVerificationErrorKind;
use crate::jws::VerificationInput;
use crate::jwt::JwtHeaderSet;
use crate::jwu;
use crypto::hashes::sha::SHA256_LEN;

pub(crate) fn expand_hmac_jwk(jwk: &Jwk, key_len: usize) -> Vec<u8> {
  let params: &JwkParamsOct = jwk.try_oct_params().unwrap();
  let k: Vec<u8> = jwu::decode_b64(&params.k).unwrap();

  if k.len() >= key_len {
    k
  } else {
    panic!("expected different key length");
  }
}

pub(crate) async fn encode(encoder: &Encoder<'_>, claims: &[u8], jwk: &Jwk) -> String {
  let shared_secret: Vec<u8> = expand_hmac_jwk(jwk, SHA256_LEN);

  let sign_fn = move |protected: Option<JwsHeader>, unprotected: Option<JwsHeader>, msg: Vec<u8>| {
    let sk = shared_secret.clone();
    async move {
      let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new()
        .with_protected(&protected)
        .with_unprotected(&unprotected);
      if header_set.try_alg().map_err(|_| "missing `alg` parameter".to_owned())? != JwsAlgorithm::HS256 {
        return Err("incompatible `alg` parameter".to_owned());
      }
      let mut mac: [u8; SHA256_LEN] = Default::default();
      crypto::macs::hmac::HMAC_SHA256(&msg, &sk, &mut mac);
      Ok(jwu::encode_b64(mac))
    }
  };

  encoder.encode(&sign_fn, claims).await.unwrap()
}

pub(crate) fn verify(verification_input: VerificationInput, jwk: &Jwk) -> Result<(), SignatureVerificationError> {
  let shared_secret: Vec<u8> = expand_hmac_jwk(jwk, SHA256_LEN);

  let mut mac: [u8; SHA256_LEN] = Default::default();
  crypto::macs::hmac::HMAC_SHA256(&verification_input.signing_input, &shared_secret, &mut mac);

  if verification_input.decoded_signature.deref() == mac {
    Ok(())
  } else {
    Err(SignatureVerificationErrorKind::InvalidSignature.into())
  }
}
