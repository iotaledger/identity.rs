use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

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
pub enum Ecdh1PUAlgorithm {
  /// ECDH One-Pass Unified Model using one-pass KDF
  ECDH_1PU,
  /// ECDH-1PU using one-pass KDF and CEK wrapped with "A128KW"
  ECDH_1PU_A128KW,
  /// ECDH-1PU using one-pass KDF and CEK wrapped with "A192KW"
  ECDH_1PU_A192KW,
  /// ECDH-1PU using one-pass KDF and CEK wrapped with "A256KW"
  ECDH_1PU_A256KW,
}

impl Ecdh1PUAlgorithm {
  /// Returns the JWA identifier of the algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::ECDH_1PU => "ECDH-1PU",
      Self::ECDH_1PU_A128KW => "ECDH-1PU+A128KW",
      Self::ECDH_1PU_A192KW => "ECDH-1PU+A192KW",
      Self::ECDH_1PU_A256KW => "ECDH-1PU+A256KW",
    }
  }

  pub fn key_len(self) -> usize {
    match self {
      Self::ECDH_1PU => unreachable!(),
      Self::ECDH_1PU_A128KW => aes::key_len_AES_GCM_128(),
      Self::ECDH_1PU_A192KW => aes::key_len_AES_GCM_192(),
      Self::ECDH_1PU_A256KW => aes::key_len_AES_GCM_256(),
    }
  }

  pub fn encrypter_from_bytes(
    self,
    curve: impl Into<EcdhKeyType>,
    public: impl AsRef<[u8]>,
    secret: impl AsRef<[u8]>,
  ) -> Result<Ecdh1PUEncrypter> {
    let kty: EcdhKeyType = curve.into();
    let key_pub: PKey<Public> = kty.public_from_bytes(public)?;
    let key_sec: PKey<Secret> = kty.private_from_bytes(secret)?;

    Ok(Ecdh1PUEncrypter {
      alg: self,
      kty,
      key_pub,
      key_sec,
      apu: None,
      apv: None,
      kid: None,
    })
  }

  pub fn encrypter_from_jwk(self, public: &Jwk, secret: &Jwk) -> Result<Ecdh1PUEncrypter> {
    let (key_pub, kty_pub): _ = EcdhKeyType::public_from_jwk(self.name(), public)?;
    let (key_sec, kty_sec): _ = EcdhKeyType::private_from_jwk(self.name(), secret)?;

    if kty_pub != kty_sec {
      return Err(Error::KeyError("Invalid Curve"));
    }

    Ok(Ecdh1PUEncrypter {
      alg: self,
      kty: kty_pub,
      key_pub,
      key_sec,
      apu: None,
      apv: None,
      kid: public.kid().map(ToString::to_string),
    })
  }

  pub fn decrypter_from_bytes(
    self,
    curve: impl Into<EcdhKeyType>,
    public: impl AsRef<[u8]>,
    secret: impl AsRef<[u8]>,
  ) -> Result<Ecdh1PUDecrypter> {
    let kty: EcdhKeyType = curve.into();
    let key_pub: PKey<Public> = kty.public_from_bytes(public)?;
    let key_sec: PKey<Secret> = kty.private_from_bytes(secret)?;

    Ok(Ecdh1PUDecrypter {
      alg: self,
      kty,
      key_pub,
      key_sec,
      kid: None,
    })
  }

  pub fn decrypter_from_jwk(self, public: &Jwk, secret: &Jwk) -> Result<Ecdh1PUDecrypter> {
    let (key_pub, kty_pub): _ = EcdhKeyType::public_from_jwk(self.name(), public)?;
    let (key_sec, kty_sec): _ = EcdhKeyType::private_from_jwk(self.name(), secret)?;

    if kty_pub != kty_sec {
      return Err(Error::KeyError("Invalid Curve"));
    }

    Ok(Ecdh1PUDecrypter {
      alg: self,
      kty: kty_sec,
      key_pub,
      key_sec,
      kid: secret.kid().map(ToString::to_string),
    })
  }
}

impl Display for Ecdh1PUAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
  }
}

impl From<Ecdh1PUAlgorithm> for JweAlgorithm {
  fn from(other: Ecdh1PUAlgorithm) -> Self {
    match other {
      Ecdh1PUAlgorithm::ECDH_1PU => Self::ECDH_1PU,
      Ecdh1PUAlgorithm::ECDH_1PU_A128KW => Self::ECDH_1PU_A128KW,
      Ecdh1PUAlgorithm::ECDH_1PU_A192KW => Self::ECDH_1PU_A192KW,
      Ecdh1PUAlgorithm::ECDH_1PU_A256KW => Self::ECDH_1PU_A256KW,
    }
  }
}

// =============================================================================
// ECDH One-Pass Unified Model Encrypter
// =============================================================================

#[derive(Debug)]
pub struct Ecdh1PUEncrypter {
  alg: Ecdh1PUAlgorithm,
  kty: EcdhKeyType,
  key_pub: PKey<Public>,
  key_sec: PKey<Secret>,
  apu: Option<Vec<u8>>,
  apv: Option<Vec<u8>>,
  kid: Option<String>,
}

impl Ecdh1PUEncrypter {
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
    let ze: Vec<u8> = self.kty.diffie_hellman(&eph_sec, &self.key_pub)?;
    let zs: Vec<u8> = self.kty.diffie_hellman(&self.key_sec, &self.key_pub)?;
    let z: Vec<u8> = [ze, zs].concat();

    // Set the ephemeral public key claim
    header.set_epk(eph_pub);

    // Concat KDF
    concat_kdf(alg, len, &z, apu, apv)
  }
}

impl JweEncrypter for Ecdh1PUEncrypter {
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
    if let Ecdh1PUAlgorithm::ECDH_1PU = self.alg {
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
      Ecdh1PUAlgorithm::ECDH_1PU => Ok(None),
      Ecdh1PUAlgorithm::ECDH_1PU_A128KW => self
        .__derive(output, self.alg.name(), self.alg.key_len())
        .and_then(|secret| aes::wrap_AES_GCM_128(key, &secret))
        .map(Some),
      Ecdh1PUAlgorithm::ECDH_1PU_A192KW => self
        .__derive(output, self.alg.name(), self.alg.key_len())
        .and_then(|secret| aes::wrap_AES_GCM_192(key, &secret))
        .map(Some),
      Ecdh1PUAlgorithm::ECDH_1PU_A256KW => self
        .__derive(output, self.alg.name(), self.alg.key_len())
        .and_then(|secret| aes::wrap_AES_GCM_256(key, &secret))
        .map(Some),
    }
  }
}

impl Deref for Ecdh1PUEncrypter {
  type Target = dyn JweEncrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// ECDH One-Pass Unified Model Decrypter
// =============================================================================

#[derive(Debug)]
pub struct Ecdh1PUDecrypter {
  alg: Ecdh1PUAlgorithm,
  kty: EcdhKeyType,
  key_pub: PKey<Public>,
  key_sec: PKey<Secret>,
  kid: Option<String>,
}

impl Ecdh1PUDecrypter {
  pub fn set_kid(&mut self, kid: impl Into<String>) {
    self.kid = Some(kid.into());
  }
}

impl JweDecrypter for Ecdh1PUDecrypter {
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
    if let Ecdh1PUAlgorithm::ECDH_1PU = self.alg {
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
      .ok_or(Error::EncError("Missing Ephemeral Public Key"))
      .and_then(|public| {
        let ze: Vec<u8> = self.kty.diffie_hellman(&self.key_sec, &public)?;
        let zs: Vec<u8> = self.kty.diffie_hellman(&self.key_sec, &self.key_pub)?;
        let z: Vec<u8> = [ze, zs].concat();

        Ok(z)
      })?;

    let key: Vec<u8> = if let Ecdh1PUAlgorithm::ECDH_1PU = self.alg {
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
        Ecdh1PUAlgorithm::ECDH_1PU => unreachable!(),
        Ecdh1PUAlgorithm::ECDH_1PU_A128KW => {
          aes::unwrap_AES_GCM_128(key, &shared, self.alg.key_len())?
        }
        Ecdh1PUAlgorithm::ECDH_1PU_A192KW => {
          aes::unwrap_AES_GCM_192(key, &shared, self.alg.key_len())?
        }
        Ecdh1PUAlgorithm::ECDH_1PU_A256KW => {
          aes::unwrap_AES_GCM_256(key, &shared, self.alg.key_len())?
        }
      }
    };

    Ok(Cow::Owned(key))
  }
}

impl Deref for Ecdh1PUDecrypter {
  type Target = dyn JweDecrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}
