use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::crypto::ecdsa_generate;
use crate::crypto::ecdsa_sign;
use crate::crypto::ecdsa_verify;
use crate::crypto::PKey;
use crate::crypto::Public;
use crate::crypto::Secret;
use crate::error::Result;
use crate::jwk::EcCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkKeyPair;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;

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
  pub const fn name(self) -> &'static str {
    match self {
      Self::ES256 => "ES256",
      Self::ES384 => "ES384",
      Self::ES512 => "ES512",
      Self::ES256K => "ES256K",
    }
  }

  pub const fn curve(self) -> EcCurve {
    match self {
      Self::ES256 => EcCurve::P256,
      Self::ES384 => EcCurve::P384,
      Self::ES512 => EcCurve::P521,
      Self::ES256K => EcCurve::Secp256K1,
    }
  }

  pub fn generate_keypair(self) -> Result<EcdsaKeyPair> {
    EcdsaKeyPair::generate(self)
  }

  pub fn keypair_from_der(self, _data: impl AsRef<[u8]>) -> Result<EcdsaKeyPair> {
    todo!("EcdsaAlgorithm::keypair_from_der")
  }

  pub fn keypair_from_pem(self, _data: impl AsRef<[u8]>) -> Result<EcdsaKeyPair> {
    todo!("EcdsaAlgorithm::keypair_from_pem")
  }

  pub fn signer_from_der(self, _data: impl AsRef<[u8]>) -> Result<EcdsaSigner> {
    todo!("EcdsaAlgorithm::signer_from_der")
  }

  pub fn signer_from_jwk(self, _data: &Jwk) -> Result<EcdsaSigner> {
    todo!("EcdsaAlgorithm::signer_from_jwk")
  }

  pub fn signer_from_pem(self, _data: impl AsRef<[u8]>) -> Result<EcdsaSigner> {
    todo!("EcdsaAlgorithm::signer_from_pem")
  }

  pub fn verifier_from_der(self, _data: impl AsRef<[u8]>) -> Result<EcdsaVerifier> {
    todo!("EcdsaAlgorithm::verifier_from_der")
  }

  pub fn verifier_from_jwk(self, _data: &Jwk) -> Result<EcdsaVerifier> {
    todo!("EcdsaAlgorithm::verifier_from_jwk")
  }

  pub fn verifier_from_pem(self, _data: impl AsRef<[u8]>) -> Result<EcdsaVerifier> {
    todo!("EcdsaAlgorithm::verifier_from_pem")
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EcdsaKeyPair {
  alg: EcdsaAlgorithm,
  kid: Option<String>,
  pkey: PKey<Public>,
  skey: PKey<Secret>,
}

impl EcdsaKeyPair {
  pub fn generate(alg: EcdsaAlgorithm) -> Result<Self> {
    let (pkey, skey) = ecdsa_generate(alg.curve())?;

    Ok(Self {
      alg,
      kid: None,
      pkey,
      skey,
    })
  }

  pub fn from_der(&self, _data: impl AsRef<[u8]>) -> Result<Self> {
    todo!("EcdsaKeyPair::from_der")
  }

  pub fn from_jwk(&self, _data: &Jwk) -> Result<Self> {
    todo!("EcdsaKeyPair::from_jwk")
  }

  pub fn from_pem(&self, _data: impl AsRef<[u8]>) -> Result<Self> {
    todo!("EcdsaKeyPair::from_pem")
  }

  pub fn to_jwk(&self, _public: bool, _secret: bool) -> Jwk {
    todo!("EcdsaKeyPair::to_jwk")
  }
}

impl JwkKeyPair for EcdsaKeyPair {
  fn to_public_pem(&self) -> Vec<u8> {
    todo!("EcdsaKeyPair::to_public_pem")
  }

  fn to_secret_pem(&self) -> Vec<u8> {
    todo!("EcdsaKeyPair::to_secret_pem")
  }

  fn to_public_der(&self) -> Vec<u8> {
    todo!("EcdsaKeyPair::to_public_der")
  }

  fn to_secret_der(&self) -> Vec<u8> {
    todo!("EcdsaKeyPair::to_secret_der")
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
