use crate::error::Result;
use crate::jws::JwsAlgorithm;
use crate::lib::*;

/// The `JwsSigner` trait specifies a common interface for JWS signature
/// creation algorithms.
pub trait JwsSigner {
  /// Returns the JWS signature algorithm.
  fn alg(&self) -> JwsAlgorithm;

  /// Returns the signer key ID.
  fn kid(&self) -> Option<&str>;

  /// Returns a cryptographic signature of the `message`.
  fn sign(&self, message: &[u8]) -> Result<Vec<u8>>;
}

impl<'a, T> JwsSigner for &'a T
where
  T: JwsSigner,
{
  fn alg(&self) -> JwsAlgorithm {
    (**self).alg()
  }

  fn kid(&self) -> Option<&str> {
    (**self).kid()
  }

  fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
    (**self).sign(message)
  }
}

/// The `JwsSigner` trait specifies a common interface for JWS signature
/// verification algorithms.
pub trait JwsVerifier {
  /// Returns the JWS signature algorithm.
  fn alg(&self) -> JwsAlgorithm;

  /// Returns the verifier key ID.
  fn kid(&self) -> Option<&str>;

  /// Verifies a cryptographic signature of the `message`.
  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()>;
}

impl<'a, T> JwsVerifier for &'a T
where
  T: JwsVerifier,
{
  fn alg(&self) -> JwsAlgorithm {
    (**self).alg()
  }

  fn kid(&self) -> Option<&str> {
    (**self).kid()
  }

  fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
    (**self).verify(message, signature)
  }
}
