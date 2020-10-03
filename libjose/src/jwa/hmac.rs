use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use crypto::key_box::SecretKey;
use crypto::rand::OsRng;
use crypto::signers::hmac;

use crate::error::Error;
use crate::error::Result;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkParams;
use crate::jwk::JwkParamsOct;
use crate::jwk::JwkType;
use crate::jwk::JwkUse;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::utils::decode_b64;
use crate::utils::encode_b64;

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
  /// Returns the JWA identifier of the Hmac algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::HS256 => "HS256",
      Self::HS384 => "HS384",
      Self::HS512 => "HS512",
    }
  }

  pub const fn key_size(self) -> usize {
    match self {
      Self::HS256 => hmac::SHA256_OUTPUT_SIZE,
      Self::HS384 => hmac::SHA384_OUTPUT_SIZE,
      Self::HS512 => hmac::SHA512_OUTPUT_SIZE,
    }
  }

  /// Creates a new random `HmacKey`.
  pub fn generate_key(self) -> Result<HmacKey> {
    HmacKey::new(self)
  }

  /// Creates a new `HmacSigner` from the given slice of bytes.
  pub fn signer_from_bytes(self, data: impl AsRef<[u8]>) -> Result<HmacSigner> {
    HmacKey::from_bytes(self, data).map(|key| key.to_signer())
  }

  /// Creates a new `HmacSigner` from a base64url-encoded key.
  pub fn signer_from_b64(self, data: impl AsRef<[u8]>) -> Result<HmacSigner> {
    HmacKey::from_b64(self, data).map(|key| key.to_signer())
  }

  /// Creates a new `HmacSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<HmacSigner> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Sign)?;

    HmacKey::from_jwk(self, data).map(|key| key.to_signer())
  }

  /// Creates a new `HmacVerifier` from the given slice of bytes.
  pub fn verifier_from_bytes(self, data: impl AsRef<[u8]>) -> Result<HmacVerifier> {
    HmacKey::from_bytes(self, data).map(|key| key.to_verifier())
  }

  /// Creates a new `HmacVerifier` from a base64url-encoded key.
  pub fn verifier_from_b64(self, data: impl AsRef<[u8]>) -> Result<HmacVerifier> {
    HmacKey::from_b64(self, data).map(|key| key.to_verifier())
  }

  /// Creates a new `HmacVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<HmacVerifier> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Verify)?;

    HmacKey::from_jwk(self, data).map(|key| key.to_verifier())
  }

  /// Creates a new Hmac JSON Web Key from the given slice of bytes.
  pub fn to_jwk(self, data: impl AsRef<[u8]>) -> Result<Jwk> {
    HmacKey::from_bytes(self, data).map(|key| key.to_jwk())
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

// =============================================================================
// Hmac Signer
// =============================================================================

#[derive(Debug)]
pub struct HmacSigner {
  alg: HmacAlgorithm,
  key: SecretKey,
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
    let signature: Vec<u8> = match self.alg {
      HmacAlgorithm::HS256 => hmac::sign_sha256(&self.key, message)?.to_vec(),
      HmacAlgorithm::HS384 => hmac::sign_sha384(&self.key, message)?.to_vec(),
      HmacAlgorithm::HS512 => hmac::sign_sha512(&self.key, message)?.to_vec(),
    };

    Ok(signature)
  }
}

impl Deref for HmacSigner {
  type Target = dyn JwsSigner;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// Hmac Verifier
// =============================================================================

#[derive(Debug)]
pub struct HmacVerifier {
  alg: HmacAlgorithm,
  key: SecretKey,
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
    match self.alg {
      HmacAlgorithm::HS256 => hmac::verify_sha256(&self.key, message, signature)?,
      HmacAlgorithm::HS384 => hmac::verify_sha384(&self.key, message, signature)?,
      HmacAlgorithm::HS512 => hmac::verify_sha512(&self.key, message, signature)?,
    }

    Ok(())
  }
}

impl Deref for HmacVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// Hmac Key
// =============================================================================

#[derive(Clone, Debug)]
pub struct HmacKey {
  alg: HmacAlgorithm,
  key: SecretKey,
  kid: Option<String>,
}

impl HmacKey {
  /// Creates a new random `HmacKey`.
  pub fn new(alg: HmacAlgorithm) -> Result<Self> {
    Ok(Self {
      alg,
      key: SecretKey::random(alg.key_size(), &mut OsRng)?,
      kid: None,
    })
  }

  /// Creates a new `HmacKey` from the given slice of bytes.
  pub fn from_bytes(alg: HmacAlgorithm, data: impl AsRef<[u8]>) -> Result<Self> {
    let data: &[u8] = data.as_ref();

    if data.len() < alg.key_size() {
      return Err(Error::InvalidKeyFormat(alg.name()));
    }

    Ok(Self {
      alg,
      key: data.into(),
      kid: None,
    })
  }

  /// Creates a new `HmacKey` from a base64url-encoded key.
  pub fn from_b64(alg: HmacAlgorithm, data: impl AsRef<[u8]>) -> Result<Self> {
    let data: Vec<u8> = decode_b64(data)?;

    if data.len() < alg.key_size() {
      return Err(Error::InvalidKeyFormat(alg.name()));
    }

    Ok(Self {
      alg,
      key: data.into(),
      kid: None,
    })
  }

  /// Creates a new `HmacKey` from a JSON Web Key.
  pub fn from_jwk(alg: HmacAlgorithm, data: &Jwk) -> Result<Self> {
    data.check_alg(alg.name())?;
    data.check_kty(JwkType::Oct)?;

    let k: &str = match data.params() {
      Some(JwkParams::Oct(JwkParamsOct { k })) => k.as_str(),
      Some(_) | None => return Err(Error::InvalidKeyFormat(alg.name())),
    };

    Self::from_b64(alg, k).map(|mut this| {
      this.kid = data.kid().map(ToString::to_string);
      this
    })
  }

  /// Creates a new Hmac JSON Web Key with the given slice of bytes.
  pub fn to_jwk(&self) -> Jwk {
    let mut jwk: Jwk = Jwk::with_kty(JwkType::Oct);
    let mut ops: Vec<JwkOperation> = Vec::with_capacity(2);

    ops.push(JwkOperation::Sign);
    ops.push(JwkOperation::Verify);

    jwk.set_alg(self.alg.name());
    jwk.set_use(JwkUse::Signature);
    jwk.set_key_ops(ops);

    jwk.set_params(JwkParamsOct {
      k: encode_b64(&self.key),
    });

    jwk
  }

  /// Creates a new `HmacSigner` from this `HmacKey`.
  pub fn to_signer(&self) -> HmacSigner {
    HmacSigner {
      alg: self.alg,
      key: self.key.clone(),
      kid: self.kid.clone(),
    }
  }

  /// Creates a new `HmacVerifier` from this `HmacKey`.
  pub fn to_verifier(&self) -> HmacVerifier {
    HmacVerifier {
      alg: self.alg,
      key: self.key.clone(),
      kid: self.kid.clone(),
    }
  }
}
