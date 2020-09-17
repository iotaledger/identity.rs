use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::crypto::hmac_generate;
use crate::crypto::hmac_sign;
use crate::crypto::hmac_verify;
use crate::crypto::PKey;
use crate::crypto::Public;
use crate::crypto::Secret;
use crate::error::Error;
use crate::error::Result;
use crate::jwk::HashAlgorithm;
use crate::jwk::Jwk;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::utils::decode_b64;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum HmacAlgorithm {
  /// HMAC using SHA-256
  HS256,
  /// HMAC using SHA-384
  HS384,
  /// HMAC using SHA-512
  HS512,
}

impl HmacAlgorithm {
  pub const fn name(self) -> &'static str {
    match self {
      Self::HS256 => "HS256",
      Self::HS384 => "HS384",
      Self::HS512 => "HS512",
    }
  }

  pub const fn hash_alg(self) -> HashAlgorithm {
    match self {
      Self::HS256 => HashAlgorithm::Sha256,
      Self::HS384 => HashAlgorithm::Sha384,
      Self::HS512 => HashAlgorithm::Sha512,
    }
  }

  pub fn generate_key(self) -> Result<PKey<Secret>> {
    hmac_generate(self)
  }

  pub fn signer_from_raw(self, data: impl AsRef<[u8]>) -> Result<HmacSigner> {
    let data: &[u8] = data.as_ref();

    if data.len() < self.hash_alg().size() {
      return Err(Error::invalid_key());
    }

    Ok(HmacSigner {
      alg: self,
      key: data.into(),
      kid: None,
    })
  }

  pub fn signer_from_b64(self, data: impl AsRef<[u8]>) -> Result<HmacSigner> {
    let data: Vec<u8> = decode_b64(data.as_ref())?;

    if data.len() < self.hash_alg().size() {
      return Err(Error::invalid_key());
    }

    Ok(HmacSigner {
      alg: self,
      key: data.into(),
      kid: None,
    })
  }

  pub fn signer_from_jwk(self, _data: &Jwk) -> Result<HmacSigner> {
    todo!("HmacAlgorithm::signer_from_jwk")
  }

  pub fn verifier_from_raw(self, data: impl AsRef<[u8]>) -> Result<HmacVerifier> {
    let data: &[u8] = data.as_ref();

    if data.len() < self.hash_alg().size() {
      return Err(Error::invalid_key());
    }

    Ok(HmacVerifier {
      alg: self,
      key: data.into(),
      kid: None,
    })
  }

  pub fn verifier_from_b64(self, data: impl AsRef<[u8]>) -> Result<HmacVerifier> {
    let data: Vec<u8> = decode_b64(data.as_ref())?;

    if data.len() < self.hash_alg().size() {
      return Err(Error::invalid_key());
    }

    Ok(HmacVerifier {
      alg: self,
      key: data.into(),
      kid: None,
    })
  }

  pub fn verifier_from_jwk(self, _data: &Jwk) -> Result<HmacVerifier> {
    todo!("HmacAlgorithm::verifier_from_jwk")
  }

  pub fn to_jwk(self, _data: impl AsRef<[u8]>) -> Jwk {
    todo!("HmacAlgorithm::to_jwk")
  }
}

impl Display for HmacAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl From<HmacAlgorithm> for JwsAlgorithm {
  fn from(other: HmacAlgorithm) -> Self {
    match other {
      HmacAlgorithm::HS256 => Self::HS256,
      HmacAlgorithm::HS384 => Self::HS384,
      HmacAlgorithm::HS512 => Self::HS512,
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HmacSigner {
  alg: HmacAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JwsSigner for HmacSigner {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    hmac_sign(self.alg, message, &self.key)
  }
}

impl Deref for HmacSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HmacVerifier {
  alg: HmacAlgorithm,
  key: PKey<Public>,
  kid: Option<String>,
}

impl JwsVerifier for HmacVerifier {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    hmac_verify(self.alg, message, signature, &self.key)
  }
}

impl Deref for HmacVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
