use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::crypto::rsassa_pss_sign;
use crate::crypto::rsassa_pss_verify;
use crate::crypto::PKey;
use crate::crypto::Public;
use crate::crypto::Secret;
use crate::error::Result;
use crate::jwa::HashAlgorithm;
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
pub enum RsassaPssAlgorithm {
  /// RSASSA-PSS using SHA-256 and MGF1 with SHA-256
  PS256,
  /// RSASSA-PSS using SHA-384 and MGF1 with SHA-384
  PS384,
  /// RSASSA-PSS using SHA-512 and MGF1 with SHA-512
  PS512,
}

impl RsassaPssAlgorithm {
  /// Returns the JWA identifier of the algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::PS256 => "PS256",
      Self::PS384 => "PS384",
      Self::PS512 => "PS512",
    }
  }

  pub const fn hash_alg(self) -> HashAlgorithm {
    match self {
      Self::PS256 => HashAlgorithm::Sha256,
      Self::PS384 => HashAlgorithm::Sha384,
      Self::PS512 => HashAlgorithm::Sha512,
    }
  }

  /// Creates a new `RsassaPssSigner` from DER-encoded material in PKCS#8 form.
  pub fn signer_from_der(self, data: impl AsRef<[u8]>) -> Result<RsassaPssSigner> {
    // TODO: Parse and validate key format
    Ok(RsassaPssSigner {
      alg: self,
      key: data.as_ref().into(),
      kid: None,
    })
  }

  /// Creates a new `RsassaPssSigner` from a PEM-encoded document.
  pub fn signer_from_pem(self, data: impl AsRef<[u8]>) -> Result<RsassaPssSigner> {
    // TODO: Parse and validate key format
    Ok(RsassaPssSigner {
      alg: self,
      key: pem_decode(&data).map(|pem| pem.pem_data.into())?,
      kid: None,
    })
  }

  /// Creates a new `RsassaPssSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<RsassaPssSigner> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Sign)?;
    data.check_alg(self.name())?;

    let key: PKey<Secret> = todo!("RsassaPssSigner::signer_from_jwk");
    let kid: Option<String> = data.kid().map(ToString::to_string);

    Ok(RsassaPssSigner {
      alg: self,
      key,
      kid,
    })
  }

  /// Creates a new `RsassaPssVerifier` from DER-encoded material in PKCS#8
  /// form.
  pub fn verifier_from_der(self, data: impl AsRef<[u8]>) -> Result<RsassaPssVerifier> {
    // TODO: Parse and validate key format
    Ok(RsassaPssVerifier {
      alg: self,
      key: data.as_ref().into(),
      kid: None,
    })
  }

  /// Creates a new `RsassaPssVerifier` from a PEM-encoded document.
  pub fn verifier_from_pem(self, data: impl AsRef<[u8]>) -> Result<RsassaPssVerifier> {
    // TODO: Parse and validate key format
    Ok(RsassaPssVerifier {
      alg: self,
      key: pem_decode(&data).map(|pem| pem.pem_data.into())?,
      kid: None,
    })
  }

  /// Creates a new `RsassaPssVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<RsassaPssVerifier> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Verify)?;
    data.check_alg(self.name())?;

    let key: PKey<Public> = todo!("RsassaPssVerifier::verifier_from_jwk");
    let kid: Option<String> = data.kid().map(ToString::to_string);

    Ok(RsassaPssVerifier {
      alg: self,
      key,
      kid,
    })
  }
}

impl Display for RsassaPssAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl From<RsassaPssAlgorithm> for JwsAlgorithm {
  fn from(other: RsassaPssAlgorithm) -> Self {
    match other {
      RsassaPssAlgorithm::PS256 => Self::PS256,
      RsassaPssAlgorithm::PS384 => Self::PS384,
      RsassaPssAlgorithm::PS512 => Self::PS512,
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RsassaPssSigner {
  alg: RsassaPssAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JwsSigner for RsassaPssSigner {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    rsassa_pss_sign(self.alg, message, &self.key)
  }
}

impl Deref for RsassaPssSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RsassaPssVerifier {
  alg: RsassaPssAlgorithm,
  key: PKey<Public>,
  kid: Option<String>,
}

impl JwsVerifier for RsassaPssVerifier {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    rsassa_pss_verify(self.alg, message, signature, &self.key)
  }
}

impl Deref for RsassaPssVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
