// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::ed25519::Ed25519PrivateKey;
#[cfg(test)]
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::ed25519::Ed25519PublicKeyAsBytes;
use fastcrypto::traits::KeyPair as _;
use fastcrypto::traits::SigningKey;
use fastcrypto::traits::ToFromBytes;
use identity_verification::jose::jwk::EdCurve;
use identity_verification::jose::jwk::Jwk;
use identity_verification::jose::jwk::JwkParamsOkp;
use identity_verification::jose::jwu;
use identity_verification::jwu::encode_b64;

use crate::key_storage::KeyStorageError;
use crate::key_storage::KeyStorageErrorKind;
use crate::key_storage::KeyStorageResult;

#[cfg(test)]
pub(crate) fn from_public_jwk(jwk: &Jwk) -> anyhow::Result<Ed25519PublicKey> {
  use identity_verification::jwu::decode_b64;

  let bytes = decode_b64(&jwk.try_okp_params()?.x)?;
  Ok(Ed25519PublicKey::from_bytes(&bytes)?)
}

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

  let sk: [u8; Ed25519PrivateKey::LENGTH] = params
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
        .with_custom_message(format!("expected key of length {}", Ed25519PrivateKey::LENGTH))
    })?;

  Ed25519KeyPair::from_bytes(&sk)
    .map_err(|_| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("invalid key"))
}

#[allow(dead_code)]
pub(crate) fn encode_jwk(key_pair: Ed25519KeyPair) -> Jwk {
  let x = jwu::encode_b64(key_pair.public().as_ref());
  let d = jwu::encode_b64(key_pair.private().as_ref());
  let mut params = JwkParamsOkp::new();
  params.x = x;
  params.d = Some(d);
  params.crv = EdCurve::Ed25519.name().to_string();
  Jwk::from_params(params)
}

#[allow(dead_code)]
pub(crate) fn pk_to_jwk(pk: &Ed25519PublicKeyAsBytes) -> Jwk {
  use identity_verification::jws::JwsAlgorithm;

  let params = JwkParamsOkp {
    crv: EdCurve::Ed25519.to_string(),
    x: encode_b64(pk.0),
    d: None,
  };

  let mut jwk = Jwk::from_params(params);
  jwk.set_alg(JwsAlgorithm::EdDSA.name());

  jwk
}
