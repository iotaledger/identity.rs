use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

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
pub enum DirectAlgorithm {
  /// Direct use of a shared symmetric key as the CEK.
  DIR,
}

impl DirectAlgorithm {
  /// Returns the JWA identifier of the algorithm.
  pub const fn name(self) -> &'static str {
    match self {
      Self::DIR => "dir",
    }
  }

  pub fn encrypter_from_bytes(self, data: impl AsRef<[u8]>) -> Result<DirectEncrypter> {
    Ok(DirectEncrypter {
      alg: self,
      key: PKey::from_raw_bytes(data),
      kid: None,
    })
  }

  pub fn encrypter_from_jwk(self, data: &Jwk) -> Result<DirectEncrypter> {
    data.check_use(JwkUse::Encryption)?;
    data.check_ops(JwkOperation::Encrypt)?;
    data.check_alg(self.name())?;
    data.check_kty(JwkType::Oct)?;

    let k: Vec<u8> = match data.params() {
      JwkParams::Oct(JwkParamsOct { k }) => decode_b64(k)?,
      _ => return Err(Error::KeyError(self.name())),
    };

    Ok(DirectEncrypter {
      alg: self,
      key: PKey::from_raw_bytes(k),
      kid: data.kid().map(ToString::to_string),
    })
  }

  pub fn decrypter_from_bytes(self, data: impl AsRef<[u8]>) -> Result<DirectDecrypter> {
    Ok(DirectDecrypter {
      alg: self,
      key: PKey::from_raw_bytes(data),
      kid: None,
    })
  }

  pub fn decrypter_from_jwk(self, data: &Jwk) -> Result<DirectDecrypter> {
    data.check_use(JwkUse::Encryption)?;
    data.check_ops(JwkOperation::Decrypt)?;
    data.check_alg(self.name())?;
    data.check_kty(JwkType::Oct)?;

    let k: Vec<u8> = match data.params() {
      JwkParams::Oct(JwkParamsOct { k }) => decode_b64(k)?,
      _ => return Err(Error::KeyError(self.name())),
    };

    Ok(DirectDecrypter {
      alg: self,
      key: PKey::from_raw_bytes(k),
      kid: data.kid().map(ToString::to_string),
    })
  }

  pub fn to_jwk(self, data: impl AsRef<[u8]>) -> Result<Jwk> {
    let mut jwk: Jwk = Jwk::new(JwkType::Oct);
    let mut ops: Vec<JwkOperation> = Vec::with_capacity(2);

    ops.push(JwkOperation::Encrypt);
    ops.push(JwkOperation::Decrypt);

    jwk.set_alg(self.name());
    jwk.set_use(JwkUse::Encryption);
    jwk.set_key_ops(ops);

    if let JwkParams::Oct(params) = jwk.params_mut() {
      params.k = encode_b64(data);
    } else {
      unreachable!()
    }

    Ok(jwk)
  }
}

impl Display for DirectAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
  }
}

impl From<DirectAlgorithm> for JweAlgorithm {
  fn from(other: DirectAlgorithm) -> Self {
    match other {
      DirectAlgorithm::DIR => Self::DIR,
    }
  }
}

// =============================================================================
// Direct Encrypter
// =============================================================================

#[derive(Debug)]
pub struct DirectEncrypter {
  alg: DirectAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JweEncrypter for DirectEncrypter {
  fn alg(&self) -> JweAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn cek(&self, enc: JweEncryption, _: &JweHeader, _: &mut JweHeader) -> Result<Option<Cow<[u8]>>> {
    let key: &[u8] = self.key.to_raw_bytes()?;

    if key.len() != enc.key_len() {
      return Err(Error::EncError("Invalid Content Encryption Key Length"));
    }

    Ok(Some(Cow::Borrowed(key)))
  }

  fn encrypt(&self, _: &[u8], _: &JweHeader, _: &mut JweHeader) -> Result<Option<Vec<u8>>> {
    Ok(None)
  }
}

impl Deref for DirectEncrypter {
  type Target = dyn JweEncrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}

// =============================================================================
// Direct Decrypter
// =============================================================================

#[derive(Debug)]
pub struct DirectDecrypter {
  alg: DirectAlgorithm,
  key: PKey<Secret>,
  kid: Option<String>,
}

impl JweDecrypter for DirectDecrypter {
  fn alg(&self) -> JweAlgorithm {
    self.alg.into()
  }

  fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  fn decrypt(&self, key: Option<&[u8]>, _: JweEncryption, _: &JweHeader) -> Result<Cow<[u8]>> {
    if key.is_some() {
      return Err(Error::EncError("Invalid Content Encryption Key"));
    }

    self.key.to_raw_bytes().map(Cow::Borrowed)
  }
}

impl Deref for DirectDecrypter {
  type Target = dyn JweDecrypter;

  fn deref(&self) -> &Self::Target {
    self
  }
}
