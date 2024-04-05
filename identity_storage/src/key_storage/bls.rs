use identity_verification::jose::jwk::Jwk;
use identity_verification::jose::jwu;
use identity_verification::jwk::BlsCurve;
use identity_verification::jwk::JwkParamsEc;
use zkryptium::bbsplus::keys::BBSplusPublicKey;
use zkryptium::bbsplus::keys::BBSplusSecretKey;

use crate::key_storage::KeyStorageError;
use crate::key_storage::KeyStorageErrorKind;
use crate::key_storage::KeyStorageResult;

pub(crate) fn expand_bls_jwk(jwk: &Jwk) -> KeyStorageResult<(BBSplusSecretKey, BBSplusPublicKey)> {
  let params: &JwkParamsEc = jwk.try_ec_params().unwrap();

  if params
    .try_bls_curve()
    .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType).with_source(err))?
    != BlsCurve::BLS12381G2
  {
    return Err(
      KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
        .with_custom_message(format!("expected an {} key", BlsCurve::BLS12381G2.name())),
    );
  }

  let sk: BBSplusSecretKey = params
    .d
    .as_deref()
    .map(jwu::decode_b64)
    .ok_or_else(|| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("expected Jwk `d` param to be present")
    })?
    .map(|v| BBSplusSecretKey::from_bytes(&v))
    .map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("unable to decode `d` param")
        .with_source(err)
    })?
    .map_err(|_| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message(format!("invalid BBS+ secret key"))
    })?;

  let x: [u8; BBSplusPublicKey::COORDINATE_LEN] = jwu::decode_b64(&params.x)
    .map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("unable to decode `x` param")
        .with_source(err)
    })?
    .try_into()
    .map_err(|_| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message(format!("expected key of length {}", BBSplusPublicKey::COORDINATE_LEN))
    })?;

  let y: [u8; BBSplusPublicKey::COORDINATE_LEN] = jwu::decode_b64(&params.y)
    .map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("unable to decode `y` param")
        .with_source(err)
    })?
    .try_into()
    .map_err(|_| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message(format!("expected key of length {}", BBSplusPublicKey::COORDINATE_LEN))
    })?;

  let pk = BBSplusPublicKey::from_coordinates(&x, &y).map_err(|_| {
    KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message(format!("invalid BBS+ public key"))
  })?;

  Ok((sk, pk))
}

#[cfg(any(test, feature = "memstore"))]
pub(crate) fn encode_bls_jwk(private_key: &BBSplusSecretKey, public_key: &BBSplusPublicKey) -> Jwk {
  let (x, y) = public_key.to_coordinates();
  let x = jwu::encode_b64(x);
  let y = jwu::encode_b64(y);

  let d = jwu::encode_b64(private_key.to_bytes());
  let mut params = JwkParamsEc::new();
  params.x = x;
  params.y = y;
  params.d = Some(d);
  params.crv = BlsCurve::BLS12381G2.name().to_owned();
  Jwk::from_params(params)
}
