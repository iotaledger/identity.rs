use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use crypto::rand::OsRng;
use crypto::signers::eddsa;
use crypto::signers::eddsa::Curve;

use crate::error::Error;
use crate::error::Result;
use crate::jwa::EdCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkUse;
use crate::jwk::JwkParamsOkp;
use crate::jwk::JwkParams;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::utils::pem_decode;
use crate::utils::Pem;
use crate::utils::decode_b64;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum EddsaAlgorithm {
  /// EdDSA signature algorithm
  EdDSA,
}

impl EddsaAlgorithm {
  /// Returns the JWA identifier of the algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::EdDSA => "EdDSA",
    }
  }

  /// Creates a new random EdDSA `PrivateKey`.
  pub fn generate_key(self, curve: EdCurve) -> Result<eddsa::PrivateKey> {
    eddsa::PrivateKey::random(curve.into(), &mut OsRng).map_err(Into::into)
  }

  /// Creates a new `EddsaSigner` from DER-encoded material in PKCS#8 form.
  pub fn signer_from_der(self, data: impl AsRef<[u8]>) -> Result<EddsaSigner> {
    let key: eddsa::PrivateKey = eddsa::PrivateKey::from_slice(Curve::Ed25519, data)?;

    Ok(EddsaSigner {
      alg: self,
      crv: EdCurve::Ed25519,
      key,
      kid: None,
    })
  }

  /// Creates a new `EddsaSigner` from a PEM-encoded document.
  pub fn signer_from_pem(self, data: impl AsRef<[u8]>) -> Result<EddsaSigner> {
    let pem: Pem = pem_decode(&data)?;

    let key: eddsa::PrivateKey = match pem.pem_type.as_str() {
      "PRIVATE KEY" => eddsa::PrivateKey::from_slice(Curve::Ed25519, pem.pem_data)?,
      _ => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    Ok(EddsaSigner {
      alg: self,
      crv: EdCurve::Ed25519,
      key,
      kid: None,
    })
  }

  /// Creates a new `EddsaSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<EddsaSigner> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Sign)?;
    data.check_alg(self.name())?;

    let params: &JwkParamsOkp = match data.params() {
      Some(JwkParams::Okp(params)) => params,
      Some(_) | None => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    let crv: EdCurve = match params.crv.as_str() {
      "Ed25519" => EdCurve::Ed25519,
      "Ed448" => EdCurve::Ed448,
      _ => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    let x: Vec<u8> = decode_b64(params.x.as_str())?;

    let d: Vec<u8> = match params.d.as_ref() {
      Some(d) =>  decode_b64(d.as_str())?,
      None => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    let key: eddsa::PrivateKey = eddsa::PrivateKey::from_slice(crv.into(), &d)?;

    if key.public_key() != eddsa::PublicKey::from_slice(crv.into(), &x)? {
      return Err(Error::InvalidKeyFormat(self.name()).into());
    }

    Ok(EddsaSigner {
      alg: self,
      crv,
      key,
      kid: data.kid().map(ToString::to_string),
    })
  }

  /// Creates a new `EddsaVerifier` from DER-encoded material in PKCS#8 form.
  pub fn verifier_from_der(self, data: impl AsRef<[u8]>) -> Result<EddsaVerifier> {
    let key: eddsa::PublicKey = eddsa::PublicKey::from_slice(Curve::Ed25519, data)?;

    Ok(EddsaVerifier {
      alg: self,
      crv: EdCurve::Ed25519,
      key,
      kid: None,
    })
  }

  /// Creates a new `EddsaVerifier` from a PEM-encoded document.
  pub fn verifier_from_pem(self, data: impl AsRef<[u8]>) -> Result<EddsaVerifier> {
    let pem: Pem = pem_decode(&data)?;

    let key: eddsa::PublicKey = match pem.pem_type.as_str() {
      "PUBLIC KEY" => eddsa::PublicKey::from_slice(Curve::Ed25519, pem.pem_data)?,
      _ => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    Ok(EddsaVerifier {
      alg: self,
      crv: EdCurve::Ed25519,
      key,
      kid: None,
    })
  }

  /// Creates a new `EddsaVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<EddsaVerifier> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Verify)?;
    data.check_alg(self.name())?;

    let params: &JwkParamsOkp = match data.params() {
      Some(JwkParams::Okp(params)) => params,
      Some(_) | None => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    let crv: EdCurve = match params.crv.as_str() {
      "Ed25519" => EdCurve::Ed25519,
      "Ed448" => EdCurve::Ed448,
      _ => return Err(Error::InvalidKeyFormat(self.name()).into()),
    };

    let x: Vec<u8> = decode_b64(params.x.as_str())?;

    let key: eddsa::PublicKey = eddsa::PublicKey::from_slice(crv.into(), &x)?;

    Ok(EddsaVerifier {
      alg: self,
      crv,
      key,
      kid: data.kid().map(ToString::to_string),
    })
  }
}

impl Display for EddsaAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl From<EddsaAlgorithm> for JwsAlgorithm {
  fn from(other: EddsaAlgorithm) -> Self {
    match other {
      EddsaAlgorithm::EdDSA => Self::EdDSA,
    }
  }
}

// =============================================================================
// Eddsa Signer
// =============================================================================

#[derive(Debug)]
pub struct EddsaSigner {
  alg: EddsaAlgorithm,
  crv: EdCurve,
  key: eddsa::PrivateKey,
  kid: Option<String>,
}

impl JwsSigner for EddsaSigner {
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

impl Deref for EddsaSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// Eddsa Verifier
// =============================================================================

#[derive(Debug)]
pub struct EddsaVerifier {
  alg: EddsaAlgorithm,
  crv: EdCurve,
  key: eddsa::PublicKey,
  kid: Option<String>,
}

impl JwsVerifier for EddsaVerifier {
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

impl Deref for EddsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
