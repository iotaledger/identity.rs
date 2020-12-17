use crate::error::Error;
use crate::error::Result;
use crate::jwa::EcCurve;
use crate::jwa::PKey;
use crate::jwa::PKeyExt as _;
use crate::jwa::Public;
use crate::jwa::Secret;
use crate::jwk::Jwk;
use crate::jwk::JwkParams;
use crate::jwk::JwkParamsEc;
use crate::jwk::JwkType;
use crate::lib::*;
use crate::utils::decode_b64;
use crate::utils::encode_b64;

#[derive(Debug)]
pub struct EcKeyPair {
  pub(crate) crv: EcCurve,
  pub(crate) key: PKey<Secret>,
  pub(crate) alg: Option<String>,
  pub(crate) kid: Option<String>,
}

impl EcKeyPair {
  pub fn public_from_jwk(data: &Jwk) -> Result<(PKey<Public>, EcCurve)> {
    data.check_kty(JwkType::Ec)?;

    let params: &JwkParamsEc = match data.params() {
      JwkParams::Ec(params) => params,
      _ => return Err(Error::KeyError("EC")),
    };

    let crv: EcCurve = __curve(&params)?;
    let key: PKey<Public> = __public(&params, crv)?;

    Ok((key, crv))
  }

  pub fn private_from_jwk(data: &Jwk) -> Result<(PKey<Secret>, EcCurve)> {
    data.check_kty(JwkType::Ec)?;

    let params: &JwkParamsEc = match data.params() {
      JwkParams::Ec(params) => params,
      _ => return Err(Error::KeyError("EC")),
    };

    let crv: EcCurve = __curve(&params)?;
    let key: PKey<Secret> = __private(&params, crv)?;

    Ok((key, crv))
  }

  pub fn random(curve: EcCurve) -> Result<Self> {
    Ok(Self {
      crv: curve,
      key: PKey::generate_ec(curve)?,
      alg: None,
      kid: None,
    })
  }

  pub fn from_jwk(data: &Jwk) -> Result<Self> {
    let (key, crv): (PKey<Secret>, EcCurve) = Self::private_from_jwk(data)?;

    Ok(Self {
      crv,
      key,
      alg: data.alg().map(ToString::to_string),
      kid: data.kid().map(ToString::to_string),
    })
  }

  pub fn to_jwk(&self, private: bool) -> Result<Jwk> {
    let mut jwk: Jwk = Jwk::new(JwkType::Ec);
    let mut params: JwkParamsEc = JwkParamsEc::new();

    let (x, y): (Vec<u8>, Vec<u8>) = self.key.ec_public_key(self.crv)?.to_ec_coord(self.crv)?;

    params.crv = self.crv.name().to_string();
    params.x = encode_b64(x);
    params.y = encode_b64(y);

    if private {
      params.d = Some(encode_b64(self.key.to_ec_bytes(self.crv)?));
    }

    jwk.set_params(params);

    Ok(jwk)
  }
}

fn __curve(params: &JwkParamsEc) -> Result<EcCurve> {
  match &*params.crv {
    "P-256" => Ok(EcCurve::P256),
    "P-384" => Ok(EcCurve::P384),
    "P-521" => Ok(EcCurve::P521),
    "secp256k1" => Ok(EcCurve::Secp256K1),
    _ => Err(Error::KeyError("EC")),
  }
}

fn __public(params: &JwkParamsEc, crv: EcCurve) -> Result<PKey<Public>> {
  let x: Vec<u8> = decode_b64(&params.x)?;
  let y: Vec<u8> = decode_b64(&params.y)?;

  PKey::from_ec_coord(crv, &x, &y)
}

fn __private(params: &JwkParamsEc, crv: EcCurve) -> Result<PKey<Secret>> {
  params
    .d
    .as_ref()
    .map(decode_b64)
    .transpose()?
    .ok_or(Error::KeyError("EC"))
    .and_then(|d| PKey::from_ec_bytes(crv, &d))
}
