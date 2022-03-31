// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;

use crate::error::Error;
use crate::error::Result;
use crate::jwk::EcCurve;
use crate::jwk::EcxCurve;
use crate::jwk::EdCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkParamsEc;
use crate::jwk::JwkParamsOct;
use crate::jwk::JwkParamsOkp;
use crate::jwk::JwkParamsRsa;
use crate::lib::*;
use crate::utils::decode_b64;
use rsa::pkcs1::FromRsaPrivateKey;
use rsa::pkcs1::FromRsaPublicKey;
use rsa::pkcs8::FromPrivateKey;
use rsa::pkcs8::FromPublicKey;

pub type RsaPublicKey = rsa::RsaPublicKey;
pub type RsaSecretKey = rsa::RsaPrivateKey;
pub type RsaUint = rsa::BigUint;
pub type RsaPadding = rsa::PaddingScheme;

pub type Ed25519PublicKey = crypto::signatures::ed25519::PublicKey;
pub type Ed25519SecretKey = crypto::signatures::ed25519::SecretKey;
pub type Ed25519Signature = crypto::signatures::ed25519::Signature;

pub type P256PublicKey = crypto::signatures::p256::PublicKey;
pub type P256SecretKey = crypto::signatures::p256::SecretKey;

pub type K256PublicKey = crypto::signatures::secp256k1::PublicKey;
pub type K256SecretKey = crypto::signatures::secp256k1::SecretKey;

pub type X25519PublicKey = crypto::keys::x25519::PublicKey;
pub type X25519SecretKey = crypto::keys::x25519::SecretKey;

pub type X448PublicKey = crypto::keys::x448::PublicKey;
pub type X448SecretKey = crypto::keys::x448::SecretKey;

pub const ED25519_PUBLIC_KEY_LEN: usize = crypto::signatures::ed25519::COMPRESSED_PUBLIC_KEY_LENGTH;
pub const ED25519_SECRET_KEY_LEN: usize = crypto::signatures::ed25519::SECRET_KEY_LENGTH;

pub const X25519_PUBLIC_KEY_LEN: usize = crypto::keys::x25519::PUBLIC_KEY_LEN;
pub const X25519_SECRET_KEY_LEN: usize = crypto::keys::x25519::SECRET_KEY_LEN;

#[derive(Clone, Copy, Debug)]
pub enum Secret<'a> {
  Arr(&'a [u8]),
  Jwk(&'a Jwk),
}

impl<'a> Secret<'a> {
  pub fn check_signing_key(&self, algorithm: &str) -> Result<()> {
    if let Self::Jwk(jwk) = self {
      jwk.check_signing_key(algorithm)?;
    }

    Ok(())
  }

  pub fn check_verifying_key(&self, algorithm: &str) -> Result<()> {
    if let Self::Jwk(jwk) = self {
      jwk.check_verifying_key(algorithm)?;
    }

    Ok(())
  }

  pub fn check_encryption_key(&self, algorithm: &str) -> Result<()> {
    if let Self::Jwk(jwk) = self {
      jwk.check_encryption_key(algorithm)?;
    }

    Ok(())
  }

  pub fn check_decryption_key(&self, algorithm: &str) -> Result<()> {
    if let Self::Jwk(jwk) = self {
      jwk.check_decryption_key(algorithm)?;
    }

    Ok(())
  }

  pub fn to_oct_key(self, key_len: usize) -> Result<Cow<'a, [u8]>> {
    match self {
      Secret::Arr(arr) => {
        if arr.len() >= key_len {
          return Ok(Cow::Borrowed(arr));
        }
      }
      Secret::Jwk(jwk) => {
        let params: &JwkParamsOct = jwk.try_oct_params()?;
        let k: Vec<u8> = decode_b64(&params.k)?;

        if k.len() >= key_len {
          return Ok(Cow::Owned(k));
        }
      }
    }

    Err(Error::KeyError("Oct Key Length"))
  }

  pub fn to_rsa_public(self) -> Result<RsaPublicKey> {
    match self {
      Secret::Arr(arr) => RsaPublicKey::from_pkcs1_der(arr)
        .or_else(|_| RsaPublicKey::from_public_key_der(arr))
        .map_err(|err| rsa::errors::Error::from(err).into()),
      Secret::Jwk(jwk) => {
        let params: &JwkParamsRsa = jwk.try_rsa_params()?;
        let n: RsaUint = decode_rsa_uint(&params.n)?;
        let e: RsaUint = decode_rsa_uint(&params.e)?;

        RsaPublicKey::new(n, e).map_err(Into::into)
      }
    }
  }

  #[allow(clippy::many_single_char_names)]
  pub fn to_rsa_secret(self) -> Result<RsaSecretKey> {
    match self {
      Secret::Arr(arr) => RsaSecretKey::from_pkcs1_der(arr)
        .or_else(|_| RsaSecretKey::from_pkcs8_der(arr))
        .map_err(|err| rsa::errors::Error::from(err).into()),
      Secret::Jwk(jwk) => {
        let params: &JwkParamsRsa = jwk.try_rsa_params()?;

        // TODO: Handle Multi-prime keys
        if params.oth.is_some() {
          return Err(Error::KeyError("multi prime keys are not supported"));
        }

        let n: RsaUint = decode_rsa_uint(&params.n)?;
        let e: RsaUint = decode_rsa_uint(&params.e)?;
        let d: RsaUint = decode_rsa_uint_opt(params.d.as_deref())?;
        let p: RsaUint = decode_rsa_uint_opt(params.p.as_deref())?;
        let q: RsaUint = decode_rsa_uint_opt(params.q.as_deref())?;

        // TODO: Check against generated properties

        let _dp: RsaUint = decode_rsa_uint_opt(params.dp.as_deref())?;
        let _dq: RsaUint = decode_rsa_uint_opt(params.dq.as_deref())?;
        let _qi: RsaUint = decode_rsa_uint_opt(params.qi.as_deref())?;

        let key: _ = RsaSecretKey::from_components(n, e, d, vec![p, q]);

        key.validate()?;

        Ok(key)
      }
    }
  }

  pub fn to_p256_public(self) -> Result<P256PublicKey> {
    expand_ec_public(EcCurve::P256, self, P256PublicKey::from_bytes)
  }

  pub fn to_p256_secret(self) -> Result<P256SecretKey> {
    expand_ec_secret(EcCurve::P256, self, P256SecretKey::from_bytes)
  }

  pub fn to_k256_public(self) -> Result<K256PublicKey> {
    expand_ec_public(EcCurve::Secp256K1, self, K256PublicKey::from_bytes)
  }

  pub fn to_k256_secret(self) -> Result<K256SecretKey> {
    expand_ec_secret(EcCurve::Secp256K1, self, K256SecretKey::from_bytes)
  }

  pub fn to_ed25519_public(self) -> Result<Ed25519PublicKey> {
    expand_ed_public(EdCurve::Ed25519, self, parse_ed25519_public_key)
  }

  pub fn to_ed25519_secret(self) -> Result<Ed25519SecretKey> {
    expand_ed_secret(EdCurve::Ed25519, self, parse_ed25519_secret_key)
  }

  pub fn to_x25519_public(self) -> Result<X25519PublicKey> {
    expand_ecx_public(EcxCurve::X25519, self, X25519PublicKey::from_bytes)
  }

  pub fn to_x25519_secret(self) -> Result<X25519SecretKey> {
    expand_ecx_secret(EcxCurve::X25519, self, X25519SecretKey::from_bytes)
  }

  pub fn to_x448_public(self) -> Result<X448PublicKey> {
    expand_ecx_public(EcxCurve::X448, self, X448PublicKey::from_bytes)
  }

  pub fn to_x448_secret(self) -> Result<X448SecretKey> {
    expand_ecx_secret(EcxCurve::X448, self, X448SecretKey::from_bytes)
  }

  pub(crate) fn expand<T, E>(
    self,
    expand_arr: impl Fn(&[u8]) -> Result<T, E>,
    expand_jwk: impl Fn(&Jwk) -> Result<Vec<u8>>,
  ) -> Result<T>
  where
    E: Into<Error>,
  {
    match self {
      Self::Arr(arr) => expand_arr(arr).map_err(Into::into),
      Self::Jwk(jwk) => expand_arr(&expand_jwk(jwk)?).map_err(Into::into),
    }
  }
}

impl<'a> From<&'a [u8]> for Secret<'a> {
  fn from(other: &'a [u8]) -> Self {
    Self::Arr(other)
  }
}

impl<'a> From<&'a Jwk> for Secret<'a> {
  fn from(other: &'a Jwk) -> Self {
    Self::Jwk(other)
  }
}

impl<'a, T> From<&'a T> for Secret<'a>
where
  T: AsRef<[u8]>,
{
  fn from(other: &'a T) -> Self {
    Self::Arr(other.as_ref())
  }
}

// =============================================================================
// =============================================================================

fn decode_rsa_uint(data: impl AsRef<[u8]>) -> Result<RsaUint> {
  decode_b64(data).map(|data| RsaUint::from_bytes_be(&data))
}

fn decode_rsa_uint_opt(data: Option<impl AsRef<[u8]>>) -> Result<RsaUint> {
  data.ok_or(Error::KeyError("RSA")).and_then(decode_rsa_uint)
}

fn parse_ed25519_public_key(arr: &[u8]) -> Result<Ed25519PublicKey> {
  let bytes: [u8; ED25519_PUBLIC_KEY_LEN] = arr.try_into().map_err(|_| Error::KeyError(EdCurve::Ed25519.name()))?;

  Ed25519PublicKey::from_compressed_bytes(bytes).map_err(|_| Error::KeyError(EdCurve::Ed25519.name()))
}

fn parse_ed25519_secret_key(arr: &[u8]) -> Result<Ed25519SecretKey> {
  let bytes: [u8; ED25519_SECRET_KEY_LEN] = arr.try_into().map_err(|_| Error::KeyError(EdCurve::Ed25519.name()))?;

  Ed25519SecretKey::from_le_bytes(bytes).map_err(|_| Error::KeyError(EdCurve::Ed25519.name()))
}

fn expand_ec_public<T, E>(curve: EcCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T>
where
  E: Into<Error>,
{
  secret.expand(f, |jwk| {
    let params: &JwkParamsEc = jwk.try_ec_params()?;

    if params.try_ec_curve()? != curve {
      return Err(Error::KeyError(curve.name()));
    }

    let bytes: Vec<u8> = decode_b64(&params.x)?
      .into_iter()
      .chain(decode_b64(&params.y)?.into_iter())
      .collect();

    Ok(bytes)
  })
}

fn expand_ec_secret<T, E>(curve: EcCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T>
where
  E: Into<Error>,
{
  secret.expand(f, |jwk| {
    let params: &JwkParamsEc = jwk.try_ec_params()?;

    if params.try_ec_curve()? != curve {
      return Err(Error::KeyError(curve.name()));
    }

    params
      .d
      .as_ref()
      .map(decode_b64)
      .transpose()?
      .ok_or_else(|| Error::KeyError(curve.name()))
  })
}

fn expand_ed_public<T, E>(curve: EdCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T>
where
  E: Into<Error>,
{
  secret.expand(
    |arr| f(arr),
    |jwk| {
      let params: &JwkParamsOkp = jwk.try_okp_params()?;

      if params.try_ed_curve()? != curve {
        return Err(Error::KeyError(curve.name()));
      }

      decode_b64(&params.x)
    },
  )
}

fn expand_ed_secret<T, E>(curve: EdCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T>
where
  E: Into<Error>,
{
  secret.expand(
    |arr| f(arr),
    |jwk| {
      let params: &JwkParamsOkp = jwk.try_okp_params()?;

      if params.try_ed_curve()? != curve {
        return Err(Error::KeyError(curve.name()));
      }

      params
        .d
        .as_deref()
        .map(decode_b64)
        .transpose()?
        .ok_or_else(|| Error::KeyError(curve.name()))
    },
  )
}

fn expand_ecx_public<T, E>(curve: EcxCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T> {
  secret.expand(
    |arr| f(arr).map_err(|_| Error::KeyError(curve.name())),
    |jwk| {
      let params: &JwkParamsOkp = jwk.try_okp_params()?;

      if params.try_ecx_curve()? != curve {
        return Err(Error::KeyError(curve.name()));
      }

      decode_b64(&params.x)
    },
  )
}

fn expand_ecx_secret<T, E>(curve: EcxCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T> {
  secret.expand(
    |arr| f(arr).map_err(|_| Error::KeyError(curve.name())),
    |jwk| {
      let params: &JwkParamsOkp = jwk.try_okp_params()?;

      if params.try_ecx_curve()? != curve {
        return Err(Error::KeyError(curve.name()));
      }

      params
        .d
        .as_ref()
        .map(decode_b64)
        .transpose()?
        .ok_or_else(|| Error::KeyError(curve.name()))
    },
  )
}
