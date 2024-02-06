// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use crate::jwk::Jwk;
use crate::jwk::JwkParamsOct;
use crate::jws::SignatureVerificationError;
use crate::jws::SignatureVerificationErrorKind;
use crate::jws::VerificationInput;
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

pub(crate) fn sign(message: &[u8], private_key: &Jwk) -> impl AsRef<[u8]> {
  let shared_secret: Vec<u8> = expand_hmac_jwk(private_key, SHA256_LEN);
  let mut mac: [u8; SHA256_LEN] = Default::default();
  crypto::macs::hmac::HMAC_SHA256(message, &shared_secret, &mut mac);
  mac
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
