// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519::PublicKey;
use crypto::signatures::ed25519::SecretKey;
use crypto::signatures::ed25519::{self};
use identity_verification::jwk::EdCurve;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkParamsOkp;
use identity_verification::jwu;

use crate::key_storage::KeyStorageError;
use crate::key_storage::KeyStorageErrorKind;
use crate::key_storage::KeyStorageResult;

pub(crate) fn expand_secret_jwk(jwk: &Jwk) -> KeyStorageResult<SecretKey> {
  let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

  if params
    .try_ed_curve()
    .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType).with_source(err))?
    != EdCurve::Ed25519
  {
    return Err(
      KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
        .with_custom_message(format!("expected an {} key", EdCurve::Ed25519.name())),
    );
  }

  let sk: [u8; ed25519::SECRET_KEY_LENGTH] = params
    .d
    .as_deref()
    .map(jwu::decode_b64)
    .ok_or_else(|| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("expected Jwk `d` param to be present")
    })?
    .map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("unable to decode `d` param")
        .with_source(err)
    })?
    .try_into()
    .map_err(|_| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message(format!("expected key of length {}", ed25519::SECRET_KEY_LENGTH))
    })?;

  Ok(SecretKey::from_bytes(sk))
}

pub(crate) fn encode_jwk(private_key: &SecretKey, public_key: &PublicKey) -> Jwk {
  let x = jwu::encode_b64(public_key.as_ref());
  let d = jwu::encode_b64(private_key.to_bytes().as_ref());
  let mut params = JwkParamsOkp::new();
  params.x = x;
  params.d = Some(d);
  params.crv = EdCurve::Ed25519.name().to_owned();
  Jwk::from_params(params)
}

#[cfg(test)]
pub(crate) fn expand_public_jwk(jwk: &Jwk) -> PublicKey {
  let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

  if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
    panic!("expected an ed25519 jwk");
  }

  let pk: [u8; ed25519::PUBLIC_KEY_LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();
  PublicKey::try_from(pk).unwrap()
}

#[cfg(test)]
pub(crate) fn generate_ed25519() -> (SecretKey, PublicKey) {
  let private_key = SecretKey::generate().unwrap();
  let public_key = private_key.public_key();
  (private_key, public_key)
}
