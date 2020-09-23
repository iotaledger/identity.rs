use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::crypto::rsassa_sign;
use crate::crypto::rsassa_verify;
use crate::crypto::PKey;
use crate::crypto::Public;
use crate::crypto::Secret;
use crate::error::Result;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkUse;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::utils::pem_decode;
use crate::alloc::String;
use crate::alloc::Vec;
use crate::alloc::ToString;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum RsassaAlgorithm {
  /// RSASSA-PKCS1-v1_5 using SHA-256
  RS256,
  /// RSASSA-PKCS1-v1_5 using SHA-384
  RS384,
  /// RSASSA-PKCS1-v1_5 using SHA-512
  RS512,
}

impl RsassaAlgorithm {
  /// Returns the JWA identifier of the algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::RS256 => "RS256",
      Self::RS384 => "RS384",
      Self::RS512 => "RS512",
    }
  }

  /// Creates a new `RsassaSigner` from DER-encoded material in PKCS#8 form.
  pub fn signer_from_der(self, data: impl AsRef<[u8]>) -> Result<RsassaSigner> {
    // TODO: Parse and validate key format
    Ok(RsassaSigner {
      alg: self,
      key: data.as_ref().into(),
      kid: None,
    })
  }

  /// Creates a new `RsassaSigner` from a PEM-encoded document.
  pub fn signer_from_pem(self, data: impl AsRef<[u8]>) -> Result<RsassaSigner> {
    // TODO: Parse and validate key format
    Ok(RsassaSigner {
      alg: self,
      key: pem_decode(&data).map(|pem| pem.pem_data.into())?,
      kid: None,
    })
  }

  /// Creates a new `RsassaSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<RsassaSigner> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Sign)?;
    data.check_alg(self.name())?;

    let key: PKey<Secret> = todo!("RsassaSigner::signer_from_jwk");
    let kid: Option<String> = data.kid().map(ToString::to_string);

    Ok(RsassaSigner {
      alg: self,
      key,
      kid,
    })
  }

  /// Creates a new `RsassaVerifier` from DER-encoded material in PKCS#8 form.
  pub fn verifier_from_der(self, data: impl AsRef<[u8]>) -> Result<RsassaVerifier> {
    // TODO: Parse and validate key format
    Ok(RsassaVerifier {
      alg: self,
      key: data.as_ref().into(),
      kid: None,
    })
  }

  /// Creates a new `RsassaVerifier` from a PEM-encoded document.
  pub fn verifier_from_pem(self, data: impl AsRef<[u8]>) -> Result<RsassaVerifier> {
    // TODO: Parse and validate key format
    Ok(RsassaVerifier {
      alg: self,
      key: pem_decode(&data).map(|pem| pem.pem_data.into())?,
      kid: None,
    })
  }

  /// Creates a new `RsassaVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<RsassaVerifier> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Verify)?;
    data.check_alg(self.name())?;

    let key: PKey<Public> = todo!("RsassaVerifier::verifier_from_jwk");
    let kid: Option<String> = data.kid().map(ToString::to_string);

    Ok(RsassaVerifier {
      alg: self,
      key,
      kid,
    })
  }
}

impl Display for RsassaAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl From<RsassaAlgorithm> for JwsAlgorithm {
  fn from(other: RsassaAlgorithm) -> Self {
    match other {
      RsassaAlgorithm::RS256 => Self::RS256,
      RsassaAlgorithm::RS384 => Self::RS384,
      RsassaAlgorithm::RS512 => Self::RS512,
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RsassaSigner {
  alg: RsassaAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JwsSigner for RsassaSigner {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    rsassa_sign(self.alg, message, &self.key)
  }
}

impl Deref for RsassaSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RsassaVerifier {
  alg: RsassaAlgorithm,
  key: PKey<Public>,
  kid: Option<String>,
}

impl JwsVerifier for RsassaVerifier {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    rsassa_verify(self.alg, message, signature, &self.key)
  }
}

impl Deref for RsassaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
