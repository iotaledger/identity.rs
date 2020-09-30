use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::crypto::ecdsa_sign;
use crate::crypto::ecdsa_verify;
use crate::crypto::PKey;
use crate::crypto::Public;
use crate::crypto::Secret;
use crate::error::Result;
use crate::jwa::EcCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkUse;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::utils::pem_decode;

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

  /// Creates a new `EcdsaSigner` from DER-encoded material in PKCS#8 form.
  pub fn signer_from_der(self, data: impl AsRef<[u8]>) -> Result<EcdsaSigner> {
    // TODO: Parse and validate key format
    Ok(EcdsaSigner {
      alg: self,
      key: data.as_ref().into(),
      kid: None,
    })
  }

  /// Creates a new `EcdsaSigner` from a PEM-encoded document.
  pub fn signer_from_pem(self, data: impl AsRef<[u8]>) -> Result<EcdsaSigner> {
    // TODO: Parse and validate key format
    Ok(EcdsaSigner {
      alg: self,
      key: pem_decode(&data).map(|pem| pem.pem_data.into())?,
      kid: None,
    })
  }

  /// Creates a new `EcdsaSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<EcdsaSigner> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Sign)?;
    data.check_alg(self.name())?;

    let key: PKey<Secret> = todo!("EcdsaAlgorithm::signer_from_jwk");
    let kid: Option<String> = data.kid().map(ToString::to_string);

    Ok(EcdsaSigner {
      alg: self,
      key,
      kid,
    })
  }

  /// Creates a new `EcdsaVerifier` from DER-encoded material in PKCS#8 form.
  pub fn verifier_from_der(self, data: impl AsRef<[u8]>) -> Result<EcdsaVerifier> {
    // TODO: Parse and validate key format
    Ok(EcdsaVerifier {
      alg: self,
      key: data.as_ref().into(),
      kid: None,
    })
  }

  /// Creates a new `EcdsaVerifier` from a PEM-encoded document.
  pub fn verifier_from_pem(self, data: impl AsRef<[u8]>) -> Result<EcdsaVerifier> {
    // TODO: Parse and validate key format
    Ok(EcdsaVerifier {
      alg: self,
      key: pem_decode(&data).map(|pem| pem.pem_data.into())?,
      kid: None,
    })
  }

  /// Creates a new `EcdsaVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<EcdsaVerifier> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Verify)?;
    data.check_alg(self.name())?;

    let key: PKey<Public> = todo!("EcdsaAlgorithm::verifier_from_jwk");
    let kid: Option<String> = data.kid().map(ToString::to_string);

    Ok(EcdsaVerifier {
      alg: self,
      key,
      kid,
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EcdsaSigner {
  alg: EcdsaAlgorithm,
  key: PKey<Secret>,
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
    ecdsa_sign(self.alg, message, &self.key)
  }
}

impl Deref for EcdsaSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EcdsaVerifier {
  alg: EcdsaAlgorithm,
  key: PKey<Public>,
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
    ecdsa_verify(self.alg, message, signature, &self.key)
  }
}

impl Deref for EcdsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
