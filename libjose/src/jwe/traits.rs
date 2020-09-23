use crate::error::Result;
use crate::alloc::Vec;
use crate::jwe::JweAlgorithm;

/// The `JweEncrypter` trait specifies a common interface for JWE encryption
/// algorithms.
pub trait JweEncrypter {
  /// Returns the JWE encryption algorithm.
  fn alg(&self) -> JweAlgorithm;

  /// Returns the encrypter key ID.
  fn kid(&self) -> Option<&str>;

  /// Returns an encrypted version of `message`.
  fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>>;
}

/// The `JweDecrypter` trait specifies a common interface for JWE decryption
/// algorithms.
pub trait JweDecrypter {
  /// Returns the JWE encryption algorithm.
  fn alg(&self) -> JweAlgorithm;

  /// Returns the decrypter key ID.
  fn kid(&self) -> Option<&str>;

  /// Returns a decrypted version of `message`.
  fn decrypt(&self, message: &[u8]) -> Result<Vec<u8>>;
}
