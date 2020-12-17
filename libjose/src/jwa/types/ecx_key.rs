use crate::error::Error;
use crate::error::Result;
use crate::jwa::EcxCurve;
use crate::jwa::PKey;
use crate::jwa::PKeyExt as _;
use crate::jwa::Public;
use crate::jwa::Secret;
use crate::jwk::Jwk;
use crate::jwk::JwkParams;
use crate::jwk::JwkParamsOkp;
use crate::jwk::JwkType;
use crate::jwk::JwkUse;
use crate::lib::*;
use crate::utils::decode_b64;
use crate::utils::encode_b64;

#[derive(Debug)]
pub struct EcxKeyPair {
  pub(crate) crv: EcxCurve,
  pub(crate) key: PKey<Secret>,
  pub(crate) alg: Option<String>,
  pub(crate) kid: Option<String>,
}

impl EcxKeyPair {
  pub fn public_from_jwk(data: &Jwk) -> Result<(PKey<Public>, EcxCurve)> {
    data.check_kty(JwkType::Okp)?;

    let params: &JwkParamsOkp = match data.params() {
      JwkParams::Okp(params) => params,
      _ => return Err(Error::KeyError("ECX")),
    };

    let crv: EcxCurve = __curve(&params)?;
    let key: PKey<Public> = __public(&params, crv)?;

    Ok((key, crv))
  }

  pub fn private_from_jwk(data: &Jwk) -> Result<(PKey<Secret>, EcxCurve)> {
    data.check_kty(JwkType::Okp)?;

    let params: &JwkParamsOkp = match data.params() {
      JwkParams::Okp(params) => params,
      _ => return Err(Error::KeyError("ECX")),
    };

    let crv: EcxCurve = __curve(&params)?;
    let key: PKey<Secret> = __private(&params, crv)?;

    Ok((key, crv))
  }

  pub fn random(curve: EcxCurve) -> Result<Self> {
    Ok(Self {
      crv: curve,
      key: PKey::generate_ecx(curve)?,
      alg: None,
      kid: None,
    })
  }

  pub fn from_jwk(data: &Jwk) -> Result<Self> {
    let (key, crv): (PKey<Secret>, EcxCurve) = Self::private_from_jwk(data)?;

    Ok(Self {
      crv,
      key,
      alg: data.alg().map(ToString::to_string),
      kid: data.kid().map(ToString::to_string),
    })
  }

  pub fn to_jwk(&self, private: bool) -> Result<Jwk> {
    let mut jwk: Jwk = Jwk::new(JwkType::Okp);
    let mut params: JwkParamsOkp = JwkParamsOkp::new();

    let key: PKey<Public> = self.key.ecx_public_key(self.crv)?;

    params.crv = self.crv.name().to_string();
    params.x = encode_b64(key.to_ecx_bytes(self.crv)?);

    if private {
      params.d = Some(encode_b64(self.key.to_ecx_bytes(self.crv)?));
    }

    jwk.set_use(JwkUse::Encryption);
    jwk.set_params(params);

    Ok(jwk)
  }
}

fn __curve(params: &JwkParamsOkp) -> Result<EcxCurve> {
  match &*params.crv {
    "X25519" => Ok(EcxCurve::X25519),
    "X448" => Ok(EcxCurve::X448),
    _ => Err(Error::KeyError("ECX")),
  }
}

fn __public(params: &JwkParamsOkp, crv: EcxCurve) -> Result<PKey<Public>> {
  decode_b64(&params.x).and_then(|x| PKey::from_ecx_bytes(crv, &x))
}

fn __private(params: &JwkParamsOkp, crv: EcxCurve) -> Result<PKey<Secret>> {
  params
    .d
    .as_ref()
    .map(decode_b64)
    .transpose()?
    .ok_or(Error::KeyError("ECX"))
    .and_then(|d| PKey::from_ecx_bytes(crv, &d))
}
