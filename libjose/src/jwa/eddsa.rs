use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::crypto::eddsa_generate;
use crate::crypto::eddsa_sign;
use crate::crypto::eddsa_verify;
use crate::crypto::PKey;
use crate::crypto::Public;
use crate::crypto::Secret;
use crate::error::Result;
use crate::jwk::EdCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkKeyPair;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum EddsaAlgorithm {
  /// EdDSA signature algorithm
  EdDSA,
}

impl EddsaAlgorithm {
  pub const fn name(self) -> &'static str {
    match self {
      Self::EdDSA => "EdDSA",
    }
  }

  pub fn generate_keypair(self, curve: EdCurve) -> Result<EddsaKeyPair> {
    EddsaKeyPair::generate(self, curve)
  }

  pub fn keypair_from_der(self, _data: impl AsRef<[u8]>) -> Result<EddsaKeyPair> {
    todo!("EddsaAlgorithm::keypair_from_der")
  }

  pub fn keypair_from_pem(self, _data: impl AsRef<[u8]>) -> Result<EddsaKeyPair> {
    todo!("EddsaAlgorithm::keypair_from_pem")
  }

  pub fn signer_from_der(self, _data: impl AsRef<[u8]>) -> Result<EddsaSigner> {
    todo!("EddsaAlgorithm::signer_from_der")
  }

  pub fn signer_from_jwk(self, _data: &Jwk) -> Result<EddsaSigner> {
    todo!("EddsaAlgorithm::signer_from_jwk")
  }

  pub fn signer_from_pem(self, _data: impl AsRef<[u8]>) -> Result<EddsaSigner> {
    todo!("EddsaAlgorithm::signer_from_pem")
  }

  pub fn verifier_from_der(self, _data: impl AsRef<[u8]>) -> Result<EddsaVerifier> {
    todo!("EddsaAlgorithm::verifier_from_der")
  }

  pub fn verifier_from_jwk(self, _data: &Jwk) -> Result<EddsaVerifier> {
    todo!("EddsaAlgorithm::verifier_from_jwk")
  }

  pub fn verifier_from_pem(self, _data: impl AsRef<[u8]>) -> Result<EddsaVerifier> {
    todo!("EddsaAlgorithm::verifier_from_pem")
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EddsaSigner {
  alg: EddsaAlgorithm,
  crv: EdCurve,
  key: PKey<Secret>,
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
    eddsa_sign(self.alg, message, &self.key)
  }
}

impl Deref for EddsaSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EddsaVerifier {
  alg: EddsaAlgorithm,
  key: PKey<Public>,
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
    eddsa_verify(self.alg, message, signature, &self.key)
  }
}

impl Deref for EddsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EddsaKeyPair {
  alg: EddsaAlgorithm,
  crv: EdCurve,
  kid: Option<String>,
  pkey: PKey<Public>,
  skey: PKey<Secret>,
}

impl EddsaKeyPair {
  pub fn generate(alg: EddsaAlgorithm, crv: EdCurve) -> Result<Self> {
    let (pkey, skey) = eddsa_generate(crv)?;

    Ok(Self {
      alg,
      crv,
      kid: None,
      pkey,
      skey,
    })
  }

  pub fn from_der(&self, _data: impl AsRef<[u8]>) -> Result<Self> {
    todo!("EddsaKeyPair::from_der")
  }

  pub fn from_jwk(&self, _data: &Jwk) -> Result<Self> {
    todo!("EddsaKeyPair::from_jwk")
  }

  pub fn from_pem(&self, _data: impl AsRef<[u8]>) -> Result<Self> {
    todo!("EddsaKeyPair::from_pem")
  }

  pub fn to_jwk(&self, _public: bool, _secret: bool) -> Jwk {
    todo!("EddsaKeyPair::to_jwk")
  }
}

impl JwkKeyPair for EddsaKeyPair {
  fn to_public_pem(&self) -> Vec<u8> {
    todo!("EddsaKeyPair::to_public_pem")
  }

  fn to_secret_pem(&self) -> Vec<u8> {
    todo!("EddsaKeyPair::to_secret_pem")
  }

  fn to_public_der(&self) -> Vec<u8> {
    todo!("EddsaKeyPair::to_public_der")
  }

  fn to_secret_der(&self) -> Vec<u8> {
    todo!("EddsaKeyPair::to_secret_der")
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
