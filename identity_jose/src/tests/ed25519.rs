// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crypto::signatures::ed25519::PublicKey;
use crypto::signatures::ed25519::SecretKey;
use crypto::signatures::ed25519::{self};

use crate::jwk::EdCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkParamsOkp;
use crate::jws::Decoder;
use crate::jws::Encoder;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::Token;
use crate::jwt::JwtHeaderSet;
use crate::jwu;

pub(crate) fn expand_secret_jwk(jwk: &Jwk) -> SecretKey {
  let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

  if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
    panic!("expected an ed25519 jwk");
  }

  let sk: [u8; ed25519::SECRET_KEY_LENGTH] = params
    .d
    .as_deref()
    .map(jwu::decode_b64)
    .unwrap()
    .unwrap()
    .try_into()
    .unwrap();

  SecretKey::from_bytes(sk)
}

pub(crate) fn expand_public_jwk(jwk: &Jwk) -> PublicKey {
  let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

  if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
    panic!("expected an ed25519 jwk");
  }

  let pk: [u8; ed25519::PUBLIC_KEY_LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();

  PublicKey::try_from(pk).unwrap()
}

pub(crate) async fn encode(encoder: &Encoder<'_>, claims: &[u8], secret_key: SecretKey) -> String {
  let sk = Arc::new(secret_key);

  let sign_fn = move |protected: Option<JwsHeader>, unprotected: Option<JwsHeader>, msg: Vec<u8>| {
    let sk = sk.clone();
    async move {
      let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new()
        .with_protected(&protected)
        .with_unprotected(&unprotected);
      if header_set.try_alg().map_err(|_| "missing `alg` parameter")? != JwsAlgorithm::EdDSA {
        return Err("incompatible `alg` parameter");
      }
      let sig: _ = sk.sign(msg.as_slice()).to_bytes();
      Ok(jwu::encode_b64(sig))
    }
  };

  encoder.encode(&sign_fn, claims).await.unwrap()
}

pub(crate) fn decode<'a>(decoder: &Decoder<'a>, encoded: &'a [u8], public_key: PublicKey) -> Token<'a> {
  let verify_fn = |protected: Option<&JwsHeader>, unprotected: Option<&JwsHeader>, msg: &[u8], sig: &[u8]| {
    let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new()
      .with_protected(protected)
      .with_unprotected(unprotected);
    if header_set.try_alg().map_err(|_| "missing `alg` parameter")? != JwsAlgorithm::EdDSA {
      return Err("incompatible `alg` parameter");
    }

    let signature_arr = <[u8; crypto::signatures::ed25519::SIGNATURE_LENGTH]>::try_from(sig)
      .map_err(|err| err.to_string())
      .unwrap();

    let signature = crypto::signatures::ed25519::Signature::from_bytes(signature_arr);
    if public_key.verify(&signature, msg) {
      Ok(())
    } else {
      Err("invalid signature")
    }
  };

  decoder.decode(&verify_fn, encoded).unwrap()
}
