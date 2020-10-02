use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use crypto::signers::rsa;
use crypto::rand::OsRng;

use crate::error::CryptoError;
use crate::error::Result;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkUse;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::utils::pem_decode;
use crate::utils::Pem;
use crate::jwk::JwkParamsRsa;
use crate::jwk::JwkParams;
use crate::utils::decode_b64;

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
  /// Returns the JWA identifier of the algorithm.
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

  /// Creates a new random RSA `PrivateKey`.
  pub fn generate_key(self, bits: rsa::RsaBits) -> Result<rsa::PrivateKey> {
    rsa::PrivateKey::random(&mut OsRng, bits).map_err(Into::into)
  }

  /// Creates a new `RsaSigner` from DER-encoded material in PKCS#8 form.
  pub fn signer_from_der(self, data: impl AsRef<[u8]>) -> Result<RsaSigner> {
    let key: rsa::PrivateKey = rsa::PrivateKey::from_slice(data)?;

    Ok(RsaSigner {
      alg: self,
      key,
      kid: None,
    })
  }

  /// Creates a new `RsaSigner` from a PEM-encoded document.
  pub fn signer_from_pem(self, data: impl AsRef<[u8]>) -> Result<RsaSigner> {
    let pem: Pem = pem_decode(&data)?;

    let key: rsa::PrivateKey = match pem.pem_type.as_str() {
      "RSA PRIVATE KEY" => rsa::PrivateKey::from_pkcs1(pem.pem_data)?,
      "PRIVATE KEY" => rsa::PrivateKey::from_pkcs8(pem.pem_data)?,
      _ => return Err(CryptoError::InvalidKeyFormat(self.name()).into()),
    };

    Ok(RsaSigner {
      alg: self,
      key,
      kid: None,
    })
  }

  /// Creates a new `RsaSigner` from a JSON Web Key.
  pub fn signer_from_jwk(self, data: &Jwk) -> Result<RsaSigner> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Sign)?;
    data.check_alg(self.name())?;

    let params: &JwkParamsRsa = match data.params() {
      Some(JwkParams::Rsa(params)) => params,
      Some(_) | None => return Err(CryptoError::InvalidKeyFormat(self.name()).into()),
    };

    // TODO: Multi-prime key
    if params.oth.is_some() {
      return Err(CryptoError::InvalidKeyFormat(self.name()).into());
    }

    let n: rsa::BigUint = decode_b64_biguint(&params.n)?;
    let e: rsa::BigUint = decode_b64_biguint(&params.e)?;
    let d: rsa::BigUint = decode_b64_biguint_opt(params.d.as_ref())?;
    let p: rsa::BigUint = decode_b64_biguint_opt(params.p.as_ref())?;
    let q: rsa::BigUint = decode_b64_biguint_opt(params.q.as_ref())?;

    let _dp: rsa::BigUint = decode_b64_biguint_opt(params.dp.as_ref())?;
    let _dq: rsa::BigUint = decode_b64_biguint_opt(params.dq.as_ref())?;
    let _qi: rsa::BigUint = decode_b64_biguint_opt(params.qi.as_ref())?;

    let key: rsa::PrivateKey = rsa::PrivateKey::new(n, e, d, vec![p, q])?;
    let kid: Option<String> = data.kid().map(ToString::to_string);

    // TODO: Validate precomputed properties

    Ok(RsaSigner {
      alg: self,
      key,
      kid,
    })
  }

  /// Creates a new `RsaVerifier` from DER-encoded material in PKCS#8 form.
  pub fn verifier_from_der(self, data: impl AsRef<[u8]>) -> Result<RsaVerifier> {
    let key: rsa::PublicKey = rsa::PublicKey::from_slice(data)?;

    Ok(RsaVerifier {
      alg: self,
      key,
      kid: None,
    })
  }

  /// Creates a new `RsaVerifier` from a PEM-encoded document.
  pub fn verifier_from_pem(self, data: impl AsRef<[u8]>) -> Result<RsaVerifier> {
    let pem: Pem = pem_decode(&data)?;

    let key: rsa::PublicKey = match pem.pem_type.as_str() {
      "RSA PUBLIC KEY" => rsa::PublicKey::from_pkcs1(&pem.pem_data)?,
      "PUBLIC KEY" => rsa::PublicKey::from_pkcs8(&pem.pem_data)?,
      _ => return Err(CryptoError::InvalidKeyFormat(self.name()).into()),
    };

    Ok(RsaVerifier {
      alg: self,
      key,
      kid: None,
    })
  }

  /// Creates a new `RsaVerifier` from a JSON Web Key.
  pub fn verifier_from_jwk(self, data: &Jwk) -> Result<RsaVerifier> {
    data.check_use(&JwkUse::Signature)?;
    data.check_ops(&JwkOperation::Verify)?;
    data.check_alg(self.name())?;

    let params: &JwkParamsRsa = match data.params() {
      Some(JwkParams::Rsa(params)) => params,
      Some(_) | None => return Err(CryptoError::InvalidKeyFormat(self.name()).into()),
    };

    let n: rsa::BigUint = decode_b64_biguint(&params.n)?;
    let e: rsa::BigUint = decode_b64_biguint(&params.e)?;

    let key: rsa::PublicKey = rsa::PublicKey::new(n, e)?;
    let kid: Option<String> = data.kid().map(ToString::to_string);

    Ok(RsaVerifier {
      alg: self,
      key,
      kid,
    })
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

// =============================================================================
// RSA Signer
// =============================================================================

#[derive(Debug)]
pub struct RsaSigner {
  alg: RsaAlgorithm,
  key: rsa::PrivateKey,
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
    let signature: Vec<u8> = match self.alg {
      RsaAlgorithm::RS256 => self.key.sign_pkcs1_sha256(message)?,
      RsaAlgorithm::RS384 => self.key.sign_pkcs1_sha384(message)?,
      RsaAlgorithm::RS512 => self.key.sign_pkcs1_sha512(message)?,
      RsaAlgorithm::PS256 => self.key.sign_pss_sha256(OsRng, message)?,
      RsaAlgorithm::PS384 => self.key.sign_pss_sha384(OsRng, message)?,
      RsaAlgorithm::PS512 => self.key.sign_pss_sha512(OsRng, message)?,
    };

    Ok(signature)
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
  key: rsa::PublicKey,
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
    match self.alg {
      RsaAlgorithm::RS256 => self.key.verify_pkcs1_sha256(message, signature)?,
      RsaAlgorithm::RS384 => self.key.verify_pkcs1_sha384(message, signature)?,
      RsaAlgorithm::RS512 => self.key.verify_pkcs1_sha512(message, signature)?,
      RsaAlgorithm::PS256 => self.key.verify_pss_sha256(OsRng, message, signature)?,
      RsaAlgorithm::PS384 => self.key.verify_pss_sha384(OsRng, message, signature)?,
      RsaAlgorithm::PS512 => self.key.verify_pss_sha512(OsRng, message, signature)?,
    }

    Ok(())
  }
}

impl Deref for RsaVerifier {
  type Target = dyn JwsVerifier;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// Helpers
// =============================================================================

fn decode_b64_biguint(data: impl AsRef<[u8]>) -> Result<rsa::BigUint> {
  decode_b64(data.as_ref()).map(|data| rsa::BigUint::from_bytes_be(&data))
}

fn decode_b64_biguint_opt(data: Option<impl AsRef<[u8]>>) -> Result<rsa::BigUint> {
  match data {
    Some(ref data) => decode_b64_biguint(data.as_ref()),
    None => todo!("Private Exponent"),
  }
}
