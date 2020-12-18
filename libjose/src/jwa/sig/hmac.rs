use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use crypto::hashes::sha::SHA256_LEN;
use crypto::hashes::sha::SHA384_LEN;
use crypto::hashes::sha::SHA512_LEN;
use crypto::macs::hmac::HMAC_SHA256;
use crypto::macs::hmac::HMAC_SHA384;
use crypto::macs::hmac::HMAC_SHA512;

use crate::error::Error;
use crate::error::Result;
use crate::jwa::PKey;
use crate::jwa::Secret;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkParams;
use crate::jwk::JwkParamsOct;
use crate::jwk::JwkType;
use crate::jwk::JwkUse;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::lib::*;
use crate::utils::decode_b64;
use crate::utils::encode_b64;

macro_rules! sign {
  ($impl:ident, $size:expr, $message:expr, $key:expr) => {{
    let mut output: [u8; $size] = [0; $size];

    $impl($message, $key, &mut output);

    output.to_vec()
  }};
}

macro_rules! verify {
  ($impl:ident, $size:expr, $message:expr, $signature:expr, $key:expr) => {{
    let mac: Vec<u8> = sign!($impl, $size, $message, $key);

    if mac != $signature {
      return Err(Error::SigError("Invalid HMAC Signature"));
    }

    Ok(())
  }};
}

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

  pub fn key_size(self) -> usize {
    match self {
      Self::HS256 => SHA256_LEN,
      Self::HS384 => SHA384_LEN,
      Self::HS512 => SHA512_LEN,
    }
  }

  /// Creates a new cryptographically random private key.
  pub fn generate_key(self) -> Result<PKey<Secret>> {
    PKey::generate_raw(self.key_size())
  }

  /// Creates a new `HmacSigner` from the given slice of bytes.
  pub fn signer_from_bytes(self, data: impl AsRef<[u8]>) -> Result<HmacSigner> {
    let data: &[u8] = data.as_ref();

    if data.len() < self.key_size() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(HmacSigner {
      alg: self,
      key: PKey::from_raw_bytes(data),
      kid: None,
    })
  }

  /// Creates a new `HmacSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<HmacSigner> {
    data.check_use(JwkUse::Signature)?;
    data.check_ops(JwkOperation::Sign)?;
    data.check_alg(self.name())?;
    data.check_kty(JwkType::Oct)?;

    let k: Vec<u8> = match data.params() {
      JwkParams::Oct(JwkParamsOct { k }) => decode_b64(k)?,
      _ => return Err(Error::KeyError(self.name())),
    };

    Ok(HmacSigner {
      alg: self,
      key: PKey::from_raw_bytes(k),
      kid: data.kid().map(ToString::to_string),
    })
  }

  /// Creates a new `HmacVerifier` from the given slice of bytes.
  pub fn verifier_from_bytes(self, data: impl AsRef<[u8]>) -> Result<HmacVerifier> {
    let data: &[u8] = data.as_ref();

    if data.len() < self.key_size() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(HmacVerifier {
      alg: self,
      key: PKey::from_raw_bytes(data),
      kid: None,
    })
  }

  /// Creates a new `HmacVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<HmacVerifier> {
    data.check_use(JwkUse::Signature)?;
    data.check_ops(JwkOperation::Verify)?;
    data.check_kty(JwkType::Oct)?;

    let k: Vec<u8> = match data.params() {
      JwkParams::Oct(JwkParamsOct { k }) => decode_b64(k)?,
      _ => return Err(Error::KeyError(self.name())),
    };

    Ok(HmacVerifier {
      alg: self,
      key: PKey::from_raw_bytes(k),
      kid: data.kid().map(ToString::to_string),
    })
  }

  /// Creates a new Hmac JSON Web Key from the given slice of bytes.
  pub fn to_jwk(self, data: impl AsRef<[u8]>) -> Result<Jwk> {
    let mut jwk: Jwk = Jwk::new(JwkType::Oct);
    let mut ops: Vec<JwkOperation> = Vec::with_capacity(2);

    ops.push(JwkOperation::Sign);
    ops.push(JwkOperation::Verify);

    jwk.set_alg(self.name());
    jwk.set_use(JwkUse::Signature);
    jwk.set_key_ops(ops);

    if let JwkParams::Oct(params) = jwk.params_mut() {
      params.k = encode_b64(data);
    } else {
      unreachable!()
    }

    Ok(jwk)
  }
}

impl Display for HmacAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
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
  key: PKey<Secret>,
  kid: Option<String>,
}

impl HmacSigner {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JwsSigner for HmacSigner {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    let key: &[u8] = self.key.to_raw_bytes()?;

    match self.alg {
      HmacAlgorithm::HS256 => Ok(sign!(HMAC_SHA256, SHA256_LEN, message, key)),
      HmacAlgorithm::HS384 => Ok(sign!(HMAC_SHA384, SHA384_LEN, message, key)),
      HmacAlgorithm::HS512 => Ok(sign!(HMAC_SHA512, SHA512_LEN, message, key)),
    }
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
  key: PKey<Secret>,
  kid: Option<String>,
}

impl HmacVerifier {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JwsVerifier for HmacVerifier {
  fn alg(&self) -> JwsAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    let key: &[u8] = self.key.to_raw_bytes()?;

    match self.alg {
      HmacAlgorithm::HS256 => verify!(HMAC_SHA256, SHA256_LEN, key, message, signature),
      HmacAlgorithm::HS384 => verify!(HMAC_SHA384, SHA384_LEN, key, message, signature),
      HmacAlgorithm::HS512 => verify!(HMAC_SHA512, SHA512_LEN, key, message, signature),
    }
  }
}

impl Deref for HmacVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}
