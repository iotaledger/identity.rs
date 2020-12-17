use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::crypto::ciphers::aes;
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum AesKwAlgorithm {
  /// AES Key Wrap with default initial value using 128-bit key.
  A128KW,
  /// AES Key Wrap with default initial value using 192-bit key.
  A192KW,
  /// AES Key Wrap with default initial value using 256-bit key.
  A256KW,
}

impl AesKwAlgorithm {
  /// Returns the JWA identifier of the algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::A128KW => "A128KW",
      Self::A192KW => "A192KW",
      Self::A256KW => "A256KW",
    }
  }

  pub fn key_len(self) -> usize {
    match self {
      Self::A128KW => aes::key_len_AES_GCM_128(),
      Self::A192KW => aes::key_len_AES_GCM_192(),
      Self::A256KW => aes::key_len_AES_GCM_256(),
    }
  }

  pub fn encrypter_from_bytes(self, data: impl AsRef<[u8]>) -> Result<AesKwEncrypter> {
    let data: &[u8] = data.as_ref();

    if data.len() != self.key_len() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(AesKwEncrypter {
      alg: self,
      key: PKey::from_raw_bytes(data),
      kid: None,
    })
  }

  pub fn encrypter_from_jwk(self, data: &Jwk) -> Result<AesKwEncrypter> {
    data.check_use(JwkUse::Encryption)?;
    data.check_ops(JwkOperation::WrapKey)?;
    data.check_alg(self.name())?;
    data.check_kty(JwkType::Oct)?;

    let k: Vec<u8> = match data.params() {
      JwkParams::Oct(JwkParamsOct { k }) => decode_b64(k)?,
      _ => return Err(Error::KeyError(self.name())),
    };

    if k.len() != self.key_len() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(AesKwEncrypter {
      alg: self,
      key: PKey::from_raw_bytes(k),
      kid: data.kid().map(ToString::to_string),
    })
  }

  pub fn decrypter_from_bytes(self, data: impl AsRef<[u8]>) -> Result<AesKwDecrypter> {
    let data: &[u8] = data.as_ref();

    if data.len() != self.key_len() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(AesKwDecrypter {
      alg: self,
      key: PKey::from_raw_bytes(data),
      kid: None,
    })
  }

  pub fn decrypter_from_jwk(self, data: &Jwk) -> Result<AesKwDecrypter> {
    data.check_use(JwkUse::Encryption)?;
    data.check_ops(JwkOperation::UnwrapKey)?;
    data.check_alg(self.name())?;
    data.check_kty(JwkType::Oct)?;

    let k: Vec<u8> = match data.params() {
      JwkParams::Oct(JwkParamsOct { k }) => decode_b64(k)?,
      _ => return Err(Error::KeyError(self.name())),
    };

    if k.len() != self.key_len() {
      return Err(Error::KeyError(self.name()));
    }

    Ok(AesKwDecrypter {
      alg: self,
      key: PKey::from_raw_bytes(k),
      kid: data.kid().map(ToString::to_string),
    })
  }
}

impl Display for AesKwAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
  }
}

impl From<AesKwAlgorithm> for JweAlgorithm {
  fn from(other: AesKwAlgorithm) -> Self {
    match other {
      AesKwAlgorithm::A128KW => Self::A128KW,
      AesKwAlgorithm::A192KW => Self::A192KW,
      AesKwAlgorithm::A256KW => Self::A256KW,
    }
  }
}

// =============================================================================
// AES Key Wrap Encrypter
// =============================================================================

#[derive(Debug)]
pub struct AesKwEncrypter {
  alg: AesKwAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JweEncrypter for AesKwEncrypter {
  fn alg(&self) -> JweAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn cek(&self, _: JweEncryption, _: &JweHeader, _: &mut JweHeader) -> Result<Option<Cow<[u8]>>> {
    Ok(None)
  }

  fn encrypt(&self, cek: &[u8], _: &JweHeader, _: &mut JweHeader) -> Result<Option<Vec<u8>>> {
    let key: &[u8] = self.key.to_raw_bytes()?;

    match self.alg {
      AesKwAlgorithm::A128KW => aes::wrap_AES_GCM_128(cek, key).map(Some),
      AesKwAlgorithm::A192KW => aes::wrap_AES_GCM_192(cek, key).map(Some),
      AesKwAlgorithm::A256KW => aes::wrap_AES_GCM_256(cek, key).map(Some),
    }
  }
}

impl Deref for AesKwEncrypter {
  type Target = dyn JweEncrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// AES Key Wrap Decrypter
// =============================================================================

#[derive(Debug)]
pub struct AesKwDecrypter {
  alg: AesKwAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JweDecrypter for AesKwDecrypter {
  fn alg(&self) -> JweAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn decrypt(&self, cek: Option<&[u8]>, _: JweEncryption, _: &JweHeader) -> Result<Cow<[u8]>> {
    let cek: &[u8] = cek.ok_or(Error::EncError("Content Encryption Key is Required"))?;
    let key: &[u8] = self.key.to_raw_bytes()?;

    match self.alg {
      AesKwAlgorithm::A128KW => aes::wrap_AES_GCM_128(cek, key).map(Cow::Owned),
      AesKwAlgorithm::A192KW => aes::wrap_AES_GCM_192(cek, key).map(Cow::Owned),
      AesKwAlgorithm::A256KW => aes::wrap_AES_GCM_256(cek, key).map(Cow::Owned),
    }
  }
}

impl Deref for AesKwDecrypter {
  type Target = dyn JweDecrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}
