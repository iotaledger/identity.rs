use alloc::vec::Vec;

use crate::error::Result;
use crate::jws::JwsAlgorithm;

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
