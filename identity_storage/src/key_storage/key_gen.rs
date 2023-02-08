use identity_jose::jwk::Jwk;

use super::KeyId;

/// The output of a JWK key generation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JwkGenOutput {
  pub key_id: KeyId,
  pub jwk: Jwk,
}

impl JwkGenOutput {
  pub fn new(key_id: KeyId, jwk: Jwk) -> Self {
    Self { key_id, jwk }
  }
}
