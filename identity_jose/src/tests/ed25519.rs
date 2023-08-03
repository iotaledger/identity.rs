// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use crypto::signatures::ed25519::PublicKey;
use crypto::signatures::ed25519::SecretKey;
use crypto::signatures::ed25519::Signature;

use crate::jwk::EdCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkParamsOkp;

use crate::jws::SignatureVerificationError;
use crate::jws::SignatureVerificationErrorKind;
use crate::jws::VerificationInput;
use crate::jwu;

pub(crate) fn expand_secret_jwk(jwk: &Jwk) -> SecretKey {
  let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

  if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
    panic!("expected an ed25519 jwk");
  }

  let sk: [u8; SecretKey::LENGTH] = params
    .d
    .as_deref()
    .map(jwu::decode_b64)
    .unwrap()
    .unwrap()
    .try_into()
    .unwrap();

  SecretKey::from_bytes(&sk)
}

pub(crate) fn expand_public_jwk(jwk: &Jwk) -> PublicKey {
  let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

  if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
    panic!("expected an ed25519 jwk");
  }

  let pk: [u8; PublicKey::LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();

  PublicKey::try_from(pk).unwrap()
}

pub(crate) fn sign(message: &[u8], private_key: &Jwk) -> impl AsRef<[u8]> {
  let sk: SecretKey = expand_secret_jwk(private_key);
  sk.sign(message).to_bytes()
}

pub(crate) fn verify(verification_input: VerificationInput, jwk: &Jwk) -> Result<(), SignatureVerificationError> {
  let public_key = expand_public_jwk(jwk);

  let signature_arr = <[u8; Signature::LENGTH]>::try_from(verification_input.decoded_signature.deref())
    .map_err(|err| err.to_string())
    .unwrap();

  let signature = Signature::from_bytes(signature_arr);
  if public_key.verify(&signature, &verification_input.signing_input) {
    Ok(())
  } else {
    Err(SignatureVerificationErrorKind::InvalidSignature.into())
  }
}
