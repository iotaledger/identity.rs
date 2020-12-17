use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use crate::error::Error;
use crate::error::Result;
use crate::jwa::EcCurve;
use crate::jwa::EcKeyPair;
use crate::jwa::EcxCurve;
use crate::jwa::EcxKeyPair;
use crate::jwa::PKey;
use crate::jwa::PKeyExt as _;
use crate::jwa::Public;
use crate::jwa::Secret;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkParams;
use crate::jwk::JwkType;
use crate::jwk::JwkUse;
use crate::lib::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EcdhKeyType {
  Ec(EcCurve),
  Ecx(EcxCurve),
}

impl EcdhKeyType {
  pub const fn kty(&self) -> JwkType {
    match self {
      Self::Ec(_) => JwkType::Ec,
      Self::Ecx(_) => JwkType::Okp,
    }
  }

  pub const fn crv(&self) -> &'static str {
    match self {
      Self::Ec(inner) => inner.name(),
      Self::Ecx(inner) => inner.name(),
    }
  }

  pub fn generate_epk(&self) -> Result<(Jwk, PKey<Secret>)> {
    match self {
      Self::Ec(curve) => {
        let keypair: EcKeyPair = EcKeyPair::random(*curve)?;
        let public: Jwk = keypair.to_jwk(false)?;

        Ok((public, keypair.key))
      }
      Self::Ecx(curve) => {
        let keypair: EcxKeyPair = EcxKeyPair::random(*curve)?;
        let public: Jwk = keypair.to_jwk(false)?;

        Ok((public, keypair.key))
      }
    }
  }

  pub fn expand_epk(&self, epk: &Jwk) -> Result<PKey<Public>> {
    if epk.kty() != self.kty() {
      return Err(Error::KeyError("EPK Key Type"));
    }

    match self {
      Self::Ec(curve) => {
        let (key, crv): (PKey<Public>, EcCurve) = EcKeyPair::public_from_jwk(epk)?;

        if crv != *curve {
          return Err(Error::KeyError("EPK Curve"));
        }

        Ok(key)
      }
      Self::Ecx(curve) => {
        let (key, crv): (PKey<Public>, EcxCurve) = EcxKeyPair::public_from_jwk(epk)?;

        if crv != *curve {
          return Err(Error::KeyError("EPK Curve"));
        }

        Ok(key)
      }
    }
  }

  pub fn public_from_bytes(&self, data: impl AsRef<[u8]>) -> Result<PKey<Public>> {
    match self {
      Self::Ec(curve) => PKey::from_ec_bytes(*curve, data),
      Self::Ecx(curve) => PKey::from_ecx_bytes(*curve, data),
    }
  }

  pub fn public_from_jwk(algorithm: &'static str, data: &Jwk) -> Result<(PKey<Public>, Self)> {
    data.check_alg(algorithm)?;
    data.check_use(JwkUse::Encryption)?;
    data.check_ops(JwkOperation::DeriveKey)?;
    data.check_ktys(&[JwkType::Ec, JwkType::Oct])?;

    match data.params() {
      JwkParams::Ec(_) => EcKeyPair::public_from_jwk(data).map(|(key, crv)| (key, Self::Ec(crv))),
      JwkParams::Okp(_) => {
        EcxKeyPair::public_from_jwk(data).map(|(key, crv)| (key, Self::Ecx(crv)))
      }
      _ => Err(Error::KeyError(algorithm)),
    }
  }

  pub fn private_from_bytes(&self, data: impl AsRef<[u8]>) -> Result<PKey<Secret>> {
    match self {
      Self::Ec(curve) => PKey::from_ec_bytes(*curve, data),
      Self::Ecx(curve) => PKey::from_ecx_bytes(*curve, data),
    }
  }

  pub fn private_from_jwk(algorithm: &'static str, data: &Jwk) -> Result<(PKey<Secret>, Self)> {
    data.check_use(JwkUse::Encryption)?;
    data.check_ops(JwkOperation::DeriveKey)?;
    data.check_alg(algorithm)?;
    data.check_ktys(&[JwkType::Ec, JwkType::Oct])?;

    match data.params() {
      JwkParams::Ec(_) => EcKeyPair::private_from_jwk(data).map(|(key, crv)| (key, Self::Ec(crv))),
      JwkParams::Okp(_) => {
        EcxKeyPair::private_from_jwk(data).map(|(key, crv)| (key, Self::Ecx(crv)))
      }
      _ => Err(Error::KeyError(algorithm)),
    }
  }

  pub fn diffie_hellman(&self, secret: &PKey<Secret>, public: &PKey<Public>) -> Result<Vec<u8>> {
    match self {
      Self::Ec(curve) => secret.ec_diffie_hellman(*curve, public),
      Self::Ecx(curve) => secret.ecx_diffie_hellman(*curve, public),
    }
  }
}

impl Display for EcdhKeyType {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}({})", self.kty(), self.crv()))
  }
}

impl From<EcCurve> for EcdhKeyType {
  fn from(other: EcCurve) -> Self {
    Self::Ec(other)
  }
}

impl From<EcxCurve> for EcdhKeyType {
  fn from(other: EcxCurve) -> Self {
    Self::Ecx(other)
  }
}
