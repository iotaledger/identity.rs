use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::error::Error;
use crate::error::Result;
use crate::jwa::EcCurve;
use crate::jwa::EcKeyPair;
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
  /// Returns the JWA identifier of the ECDSA algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::ES256 => "ES256",
      Self::ES384 => "ES384",
      Self::ES512 => "ES512",
      Self::ES256K => "ES256K",
    }
  }

  /// Returns the curve of the ECDSA algorithm.
  pub const fn curve(self) -> EcCurve {
    match self {
      Self::ES256 => EcCurve::P256,
      Self::ES384 => EcCurve::P384,
      Self::ES512 => EcCurve::P521,
      Self::ES256K => EcCurve::Secp256K1,
    }
  }

  /// Creates a new cryptographically random private key.
  pub fn generate_key(self) -> Result<EcKeyPair> {
    EcKeyPair::random(self.curve())
  }

  /// Creates a new `EcdsaSigner` from a raw scalar value (big endian).
  pub fn signer_from_bytes(self, data: impl AsRef<[u8]>) -> Result<EcdsaSigner> {
    Ok(EcdsaSigner {
      alg: self,
      key: PKey::from_ec_bytes(self.curve(), data)?,
      kid: None,
    })
  }

  /// Creates a new `EcdsaSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<EcdsaSigner> {
    data.check_use(JwkUse::Signature)?;
    data.check_ops(JwkOperation::Sign)?;
    data.check_alg(self.name())?;

    let key: EcKeyPair = EcKeyPair::from_jwk(data)?;

    if key.crv != self.curve() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(EcdsaSigner {
      alg: self,
      key: key.key,
      kid: key.kid,
    })
  }

  /// Creates a new `EcdsaVerifier` from an SEC1-encoded public key.
  pub fn verifier_from_bytes(self, data: impl AsRef<[u8]>) -> Result<EcdsaVerifier> {
    Ok(EcdsaVerifier {
      alg: self,
      key: PKey::from_ec_bytes(self.curve(), data)?,
      kid: None,
    })
  }

  /// Creates a new `EcdsaVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<EcdsaVerifier> {
    data.check_use(JwkUse::Signature)?;
    data.check_ops(JwkOperation::Verify)?;
    data.check_alg(self.name())?;

    let (key, crv): (PKey<Public>, EcCurve) = EcKeyPair::public_from_jwk(data)?;

    if crv != self.curve() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(EcdsaVerifier {
      alg: self,
      key,
      kid: data.kid().map(ToString::to_string),
    })
  }
}

impl Display for EcdsaAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
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

// =============================================================================
// ECDSA Signer
// =============================================================================

#[derive(Debug)]
pub struct EcdsaSigner {
  alg: EcdsaAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl EcdsaSigner {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JwsSigner for EcdsaSigner {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    self.key.try_ec_sign(self.alg.curve(), message)
  }
}

impl Deref for EcdsaSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// ECDSA Verifier
// =============================================================================

#[derive(Debug)]
pub struct EcdsaVerifier {
  alg: EcdsaAlgorithm,
  key: PKey<Public>,
  kid: Option<String>,
}

impl EcdsaVerifier {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JwsVerifier for EcdsaVerifier {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    self.key.try_ec_verify(self.alg.curve(), message, signature)
  }
}

impl Deref for EcdsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_works() {
//     let signer = ES256.signer_from_bytes(keypair.secret())?;
//     let verifier = ES256.verifier_from_bytes(keypair.public())?;

//     let signer = ES384.signer_from_bytes(keypair.secret())?;
//     let verifier = ES384.verifier_from_bytes(keypair.public())?;

//     let signer = ES512.signer_from_bytes(keypair.secret())?;
//     let verifier = ES512.verifier_from_bytes(keypair.public())?;

//     let signer = ES256K.signer_from_bytes(keypair.secret())?;
//     let verifier = ES256K.verifier_from_bytes(keypair.public())?;

//   }
// }
