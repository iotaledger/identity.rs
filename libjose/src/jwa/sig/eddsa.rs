use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::error::Result;
use crate::jwa::EdCurve;
use crate::jwa::EdKeyPair;
use crate::jwa::PKey;
use crate::jwa::PKeyExt as _;
use crate::jwa::Public;
use crate::jwa::Secret;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkUse;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::lib::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum EddsaAlgorithm {
  /// EdDSA signature algorithm
  EdDSA,
}

impl EddsaAlgorithm {
  /// Returns the JWA identifier of the EdDSA algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::EdDSA => "EdDSA",
    }
  }

  /// Creates a new cryptographically random private key.
  pub fn generate_key(self, curve: EdCurve) -> Result<EdKeyPair> {
    EdKeyPair::random(curve)
  }

  /// Creates a new `EddsaSigner` from a slice of bytes.
  pub fn signer_from_bytes(self, curve: EdCurve, data: impl AsRef<[u8]>) -> Result<EddsaSigner> {
    Ok(EddsaSigner {
      alg: self,
      crv: curve,
      key: PKey::from_ed_bytes(curve, data)?,
      kid: None,
    })
  }

  /// Creates a new `EddsaSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<EddsaSigner> {
    data.check_use(JwkUse::Signature)?;
    data.check_ops(JwkOperation::Sign)?;
    data.check_alg(self.name())?;

    let key: EdKeyPair = EdKeyPair::from_jwk(data)?;

    Ok(EddsaSigner {
      alg: self,
      crv: key.crv,
      key: key.key,
      kid: key.kid,
    })
  }

  /// Creates a new `EddsaVerifier` from a slice of bytes.
  pub fn verifier_from_bytes(
    self,
    curve: EdCurve,
    data: impl AsRef<[u8]>,
  ) -> Result<EddsaVerifier> {
    Ok(EddsaVerifier {
      alg: self,
      crv: curve,
      key: PKey::from_ed_bytes(curve, data)?,
      kid: None,
    })
  }

  /// Creates a new `EddsaVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<EddsaVerifier> {
    data.check_use(JwkUse::Signature)?;
    data.check_ops(JwkOperation::Verify)?;
    data.check_alg(self.name())?;

    let (key, crv): (PKey<Public>, EdCurve) = EdKeyPair::public_from_jwk(data)?;

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
    f.write_str(self.name())
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
// EdDSA Signer
// =============================================================================

#[derive(Debug)]
pub struct EddsaSigner {
  alg: EddsaAlgorithm,
  crv: EdCurve,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl EddsaSigner {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JwsSigner for EddsaSigner {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    self.key.try_ed_sign(self.crv, message)
  }
}

impl Deref for EddsaSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// EdDSA Verifier
// =============================================================================

#[derive(Debug)]
pub struct EddsaVerifier {
  alg: EddsaAlgorithm,
  crv: EdCurve,
  key: PKey<Public>,
  kid: Option<String>,
}

impl EddsaVerifier {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JwsVerifier for EddsaVerifier {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    self.key.try_ed_verify(self.crv, message, signature)
  }
}

impl Deref for EddsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
