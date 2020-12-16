use crate::error::Result;
use crate::jwe::JweAlgorithm;
use crate::jwe::JweEncryption;
use crate::jwe::JweHeader;
use crate::lib::*;

/// The `JweEncrypter` trait specifies a common interface for JWE encryption
/// algorithms.
pub trait JweEncrypter {
  /// Returns the JWE encryption algorithm.
  fn alg(&self) -> JweAlgorithm;

  /// Returns the encrypter key ID.
  fn kid(&self) -> Option<&str>;

  /// Returns a computed content encryption key.
  fn cek(
    &self,
    enc: JweEncryption,
    header: &JweHeader,
    output: &mut JweHeader,
  ) -> Result<Option<Cow<[u8]>>>;

  /// Returns an encrypted key.
  fn encrypt(
    &self,
    key: &[u8],
    header: &JweHeader,
    output: &mut JweHeader,
  ) -> Result<Option<Vec<u8>>>;
}

impl<'a, T> JweEncrypter for &'a T
where
  T: JweEncrypter,
{
  fn alg(&self) -> JweAlgorithm {
    (**self).alg()
  }

  fn kid(&self) -> Option<&str> {
    (**self).kid()
  }

  fn cek(
    &self,
    enc: JweEncryption,
    header: &JweHeader,
    output: &mut JweHeader,
  ) -> Result<Option<Cow<[u8]>>> {
    (**self).cek(enc, header, output)
  }

  fn encrypt(
    &self,
    key: &[u8],
    header: &JweHeader,
    output: &mut JweHeader,
  ) -> Result<Option<Vec<u8>>> {
    (**self).encrypt(key, header, output)
  }
}

/// The `JweDecrypter` trait specifies a common interface for JWE decryption
/// algorithms.
pub trait JweDecrypter {
  /// Returns the JWE encryption algorithm.
  fn alg(&self) -> JweAlgorithm;

  /// Returns the decrypter key ID.
  fn kid(&self) -> Option<&str>;

  /// Returns a decrypted key.
  fn decrypt(
    &self,
    key: Option<&[u8]>,
    enc: JweEncryption,
    header: &JweHeader,
  ) -> Result<Cow<[u8]>>;
}

impl<'a, T> JweDecrypter for &'a T
where
  T: JweDecrypter,
{
  fn alg(&self) -> JweAlgorithm {
    (**self).alg()
  }

  fn kid(&self) -> Option<&str> {
    (**self).kid()
  }

  fn decrypt(
    &self,
    key: Option<&[u8]>,
    enc: JweEncryption,
    header: &JweHeader,
  ) -> Result<Cow<[u8]>> {
    (**self).decrypt(key, enc, header)
  }
}
