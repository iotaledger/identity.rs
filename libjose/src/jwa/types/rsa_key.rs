use crate::error::Error;
use crate::error::Result;
use crate::jwa::PKey;
use crate::jwa::Public;
use crate::jwa::RsaBits;
use crate::jwa::Secret;
use crate::jwk::Jwk;
use crate::jwk::JwkParams;
use crate::jwk::JwkParamsRsa;
use crate::jwk::JwkType;
use crate::jwk::JwkUse;
use crate::lib::*;
use crate::utils::decode_b64;
use crate::utils::encode_b64;

#[derive(Debug)]
pub struct RsaKeyPair {
  pub(crate) key: PKey<Secret>,
  pub(crate) alg: Option<String>,
  pub(crate) kid: Option<String>,
}

impl RsaKeyPair {
  pub fn public_from_jwk(data: &Jwk) -> Result<PKey<Public>> {
    data.check_kty(JwkType::Rsa)?;

    let params: &JwkParamsRsa = match data.params() {
      JwkParams::Rsa(params) => params,
      _ => return Err(Error::KeyError("Rsa")),
    };

    let n: rsa::BigUint = decode_b64_biguint(&params.n)?;
    let e: rsa::BigUint = decode_b64_biguint(&params.e)?;

    PKey::from_rsa_public_components(n, e)
  }

  #[allow(clippy::many_single_char_names)]
  pub fn private_from_jwk(data: &Jwk) -> Result<PKey<Secret>> {
    data.check_kty(JwkType::Rsa)?;

    let params: &JwkParamsRsa = match data.params() {
      JwkParams::Rsa(params) => params,
      _ => return Err(Error::KeyError("Rsa")),
    };

    // TODO: Handle Multi-prime keys
    if params.oth.is_some() {
      return Err(Error::KeyError("Rsa"));
    }

    let n: rsa::BigUint = decode_b64_biguint(&params.n)?;
    let e: rsa::BigUint = decode_b64_biguint(&params.e)?;
    let d: rsa::BigUint = decode_b64_biguint_opt(params.d.as_deref())?;
    let p: rsa::BigUint = decode_b64_biguint_opt(params.p.as_deref())?;
    let q: rsa::BigUint = decode_b64_biguint_opt(params.q.as_deref())?;

    let _dp: rsa::BigUint = decode_b64_biguint_opt(params.dp.as_deref())?;
    let _dq: rsa::BigUint = decode_b64_biguint_opt(params.dq.as_deref())?;
    let _qi: rsa::BigUint = decode_b64_biguint_opt(params.qi.as_deref())?;

    PKey::from_rsa_secret_components(n, e, d, vec![p, q])
  }

  pub fn random(bits: RsaBits) -> Result<Self> {
    Ok(Self {
      key: PKey::generate_rsa(bits)?,
      alg: None,
      kid: None,
    })
  }

  pub fn from_jwk(data: &Jwk) -> Result<Self> {
    let key: PKey<Secret> = Self::private_from_jwk(data)?;

    Ok(Self {
      key,
      alg: data.alg().map(ToString::to_string),
      kid: data.kid().map(ToString::to_string),
    })
  }

  pub fn to_jwk(&self, private: bool) -> Result<Jwk> {
    let mut jwk: Jwk = Jwk::new(JwkType::Rsa);
    let mut params: JwkParamsRsa = JwkParamsRsa::new();

    let key: PKey<Public> = self.key.rsa_public_key()?;
    let (n, e): _ = key.to_rsa_public_components()?;

    params.n = encode_b64_biguint(n);
    params.e = encode_b64_biguint(e);

    if private {
      let (d, primes): _ = self.key.to_rsa_secret_components()?;

      // TODO: Handle Multi-prime keys
      if primes.len() != 2 {
        return Err(Error::KeyError("Rsa"));
      }

      params.d = Some(encode_b64_biguint(d));
      params.p = Some(encode_b64_biguint(&primes[0]));
      params.q = Some(encode_b64_biguint(&primes[1]));

      // TODO: Add dp/dp/qi
    }

    jwk.set_use(JwkUse::Signature);
    jwk.set_params(params);

    Ok(jwk)
  }
}

fn encode_b64_biguint(data: &rsa::BigUint) -> String {
  encode_b64(data.to_bytes_be())
}

fn decode_b64_biguint(data: impl AsRef<[u8]>) -> Result<rsa::BigUint> {
  decode_b64(data.as_ref()).map(|data| rsa::BigUint::from_bytes_be(&data))
}

fn decode_b64_biguint_opt(data: Option<impl AsRef<[u8]>>) -> Result<rsa::BigUint> {
  data
    .as_ref()
    .ok_or(Error::KeyError("Rsa"))
    .and_then(decode_b64_biguint)
}
