use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use crypto::ciphers::aes::AES_128_GCM;
use crypto::ciphers::aes::AES_192_GCM;
use crypto::ciphers::aes::AES_256_GCM;
use crypto::ciphers::chacha::CHACHA20_POLY1305;
use crypto::ciphers::chacha::XCHACHA20_POLY1305;

use crate::crypto::ciphers::aes;
use crate::error::Error;
use crate::error::Result;
use crate::jwa::EcdhKeyType;
use crate::jwa::PKey;
use crate::jwa::Public;
use crate::jwa::Secret;
use crate::jwe::JweAlgorithm;
use crate::jwe::JweDecrypter;
use crate::jwe::JweEncrypter;
use crate::jwe::JweEncryption;
use crate::jwe::JweHeader;
use crate::jwk::Jwk;
use crate::lib::*;
use crate::utils::concat_kdf;
use crate::utils::decode_b64;
use crate::utils::encode_b64;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum EcdhESAlgorithm {
  /// Elliptic Curve Diffie-Hellman Ephemeral Static key agreement using Concat
  /// KDF.
  ECDH_ES,
  /// ECDH-ES using Concat KDF and CEK wrapped with "A128KW".
  ECDH_ES_A128KW,
  /// ECDH-ES using Concat KDF and CEK wrapped with "A192KW".
  ECDH_ES_A192KW,
  /// ECDH-ES using Concat KDF and CEK  wrapped with "A256KW".
  ECDH_ES_A256KW,
  /// ECDH-ES using Concat KDF and CEK wrapped with C20PKW.
  ECDH_ES_C20PKW,
  /// ECDH-ES using Concat KDF and CEK wrapped with XC20PKW.
  ECDH_ES_XC20PKW,
}

impl EcdhESAlgorithm {
  /// Returns the JWA identifier of the algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::ECDH_ES => "ECDH-ES",
      Self::ECDH_ES_A128KW => "ECDH-ES+A128KW",
      Self::ECDH_ES_A192KW => "ECDH-ES+A192KW",
      Self::ECDH_ES_A256KW => "ECDH-ES+A256KW",
      Self::ECDH_ES_C20PKW => "ECDH-ES+C20PKW",
      Self::ECDH_ES_XC20PKW => "ECDH-ES+XC20PKW",
    }
  }

  pub fn key_len(self) -> usize {
    match self {
      Self::ECDH_ES => unreachable!(),
      Self::ECDH_ES_A128KW => AES_128_GCM::KEY_LENGTH,
      Self::ECDH_ES_A192KW => AES_192_GCM::KEY_LENGTH,
      Self::ECDH_ES_A256KW => AES_256_GCM::KEY_LENGTH,
      Self::ECDH_ES_C20PKW => CHACHA20_POLY1305::KEY_LENGTH,
      Self::ECDH_ES_XC20PKW => XCHACHA20_POLY1305::KEY_LENGTH,
    }
  }

  pub fn encrypter_from_bytes(
    self,
    curve: impl Into<EcdhKeyType>,
    data: impl AsRef<[u8]>,
  ) -> Result<EcdhESEncrypter> {
    let kty: EcdhKeyType = curve.into();
    let key: PKey<Public> = kty.public_from_bytes(data)?;

    Ok(EcdhESEncrypter {
      alg: self,
      kty,
      key,
      apu: None,
      apv: None,
      kid: None,
    })
  }

  pub fn encrypter_from_jwk(self, data: &Jwk) -> Result<EcdhESEncrypter> {
    let (key, kty): _ = EcdhKeyType::public_from_jwk(self.name(), data)?;

    Ok(EcdhESEncrypter {
      alg: self,
      kty,
      key,
      apu: None,
      apv: None,
      kid: data.kid().map(ToString::to_string),
    })
  }

  pub fn decrypter_from_bytes(
    self,
    curve: impl Into<EcdhKeyType>,
    data: impl AsRef<[u8]>,
  ) -> Result<EcdhESDecrypter> {
    let kty: EcdhKeyType = curve.into();
    let key: PKey<Secret> = kty.private_from_bytes(data)?;

    Ok(EcdhESDecrypter {
      alg: self,
      kty,
      key,
      kid: None,
    })
  }

  pub fn decrypter_from_jwk(self, data: &Jwk) -> Result<EcdhESDecrypter> {
    let (key, kty): _ = EcdhKeyType::private_from_jwk(self.name(), data)?;

    Ok(EcdhESDecrypter {
      alg: self,
      kty,
      key,
      kid: data.kid().map(ToString::to_string),
    })
  }
}

impl Display for EcdhESAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
  }
}

impl From<EcdhESAlgorithm> for JweAlgorithm {
  fn from(other: EcdhESAlgorithm) -> Self {
    match other {
      EcdhESAlgorithm::ECDH_ES => Self::ECDH_ES,
      EcdhESAlgorithm::ECDH_ES_A128KW => Self::ECDH_ES_A128KW,
      EcdhESAlgorithm::ECDH_ES_A192KW => Self::ECDH_ES_A192KW,
      EcdhESAlgorithm::ECDH_ES_A256KW => Self::ECDH_ES_A256KW,
      EcdhESAlgorithm::ECDH_ES_C20PKW => Self::ECDH_ES_C20PKW,
      EcdhESAlgorithm::ECDH_ES_XC20PKW => Self::ECDH_ES_XC20PKW,
    }
  }
}

// =============================================================================
// ECDH Ephemeral Static Encrypter
// =============================================================================

#[derive(Debug)]
pub struct EcdhESEncrypter {
  alg: EcdhESAlgorithm,
  kty: EcdhKeyType,
  key: PKey<Public>,
  apu: Option<Vec<u8>>,
  apv: Option<Vec<u8>>,
  kid: Option<String>,
}

impl EcdhESEncrypter {
  pub fn set_apu(&mut self, apu: impl Into<Vec<u8>>) {
    self.apu = Some(apu.into());
  }

  pub fn set_apv(&mut self, apv: impl Into<Vec<u8>>) {
    self.apv = Some(apv.into());
  }

  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }

  fn __derive(&self, header: &mut JweHeader, alg: &str, len: usize) -> Result<Vec<u8>> {
    let __apu: Vec<u8>;
    let __apv: Vec<u8>;

    let apu: &[u8] = match header.apu() {
      Some(value) => {
        __apu = decode_b64(value)?;
        &__apu
      }
      None => match self.apu.as_deref() {
        Some(value) => {
          header.set_apu(encode_b64(value));
          value
        }
        None => &[],
      },
    };

    let apv: &[u8] = match header.apv() {
      Some(value) => {
        __apv = decode_b64(value)?;
        &__apv
      }
      None => match self.apv.as_deref() {
        Some(value) => {
          header.set_apv(encode_b64(value));
          value
        }
        None => &[],
      },
    };

    // Generate an ephemeral key pair
    let (eph_pub, eph_sec): (Jwk, PKey<Secret>) = self.kty.generate_epk()?;

    // Compute the shared secret
    let z: Vec<u8> = self.kty.diffie_hellman(&eph_sec, &self.key)?;

    // Set the ephemeral public key claim
    header.set_epk(eph_pub);

    // Concat KDF
    concat_kdf(alg, len, &z, apu, apv)
  }
}

impl JweEncrypter for EcdhESEncrypter {
  fn alg(&self) -> JweAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn cek(
    &self,
    enc: JweEncryption,
    _: &JweHeader,
    output: &mut JweHeader,
  ) -> Result<Option<Cow<[u8]>>> {
    if let EcdhESAlgorithm::ECDH_ES = self.alg {
      self
        .__derive(output, enc.name(), enc.key_len())
        .map(Cow::Owned)
        .map(Some)
    } else {
      Ok(None)
    }
  }

  fn encrypt(&self, key: &[u8], _: &JweHeader, output: &mut JweHeader) -> Result<Option<Vec<u8>>> {
    match self.alg {
      EcdhESAlgorithm::ECDH_ES => Ok(None),
      EcdhESAlgorithm::ECDH_ES_A128KW => self
        .__derive(output, self.alg.name(), self.alg.key_len())
        .and_then(|secret| aes::wrap_AES_GCM_128(key, &secret))
        .map(Some),
      EcdhESAlgorithm::ECDH_ES_A192KW => self
        .__derive(output, self.alg.name(), self.alg.key_len())
        .and_then(|secret| aes::wrap_AES_GCM_192(key, &secret))
        .map(Some),
      EcdhESAlgorithm::ECDH_ES_A256KW => self
        .__derive(output, self.alg.name(), self.alg.key_len())
        .and_then(|secret| aes::wrap_AES_GCM_256(key, &secret))
        .map(Some),
      EcdhESAlgorithm::ECDH_ES_C20PKW => {
        todo!("EcdhESEncrypter::encrypt(ECDH_ES_C20PKW)")
      }
      EcdhESAlgorithm::ECDH_ES_XC20PKW => {
        todo!("EcdhESEncrypter::encrypt(ECDH_ES_XC20PKW)")
      }
    }
  }
}

impl Deref for EcdhESEncrypter {
  type Target = dyn JweEncrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// ECDH Ephemeral Static Decrypter
// =============================================================================

#[derive(Debug)]
pub struct EcdhESDecrypter {
  alg: EcdhESAlgorithm,
  kty: EcdhKeyType,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl EcdhESDecrypter {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JweDecrypter for EcdhESDecrypter {
  fn alg(&self) -> JweAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn decrypt(
    &self,
    key: Option<&[u8]>,
    enc: JweEncryption,
    header: &JweHeader,
  ) -> Result<Cow<[u8]>> {
    if let EcdhESAlgorithm::ECDH_ES = self.alg {
      if key.is_some() {
        return Err(Error::EncError("Invalid Encrypted Key"));
      }
    } else if key.is_none() {
      return Err(Error::EncError("Missing Encrypted Key"));
    }

    let apu: Option<Vec<u8>> = header.apu().map(decode_b64).transpose()?;
    let apv: Option<Vec<u8>> = header.apv().map(decode_b64).transpose()?;

    let derived: Vec<u8> = header
      .epk()
      .map(|epk| self.kty.expand_epk(&epk))
      .transpose()?
      .ok_or(Error::MissingClaim("epk"))
      .and_then(|public| self.kty.diffie_hellman(&self.key, &public))?;

    let key: Vec<u8> = if let EcdhESAlgorithm::ECDH_ES = self.alg {
      concat_kdf(
        enc.name(),
        enc.key_len(),
        &derived,
        apu.as_deref().unwrap_or_default(),
        apv.as_deref().unwrap_or_default(),
      )?
    } else {
      let shared: Vec<u8> = concat_kdf(
        self.alg.name(),
        self.alg.key_len(),
        &derived,
        apu.as_deref().unwrap_or_default(),
        apv.as_deref().unwrap_or_default(),
      )?;

      let key: &[u8] = key.unwrap_or_else(|| unreachable!());

      match self.alg {
        EcdhESAlgorithm::ECDH_ES => unreachable!(),
        EcdhESAlgorithm::ECDH_ES_A128KW => {
          aes::unwrap_AES_GCM_128(key, &shared, self.alg.key_len())?
        }
        EcdhESAlgorithm::ECDH_ES_A192KW => {
          aes::unwrap_AES_GCM_192(key, &shared, self.alg.key_len())?
        }
        EcdhESAlgorithm::ECDH_ES_A256KW => {
          aes::unwrap_AES_GCM_256(key, &shared, self.alg.key_len())?
        }
        EcdhESAlgorithm::ECDH_ES_C20PKW => {
          todo!("EcdhESDecrypter::encrypt(ECDH_ES_C20PKW)")
        }
        EcdhESAlgorithm::ECDH_ES_XC20PKW => {
          todo!("EcdhESDecrypter::encrypt(ECDH_ES_XC20PKW)")
        }
      }
    };

    Ok(Cow::Owned(key))
  }
}

impl Deref for EcdhESDecrypter {
  type Target = dyn JweDecrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}
