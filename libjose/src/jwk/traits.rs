use crate::jwk::Jwk;

/// The `JwkKeyPair` trait specifies a common interface for JWK key management.
pub trait JwkKeyPair {
  /// Returns the serialize public key in PEM format.
  fn to_public_pem(&self) -> Vec<u8>;

  /// Returns the serialize secret key in PEM format.
  fn to_secret_pem(&self) -> Vec<u8>;

  /// Returns the serialize public key in DER format.
  fn to_public_der(&self) -> Vec<u8>;

  /// Returns the serialize secret key in DER format.
  fn to_secret_der(&self) -> Vec<u8>;

  /// Create a `Jwk` with public access.
  fn to_public_jwk(&self) -> Jwk;

  /// Create a `Jwk` with private access.
  fn to_secret_jwk(&self) -> Jwk;

  /// Create a `Jwk` with public/private access.
  fn to_combined_jwk(&self) -> Jwk;
}
