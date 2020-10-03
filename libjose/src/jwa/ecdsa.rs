use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use crypto::rand::OsRng;
use crypto::signers::ecdsa;

use crate::error::Error;
use crate::error::Result;
use crate::jwa::EcCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkParams;
use crate::jwk::JwkParamsEc;
use crate::jwk::JwkUse;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::utils::decode_b64;
use crate::utils::pem_decode;
use crate::utils::Pem;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum EcdsaAlgorithm {
  /// ECDSA using P-256 and SHA-256
  ES256,
  /// ECDSA using P-384 and SHA-384
  ES384,
  /// ECDSA using P-521 and SHA-512
  ES512,
  /// ECDSA using secp256k1 curve and SHA-256
  ES256K,
}

impl EcdsaAlgorithm {
  /// Returns the JWA identifier of the algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::ES256 => "ES256",
      Self::ES384 => "ES384",
      Self::ES512 => "ES512",
      Self::ES256K => "ES256K",
    }
  }

  /// Returns the curve of the algorithm.
  pub const fn curve(self) -> EcCurve {
    match self {
      Self::ES256 => EcCurve::P256,
      Self::ES384 => EcCurve::P384,
      Self::ES512 => EcCurve::P521,
      Self::ES256K => EcCurve::Secp256K1,
    }
  }

  /// Creates a new random ECDSA `PrivateKey`.
  pub fn generate_key(self) -> Result<ecdsa::PrivateKey> {
    Ok(ecdsa::PrivateKey::random(self.curve().into(), OsRng))
  }

  /// Creates a new `EcdsaSigner` from DER-encoded material in PKCS#8 form.
  pub fn signer_from_der(self, data: impl AsRef<[u8]>) -> Result<EcdsaSigner> {
    let key: ecdsa::PrivateKey = ecdsa::PrivateKey::from_slice(self.curve().into(), data)?;

    Ok(EcdsaSigner {
      alg: self,
      key,
      kid: None,
    })
  }

  /// Creates a new `EcdsaSigner` from a PEM-encoded document.
  pub fn signer_from_pem(self, data: impl AsRef<[u8]>) -> Result<EcdsaSigner> {
    let pem: Pem = pem_decode(&data)?;

    let key: ecdsa::PrivateKey = match pem.pem_type.as_str() {
      "PRIVATE KEY" => ecdsa::PrivateKey::from_slice(self.curve().into(), pem.pem_data)?,
      _ => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    Ok(EcdsaSigner {
      alg: self,
      key,
      kid: None,
    })
  }

  /// Creates a new `EcdsaSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<EcdsaSigner> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Sign)?;
    data.check_alg(self.name())?;

    let params: &JwkParamsEc = match data.params() {
      Some(JwkParams::Ec(params)) => params,
      Some(_) | None => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    if params.crv != self.curve().name() {
      return Err(Error::InvalidKeyFormat(self.name()).into());
    }

    let x: Vec<u8> = decode_b64(params.x.as_str())?;
    let y: Vec<u8> = decode_b64(params.y.as_str())?;

    let d: Vec<u8> = match params.d.as_deref() {
      Some(d) => decode_b64(d)?,
      None => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    let key: ecdsa::PrivateKey = ecdsa::PrivateKey::from_slice(self.curve().into(), &d)?;

    if key.public_key() != ecdsa::PublicKey::from_coord(self.curve().into(), x, y)? {
      return Err(Error::InvalidKeyFormat(self.name()).into());
    }

    Ok(EcdsaSigner {
      alg: self,
      key,
      kid: data.kid().map(ToString::to_string),
    })
  }

  /// Creates a new `EcdsaVerifier` from DER-encoded material in PKCS#8 form.
  pub fn verifier_from_der(self, data: impl AsRef<[u8]>) -> Result<EcdsaVerifier> {
    let key: ecdsa::PublicKey = ecdsa::PublicKey::from_slice(self.curve().into(), data)?;

    Ok(EcdsaVerifier {
      alg: self,
      key,
      kid: None,
    })
  }

  /// Creates a new `EcdsaVerifier` from a PEM-encoded document.
  pub fn verifier_from_pem(self, data: impl AsRef<[u8]>) -> Result<EcdsaVerifier> {
    let pem: Pem = pem_decode(&data)?;

    let key: ecdsa::PublicKey = match pem.pem_type.as_str() {
      "PUBLIC KEY" => ecdsa::PublicKey::from_slice(self.curve().into(), pem.pem_data)?,
      _ => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    Ok(EcdsaVerifier {
      alg: self,
      key,
      kid: None,
    })
  }

  /// Creates a new `EcdsaVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<EcdsaVerifier> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Verify)?;
    data.check_alg(self.name())?;

    let params: &JwkParamsEc = match data.params() {
      Some(JwkParams::Ec(params)) => params,
      Some(_) | None => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    if params.crv != self.curve().name() {
      return Err(Error::InvalidKeyFormat(self.name()).into());
    }

    let x: Vec<u8> = decode_b64(params.x.as_str())?;
    let y: Vec<u8> = decode_b64(params.y.as_str())?;

    let key: ecdsa::PublicKey = ecdsa::PublicKey::from_coord(self.curve().into(), x, y)?;

    Ok(EcdsaVerifier {
      alg: self,
      key,
      kid: data.kid().map(ToString::to_string),
    })
  }
}

impl Display for EcdsaAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl From<EcdsaAlgorithm> for JwsAlgorithm {
  fn from(other: EcdsaAlgorithm) -> Self {
    match other {
      EcdsaAlgorithm::ES256 => Self::ES256,
      EcdsaAlgorithm::ES384 => Self::ES384,
      EcdsaAlgorithm::ES512 => Self::ES512,
      EcdsaAlgorithm::ES256K => Self::ES256K,
    }
  }
}

#[derive(Debug)]
pub struct EcdsaSigner {
  alg: EcdsaAlgorithm,
  key: ecdsa::PrivateKey,
  kid: Option<String>,
}

impl JwsSigner for EcdsaSigner {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    self
      .key
      .sign(message)
      .map_err(Into::into)
      .map(|signature| signature.as_ref().to_vec())
  }
}

impl Deref for EcdsaSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

#[derive(Debug)]
pub struct EcdsaVerifier {
  alg: EcdsaAlgorithm,
  key: ecdsa::PublicKey,
  kid: Option<String>,
}

impl JwsVerifier for EcdsaVerifier {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    self.key.verify(message, signature).map_err(Into::into)
  }
}

impl Deref for EcdsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
