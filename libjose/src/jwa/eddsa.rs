use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crypto::key_box::PublicKey;
use crypto::key_box::SecretKey;
use crate::error::Result;
use crate::error::CryptoError;
use crate::jwa::EdCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkUse;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::utils::pem_decode;
use crate::utils::Pem;

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

  /// Creates a new `EddsaSigner` from DER-encoded material in PKCS#8 form.
  pub fn signer_from_der(self, data: impl AsRef<[u8]>) -> Result<EddsaSigner> {
    // TODO: Parse and validate key format
    let key: SecretKey = data.as_ref().into();

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

    // TODO: Parse and validate key format
    let key: SecretKey = match pem.pem_type.as_str() {
      "PRIVATE KEY" => pem.pem_data.into(),
      _ => return Err(CryptoError::InvalidKeyFormat(self.name()).into()),
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

    let (key, crv): (SecretKey, EdCurve) = todo!("EddsaAlgorithm::signer_from_jwk");
    let kid: Option<String> = data.kid().map(ToString::to_string);

    Ok(EddsaSigner {
      alg: self,
      crv,
      key,
      kid,
    })
  }

  /// Creates a new `EddsaVerifier` from DER-encoded material in PKCS#8 form.
  pub fn verifier_from_der(self, data: impl AsRef<[u8]>) -> Result<EddsaVerifier> {
    // TODO: Parse and validate key format
    let key: PublicKey = data.as_ref().into();

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

    // TODO: Parse and validate key format
    let key: PublicKey = match pem.pem_type.as_str() {
      "PUBLIC KEY" => pem.pem_data.into(),
      _ => return Err(CryptoError::InvalidKeyFormat(self.name()).into()),
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

    let key: PublicKey = todo!("EddsaAlgorithm::verifier_from_jwk");
    let kid: Option<String> = data.kid().map(ToString::to_string);

    Ok(EddsaVerifier {
      alg: self,
      crv: EdCurve::Ed25519,
      key,
      kid,
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
  key: SecretKey,
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
    match self.crv {
      EdCurve::Ed25519 => todo!("EddsaSigner::sign(Ed25519)"),
      EdCurve::Ed448 => todo!("EddsaSigner::sign(Ed448)"),
    }
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
  key: PublicKey,
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
    match self.crv {
      EdCurve::Ed25519 => todo!("EddsaVerifier::verify(Ed25519)"),
      EdCurve::Ed448 => todo!("EddsaVerifier::verify(Ed448)"),
    }
  }
}

impl Deref for EddsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
