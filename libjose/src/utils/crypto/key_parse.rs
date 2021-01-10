use core::convert::TryInto;
use core::iter::once;

use crate::error::Error;
use crate::error::Result;
use crate::jwk::EcCurve;
use crate::jwk::EcxCurve;
use crate::jwk::EdCurve;
use crate::jwk::JwkParamsEc;
use crate::jwk::JwkParamsOct;
use crate::jwk::JwkParamsOkp;
use crate::jwk::JwkParamsRsa;
use crate::lib::*;
use crate::utils::decode_b64;
use crate::utils::Secret;

pub type RsaPublicKey = rsa::RSAPublicKey;
pub type RsaSecretKey = rsa::RSAPrivateKey;
pub type RsaUint = rsa::BigUint;
pub type RsaPadding = rsa::PaddingScheme;

pub type Ed25519PublicKey = crypto::ed25519::PublicKey;
pub type Ed25519SecretKey = crypto::ed25519::SecretKey;
pub type Ed25519Signature = crypto::ed25519::Signature;

pub type P256PublicKey = p256::ecdsa::VerifyingKey;
pub type P256SecretKey = p256::ecdsa::SigningKey;
pub type P256Signature = p256::ecdsa::Signature;

pub type K256PublicKey = k256::ecdsa::VerifyingKey;
pub type K256SecretKey = k256::ecdsa::SigningKey;
pub type K256Signature = k256::ecdsa::Signature;

pub type X25519PublicKey = x25519_dalek::PublicKey;
pub type X25519SecretKey = x25519_dalek::StaticSecret;

pub type X448PublicKey = x448::PublicKey;
pub type X448SecretKey = x448::Secret;

pub const X25519_PUBLIC_LEN: usize = 32;
pub const X25519_SECRET_LEN: usize = 32;

pub const X448_PUBLIC_LEN: usize = 56;
pub const X448_SECRET_LEN: usize = 56;

const SEC1_UNCOMPRESSED_TAG: u8 = 0x04;

// =============================================================================
// =============================================================================

#[macro_export]
macro_rules! rsa_padding {
  (@PKCS1_SHA256) => {
    ::rsa::PaddingScheme::new_pkcs1v15_sign(Some(::rsa::Hash::SHA2_256))
  };
  (@PKCS1_SHA384) => {
    ::rsa::PaddingScheme::new_pkcs1v15_sign(Some(::rsa::Hash::SHA2_384))
  };
  (@PKCS1_SHA512) => {
    ::rsa::PaddingScheme::new_pkcs1v15_sign(Some(::rsa::Hash::SHA2_512))
  };
  (@PSS_SHA256) => {
    ::rsa::PaddingScheme::new_pss::<::sha2::Sha256, _>(::rand::rngs::OsRng)
  };
  (@PSS_SHA384) => {
    ::rsa::PaddingScheme::new_pss::<::sha2::Sha384, _>(::rand::rngs::OsRng)
  };
  (@PSS_SHA512) => {
    ::rsa::PaddingScheme::new_pss::<::sha2::Sha512, _>(::rand::rngs::OsRng)
  };
  (@RSA1_5) => {
    ::rsa::PaddingScheme::new_pkcs1v15_encrypt()
  };
  (@RSA_OAEP) => {
    ::rsa::PaddingScheme::new_oaep::<::sha1::Sha1>()
  };
  (@RSA_OAEP_256) => {
    ::rsa::PaddingScheme::new_oaep::<::sha2::Sha256>()
  };
  (@RSA_OAEP_384) => {
    ::rsa::PaddingScheme::new_oaep::<::sha2::Sha384>()
  };
  (@RSA_OAEP_512) => {
    ::rsa::PaddingScheme::new_oaep::<::sha2::Sha512>()
  };
}

// =============================================================================
// =============================================================================

pub fn expand_oct_key(key_len: usize, secret: Secret<'_>) -> Result<Cow<'_, [u8]>> {
  match secret {
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

// =============================================================================
// =============================================================================

pub fn expand_rsa_public(secret: Secret<'_>) -> Result<RsaPublicKey> {
  match secret {
    Secret::Arr(arr) => RsaPublicKey::from_pkcs1(arr)
      .or_else(|_| RsaPublicKey::from_pkcs8(arr))
      .map_err(Into::into),
    Secret::Jwk(jwk) => {
      let params: &JwkParamsRsa = jwk.try_rsa_params()?;
      let n: RsaUint = decode_rsa_uint(&params.n)?;
      let e: RsaUint = decode_rsa_uint(&params.e)?;

      RsaPublicKey::new(n, e).map_err(Into::into)
    }
  }
}

#[allow(clippy::many_single_char_names)]
pub fn expand_rsa_secret(secret: Secret<'_>) -> Result<RsaSecretKey> {
  match secret {
    Secret::Arr(arr) => RsaSecretKey::from_pkcs1(arr)
      .or_else(|_| RsaSecretKey::from_pkcs8(arr))
      .map_err(Into::into),
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

fn decode_rsa_uint(data: impl AsRef<[u8]>) -> Result<RsaUint> {
  decode_b64(data).map(|data| RsaUint::from_bytes_be(&data))
}

fn decode_rsa_uint_opt(data: Option<impl AsRef<[u8]>>) -> Result<RsaUint> {
  data.ok_or(Error::KeyError("RSA")).and_then(decode_rsa_uint)
}

// =============================================================================
// =============================================================================

pub fn expand_ec_public<T, E>(curve: EcCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T>
where
  E: Into<Error>,
{
  secret.expand(f, |jwk| {
    let params: &JwkParamsEc = jwk.try_ec_params()?;

    if params.try_ec_curve()? != curve {
      return Err(Error::KeyError("Ec Curve"));
    }

    let sec1: Vec<u8> = once(SEC1_UNCOMPRESSED_TAG)
      .chain(decode_b64(&params.x)?.into_iter())
      .chain(decode_b64(&params.y)?.into_iter())
      .collect();

    Ok(sec1)
  })
}

pub fn expand_ec_secret<T, E>(curve: EcCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T>
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

pub fn expand_p256_public(secret: Secret<'_>) -> Result<P256PublicKey> {
  expand_ec_public(EcCurve::P256, secret, P256PublicKey::from_sec1_bytes)
}

pub fn expand_p256_secret(secret: Secret<'_>) -> Result<P256SecretKey> {
  expand_ec_secret(EcCurve::P256, secret, P256SecretKey::from_bytes)
}

pub fn expand_k256_public(secret: Secret<'_>) -> Result<K256PublicKey> {
  expand_ec_public(EcCurve::Secp256K1, secret, K256PublicKey::from_sec1_bytes)
}

pub fn expand_k256_secret(secret: Secret<'_>) -> Result<K256SecretKey> {
  expand_ec_secret(EcCurve::Secp256K1, secret, K256SecretKey::from_bytes)
}

// =============================================================================
// =============================================================================

pub fn expand_ed25519_public(secret: Secret<'_>) -> Result<Ed25519PublicKey> {
  secret.expand(
    |arr| arr.try_into(),
    |jwk| {
      let params: &JwkParamsOkp = jwk.try_okp_params()?;

      if params.try_ed_curve()? != EdCurve::Ed25519 {
        return Err(Error::KeyError("Ed25519"));
      }

      decode_b64(&params.x)
    },
  )
}

pub fn expand_ed25519_secret(secret: Secret<'_>) -> Result<Ed25519SecretKey> {
  secret.expand(
    |arr| arr.try_into(),
    |jwk| {
      let params: &JwkParamsOkp = jwk.try_okp_params()?;

      if params.try_ed_curve()? != EdCurve::Ed25519 {
        return Err(Error::KeyError("Ed25519"));
      }

      params
        .d
        .as_deref()
        .map(decode_b64)
        .transpose()?
        .ok_or(Error::KeyError("Ed25519"))
    },
  )
}

// =============================================================================
// =============================================================================

pub fn expand_ecx_public<T, E>(curve: EcxCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T> {
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

pub fn expand_ecx_secret<T, E>(curve: EcxCurve, secret: Secret<'_>, f: impl Fn(&[u8]) -> Result<T, E>) -> Result<T> {
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

pub fn expand_x25519_public(secret: Secret<'_>) -> Result<X25519PublicKey> {
  expand_ecx_public(EcxCurve::X25519, secret, |arr| {
    TryInto::<[u8; X25519_PUBLIC_LEN]>::try_into(arr).map(Into::into)
  })
}

pub fn expand_x25519_secret(secret: Secret<'_>) -> Result<X25519SecretKey> {
  expand_ecx_secret(EcxCurve::X25519, secret, |arr| {
    TryInto::<[u8; X25519_SECRET_LEN]>::try_into(arr).map(Into::into)
  })
}

// =============================================================================
// =============================================================================

pub fn expand_x448_public(secret: Secret<'_>) -> Result<X448PublicKey> {
  expand_ecx_public(EcxCurve::X448, secret, |arr| {
    X448PublicKey::from_bytes(arr).ok_or_else(|| Error::KeyError(EcxCurve::X448.name()))
  })
}

pub fn expand_x448_secret(secret: Secret<'_>) -> Result<X448SecretKey> {
  expand_ecx_secret(EcxCurve::X448, secret, |arr| {
    TryInto::<[u8; X448_SECRET_LEN]>::try_into(arr).map(Into::into)
  })
}
