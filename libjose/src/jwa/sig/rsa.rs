use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::error::Result;
use crate::jwa::PKey;
use crate::jwa::PKeyExt as _;
use crate::jwa::Public;
use crate::jwa::RsaBits;
use crate::jwa::RsaKeyPair;
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
pub enum RsaAlgorithm {
  /// RSASSA-PKCS1-v1_5 using SHA-256
  RS256,
  /// RSASSA-PKCS1-v1_5 using SHA-384
  RS384,
  /// RSASSA-PKCS1-v1_5 using SHA-512
  RS512,
  /// RSASSA-PSS using SHA-256 and MGF1 with SHA-256
  PS256,
  /// RSASSA-PSS using SHA-384 and MGF1 with SHA-384
  PS384,
  /// RSASSA-PSS using SHA-512 and MGF1 with SHA-512
  PS512,
}

impl RsaAlgorithm {
  /// Returns the JWA identifier of the RSA algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::RS256 => "RS256",
      Self::RS384 => "RS384",
      Self::RS512 => "RS512",
      Self::PS256 => "PS256",
      Self::PS384 => "PS384",
      Self::PS512 => "PS512",
    }
  }

  /// Creates a new cryptographically random private key.
  pub fn generate_key(self, bits: RsaBits) -> Result<RsaKeyPair> {
    RsaKeyPair::random(bits)
  }

  /// Creates a new `RsaSigner` from a DER-encoded PKCS#1 RSA private key.
  pub fn signer_from_pkcs1(self, data: impl AsRef<[u8]>) -> Result<RsaSigner> {
    Ok(RsaSigner {
      alg: self,
      key: PKey::from_rsa_pkcs1(data)?,
      kid: None,
    })
  }

  /// Creates a new `RsaSigner` from a DER-encoded PKCS#8 RSA private key.
  pub fn signer_from_pkcs8(self, data: impl AsRef<[u8]>) -> Result<RsaSigner> {
    Ok(RsaSigner {
      alg: self,
      key: PKey::from_rsa_pkcs8(data)?,
      kid: None,
    })
  }

  /// Creates a new `RsaSigner` from a PEM-encoded RSA private key.
  #[cfg(feature = "pem")]
  pub fn signer_from_pem(self, data: impl AsRef<[u8]>) -> Result<RsaSigner> {
    Ok(RsaSigner {
      alg: self,
      key: PKey::from_rsa_pem(data)?,
      kid: None,
    })
  }

  /// Creates a new `RsaSigner` from a JSON Web Key.
  #[allow(clippy::many_single_char_names)]
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<RsaSigner> {
    data.check_use(JwkUse::Signature)?;
    data.check_ops(JwkOperation::Sign)?;
    data.check_alg(self.name())?;

    let key: RsaKeyPair = RsaKeyPair::from_jwk(data)?;

    Ok(RsaSigner {
      alg: self,
      key: key.key,
      kid: key.kid,
    })
  }

  /// Creates a new `RsaVerifier` from a DER-encoded PKCS#8 RSA public key.
  pub fn verifier_from_pkcs1(self, data: impl AsRef<[u8]>) -> Result<RsaVerifier> {
    Ok(RsaVerifier {
      alg: self,
      key: PKey::from_rsa_pkcs1(data)?,
      kid: None,
    })
  }

  /// Creates a new `RsaVerifier` from a DER-encoded PKCS#8 RSA public key.
  pub fn verifier_from_pkcs8(self, data: impl AsRef<[u8]>) -> Result<RsaVerifier> {
    Ok(RsaVerifier {
      alg: self,
      key: PKey::from_rsa_pkcs8(data)?,
      kid: None,
    })
  }

  /// Creates a new `RsaVerifier` from a PEM-encoded RSA public key.
  pub fn verifier_from_pem(self, data: impl AsRef<[u8]>) -> Result<RsaVerifier> {
    Ok(RsaVerifier {
      alg: self,
      key: PKey::from_rsa_pem(data)?,
      kid: None,
    })
  }

  /// Creates a new `RsaVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<RsaVerifier> {
    data.check_use(JwkUse::Signature)?;
    data.check_ops(JwkOperation::Verify)?;
    data.check_alg(self.name())?;

    Ok(RsaVerifier {
      alg: self,
      key: RsaKeyPair::public_from_jwk(data)?,
      kid: data.kid().map(ToString::to_string),
    })
  }
}

impl Display for RsaAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
  }
}

impl From<RsaAlgorithm> for JwsAlgorithm {
  fn from(other: RsaAlgorithm) -> Self {
    match other {
      RsaAlgorithm::RS256 => Self::RS256,
      RsaAlgorithm::RS384 => Self::RS384,
      RsaAlgorithm::RS512 => Self::RS512,
      RsaAlgorithm::PS256 => Self::PS256,
      RsaAlgorithm::PS384 => Self::PS384,
      RsaAlgorithm::PS512 => Self::PS512,
    }
  }
}

// =============================================================================
// RSA Signer
// =============================================================================

#[derive(Debug)]
pub struct RsaSigner {
  alg: RsaAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl RsaSigner {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JwsSigner for RsaSigner {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    self.key.try_rsa_sign(self.alg, message)
  }
}

impl Deref for RsaSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// RSA Verifier
// =============================================================================

#[derive(Debug)]
pub struct RsaVerifier {
  alg: RsaAlgorithm,
  key: PKey<Public>,
  kid: Option<String>,
}

impl RsaVerifier {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JwsVerifier for RsaVerifier {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    self.key.try_rsa_verify(self.alg, message, signature)
  }
}

impl Deref for RsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
