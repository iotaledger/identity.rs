// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519::SecretKey;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::ed25519::Ed25519PublicKeyAsBytes;
use fastcrypto::traits::ToFromBytes;
use identity_verification::jose::jwk::EdCurve;
use identity_verification::jose::jwk::Jwk;
use identity_verification::jose::jwk::JwkParamsOkp;
use identity_verification::jose::jwu;
use identity_verification::jwu::encode_b64;

use crate::key_storage::KeyStorageError;
use crate::key_storage::KeyStorageErrorKind;
use crate::key_storage::KeyStorageResult;

pub(crate) fn jwk_to_keypair(jwk: &Jwk) -> KeyStorageResult<Ed25519KeyPair> {
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

  let sk: [u8; SecretKey::LENGTH] = params
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
        .with_custom_message(format!("expected key of length {}", SecretKey::LENGTH))
    })?;

  Ed25519KeyPair::from_bytes(&sk)
    .map_err(|_| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("invalid key"))
}

#[cfg(any(test, feature = "memstore"))]
pub(crate) fn encode_jwk(private_key: &SecretKey, public_key: &crypto::signatures::ed25519::PublicKey) -> Jwk {
  let x = jwu::encode_b64(public_key.as_ref());
  let d = jwu::encode_b64(private_key.to_bytes().as_ref());
  let mut params = JwkParamsOkp::new();
  params.x = x;
  params.d = Some(d);
  params.crv = EdCurve::Ed25519.name().to_string();
  Jwk::from_params(params)
}

pub(crate) fn pk_to_jwk(pk: &Ed25519PublicKeyAsBytes) -> Jwk {
  let params = JwkParamsOkp {
    crv: EdCurve::Ed25519.to_string(),
    x: encode_b64(&pk.0),
    d: None,
  };

  Jwk::from_params(params)
}
