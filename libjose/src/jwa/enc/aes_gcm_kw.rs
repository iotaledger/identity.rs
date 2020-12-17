use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::crypto::ciphers::aes;
use crate::crypto::rand::random_bytes;
use crate::crypto::rand::OsRng;
use crate::error::Error;
use crate::error::Result;
use crate::jwa::PKey;
use crate::jwa::Secret;
use crate::jwe::JweAlgorithm;
use crate::jwe::JweDecrypter;
use crate::jwe::JweEncrypter;
use crate::jwe::JweEncryption;
use crate::jwe::JweHeader;
use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkParams;
use crate::jwk::JwkParamsOct;
use crate::jwk::JwkType;
use crate::jwk::JwkUse;
use crate::lib::*;
use crate::utils::decode_b64;
use crate::utils::encode_b64;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum AesGcmKwAlgorithm {
  /// Key wrapping with AES GCM using 128-bit key.
  A128GCMKW,
  /// Key wrapping with AES GCM using 192-bit key.
  A192GCMKW,
  /// Key wrapping with AES GCM using 256-bit key.
  A256GCMKW,
}

impl AesGcmKwAlgorithm {
  /// Returns the JWA identifier of the algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::A128GCMKW => "A128GCMKW",
      Self::A192GCMKW => "A192GCMKW",
      Self::A256GCMKW => "A256GCMKW",
    }
  }

  pub fn key_len(self) -> usize {
    match self {
      Self::A128GCMKW => aes::key_len_AES_GCM_128(),
      Self::A192GCMKW => aes::key_len_AES_GCM_192(),
      Self::A256GCMKW => aes::key_len_AES_GCM_256(),
    }
  }

  pub fn encrypter_from_bytes(self, data: impl AsRef<[u8]>) -> Result<AesGcmKwEncrypter> {
    let data: &[u8] = data.as_ref();

    if data.len() != self.key_len() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(AesGcmKwEncrypter {
      alg: self,
      key: PKey::from_raw_bytes(data),
      kid: None,
    })
  }

  pub fn encrypter_from_jwk(self, data: &Jwk) -> Result<AesGcmKwEncrypter> {
    data.check_use(JwkUse::Encryption)?;
    data.check_ops(JwkOperation::Encrypt)?;
    data.check_alg(self.name())?;
    data.check_kty(JwkType::Oct)?;

    let k: Vec<u8> = match data.params() {
      JwkParams::Oct(JwkParamsOct { k }) => decode_b64(k)?,
      _ => return Err(Error::KeyError(self.name())),
    };

    if k.len() != self.key_len() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(AesGcmKwEncrypter {
      alg: self,
      key: PKey::from_raw_bytes(k),
      kid: data.kid().map(ToString::to_string),
    })
  }

  pub fn decrypter_from_bytes(self, data: impl AsRef<[u8]>) -> Result<AesGcmKwDecrypter> {
    let data: &[u8] = data.as_ref();

    if data.len() != self.key_len() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(AesGcmKwDecrypter {
      alg: self,
      key: PKey::from_raw_bytes(data),
      kid: None,
    })
  }

  pub fn decrypter_from_jwk(self, data: &Jwk) -> Result<AesGcmKwDecrypter> {
    data.check_use(JwkUse::Encryption)?;
    data.check_ops(JwkOperation::Decrypt)?;
    data.check_alg(self.name())?;
    data.check_kty(JwkType::Oct)?;

    let k: Vec<u8> = match data.params() {
      JwkParams::Oct(JwkParamsOct { k }) => decode_b64(k)?,
      _ => return Err(Error::KeyError(self.name())),
    };

    if k.len() != self.key_len() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(AesGcmKwDecrypter {
      alg: self,
      key: PKey::from_raw_bytes(k),
      kid: data.kid().map(ToString::to_string),
    })
  }
}

impl Display for AesGcmKwAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
  }
}

impl From<AesGcmKwAlgorithm> for JweAlgorithm {
  fn from(other: AesGcmKwAlgorithm) -> Self {
    match other {
      AesGcmKwAlgorithm::A128GCMKW => Self::A128GCMKW,
      AesGcmKwAlgorithm::A192GCMKW => Self::A192GCMKW,
      AesGcmKwAlgorithm::A256GCMKW => Self::A256GCMKW,
    }
  }
}

// =============================================================================
// AES GCM Key Wrap Encrypter
// =============================================================================

#[derive(Debug)]
pub struct AesGcmKwEncrypter {
  alg: AesGcmKwAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JweEncrypter for AesGcmKwEncrypter {
  fn alg(&self) -> JweAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn cek(&self, _: JweEncryption, _: &JweHeader, _: &mut JweHeader) -> Result<Option<Cow<[u8]>>> {
    Ok(None)
  }

  fn encrypt(&self, cek: &[u8], _: &JweHeader, output: &mut JweHeader) -> Result<Option<Vec<u8>>> {
    let iv: Vec<u8> = random_bytes(32, OsRng)?;
    let key: &[u8] = self.key.to_raw_bytes()?;

    let (ciphertext, tag): _ = match self.alg {
      AesGcmKwAlgorithm::A128GCMKW => aes::encrypt_AES_GCM_128(cek, key, &iv, &[])?,
      AesGcmKwAlgorithm::A192GCMKW => aes::encrypt_AES_GCM_192(cek, key, &iv, &[])?,
      AesGcmKwAlgorithm::A256GCMKW => aes::encrypt_AES_GCM_256(cek, key, &iv, &[])?,
    };

    output.set_iv(encode_b64(iv));
    output.set_tag(encode_b64(tag));

    Ok(Some(ciphertext))
  }
}

impl Deref for AesGcmKwEncrypter {
  type Target = dyn JweEncrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// AES GCM Key Wrap Decrypter
// =============================================================================

#[derive(Debug)]
pub struct AesGcmKwDecrypter {
  alg: AesGcmKwAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JweDecrypter for AesGcmKwDecrypter {
  fn alg(&self) -> JweAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn decrypt(&self, cek: Option<&[u8]>, _: JweEncryption, header: &JweHeader) -> Result<Cow<[u8]>> {
    let cek: &[u8] = cek.ok_or(Error::EncError("Content Encryption Key is Required"))?;
    let key: &[u8] = self.key.to_raw_bytes()?;

    let iv: Vec<u8> = header
      .iv()
      .ok_or(Error::EncError("Missing Header Claim: `iv`"))
      .and_then(decode_b64)?;

    let tag: Vec<u8> = header
      .tag()
      .ok_or(Error::EncError("Missing Header Claim: `tag`"))
      .and_then(decode_b64)?;

    match self.alg {
      AesGcmKwAlgorithm::A128GCMKW => {
        aes::decrypt_AES_GCM_128(cek, key, &iv, &[], &tag).map(Cow::Owned)
      }
      AesGcmKwAlgorithm::A192GCMKW => {
        aes::decrypt_AES_GCM_192(cek, key, &iv, &[], &tag).map(Cow::Owned)
      }
      AesGcmKwAlgorithm::A256GCMKW => {
        aes::decrypt_AES_GCM_256(cek, key, &iv, &[], &tag).map(Cow::Owned)
      }
    }
  }
}

impl Deref for AesGcmKwDecrypter {
  type Target = dyn JweDecrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}
