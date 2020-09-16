use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::crypto::rsa_generate;
use crate::crypto::rsa_sign;
use crate::crypto::rsa_verify;
use crate::crypto::PKey;
use crate::crypto::Public;
use crate::crypto::Secret;
use crate::error::Result;
use crate::jwk::Jwk;
use crate::jwk::JwkKeyPair;
use crate::jwk::RsaBits;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;

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

  pub fn generate_keypair(self, bits: RsaBits) -> Result<RsaKeyPair> {
    RsaKeyPair::generate(self, bits)
  }

  pub fn keypair_from_der(self, _data: impl AsRef<[u8]>) -> Result<RsaKeyPair> {
    todo!("RsaAlgorithm::keypair_from_der")
  }

  pub fn keypair_from_pem(self, _data: impl AsRef<[u8]>) -> Result<RsaKeyPair> {
    todo!("RsaAlgorithm::keypair_from_pem")
  }

  pub fn signer_from_der(self, _data: impl AsRef<[u8]>) -> Result<RsaSigner> {
    todo!("RsaAlgorithm::signer_from_der")
  }

  pub fn signer_from_jwk(self, _data: &Jwk) -> Result<RsaSigner> {
    todo!("RsaAlgorithm::signer_from_jwk")
  }

  pub fn signer_from_pem(self, _data: impl AsRef<[u8]>) -> Result<RsaSigner> {
    todo!("RsaAlgorithm::signer_from_pem")
  }

  pub fn verifier_from_der(self, _data: impl AsRef<[u8]>) -> Result<RsaVerifier> {
    todo!("RsaAlgorithm::verifier_from_der")
  }

  pub fn verifier_from_jwk(self, _data: &Jwk) -> Result<RsaVerifier> {
    todo!("RsaAlgorithm::verifier_from_jwk")
  }

  pub fn verifier_from_pem(self, _data: impl AsRef<[u8]>) -> Result<RsaVerifier> {
    todo!("RsaAlgorithm::verifier_from_pem")
  }
}

impl Display for RsaAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}", self.name()))
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RsaSigner {
  alg: RsaAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JwsSigner for RsaSigner {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    rsa_sign(self.alg, message, &self.key)
  }
}

impl Deref for RsaSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RsaVerifier {
  alg: RsaAlgorithm,
  key: PKey<Public>,
  kid: Option<String>,
}

impl JwsVerifier for RsaVerifier {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    rsa_verify(self.alg, message, signature, &self.key)
  }
}

impl Deref for RsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RsaKeyPair {
  alg: RsaAlgorithm,
  kid: Option<String>,
  pkey: PKey<Public>,
  skey: PKey<Secret>,
}

impl RsaKeyPair {
  pub fn generate(alg: RsaAlgorithm, bits: RsaBits) -> Result<Self> {
    let (pkey, skey) = rsa_generate(bits)?;

    Ok(Self {
      alg,
      kid: None,
      pkey,
      skey,
    })
  }

  pub fn from_der(&self, _data: impl AsRef<[u8]>) -> Result<Self> {
    todo!("RsaKeyPair::from_der")
  }

  pub fn from_jwk(&self, _data: &Jwk) -> Result<Self> {
    todo!("RsaKeyPair::from_jwk")
  }

  pub fn from_pem(&self, _data: impl AsRef<[u8]>) -> Result<Self> {
    todo!("RsaKeyPair::from_pem")
  }

  pub fn to_jwk(&self, _public: bool, _secret: bool) -> Jwk {
    todo!("RsaKeyPair::to_jwk")
  }
}

impl JwkKeyPair for RsaKeyPair {
  fn to_public_pem(&self) -> Vec<u8> {
    todo!("RsaKeyPair::to_public_pem")
  }

  fn to_secret_pem(&self) -> Vec<u8> {
    todo!("RsaKeyPair::to_secret_pem")
  }

  fn to_public_der(&self) -> Vec<u8> {
    todo!("RsaKeyPair::to_public_der")
  }

  fn to_secret_der(&self) -> Vec<u8> {
    todo!("RsaKeyPair::to_secret_der")
  }

  fn to_public_jwk(&self) -> Jwk {
    self.to_jwk(true, false)
  }

  fn to_secret_jwk(&self) -> Jwk {
    self.to_jwk(false, true)
  }

  fn to_combined_jwk(&self) -> Jwk {
    self.to_jwk(true, true)
  }
}
