//TODO: hybrid - composite public key

use identity_jose::jwk::Jwk;

/// Mame of algorithms used to generate the hybrid signature. Values taken from [here](https://datatracker.ietf.org/doc/html/draft-ietf-lamps-pq-composite-sigs-02#name-domain-separators).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum CompositeAlgId {
  /// DER encoded value in hex = 060B6086480186FA6B50080103
  #[serde(rename = "id-MLDSA44-Ed25519-SHA512")]
  IdMldsa44Ed25519Sha512,
  /// DER encoded value in hex = 060B6086480186FA6B5008010A
  #[serde(rename = "id-MLDSA65-Ed25519-SHA512")]
  IdMldsa65Ed25519Sha512,
}

impl CompositeAlgId {
  /// Returns the JWS algorithm as a `str` slice.
  pub const fn name(self) -> &'static str {
    match self {
      Self::IdMldsa44Ed25519Sha512 => "id-MLDSA44-Ed25519-SHA512",
      Self::IdMldsa65Ed25519Sha512 => "id-MLDSA65-Ed25519-SHA512",
    }
  }
}

/// Represent a combination of a traditional public key and a post-quantum public key both in Jwk format.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct CompositePublicKey {
  #[serde(rename = "algId")]
  alg_id: CompositeAlgId,
  #[serde(rename = "traditionalPublicKey")]
  traditional_public_key: Jwk,
  #[serde(rename = "pqPublicKey")]
  pq_public_key: Jwk,
}

impl CompositePublicKey {
  /// Create a new CompositePublicKey structure.
  pub fn new(alg_id: CompositeAlgId, traditional_public_key: Jwk, pq_public_key: Jwk) -> Self {
    Self {
      alg_id,
      traditional_public_key,
      pq_public_key,
    }
  }
  /// Get the `algId` value.
  pub fn alg_id(&self) -> CompositeAlgId {
    self.alg_id
  }
  /// Get the post-quantum public key in Jwk format.
  pub fn pq_public_key(&self) -> &Jwk {
    &self.pq_public_key
  }
  /// Get the traditional public key in Jwk format.
  pub fn traditional_public_key(&self) -> &Jwk {
    &self.traditional_public_key
  }
}
