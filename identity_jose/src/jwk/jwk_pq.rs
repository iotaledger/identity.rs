// =============================================================================
// Post-quantum algorithm key parameters
// =============================================================================

use zeroize::Zeroize;

use super::{JwkParams, JwkType};

// TODO: PQ - parameter for PQ keys

/// Parameters for Post-Quantum algorithm keys
///
/// [More Info](https://datatracker.ietf.org/doc/html/draft-ietf-cose-dilithium)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize, Zeroize)]
#[zeroize(drop)]
pub struct JwkParamsPQ {

  /// The public key as a base64url-encoded value.
  #[serde(rename = "pub")]
  pub public: String, // Public Key
  /// The private key as a base64url-encoded value.
  #[serde(skip_serializing_if = "Option::is_none", rename = "priv")]
  pub private: Option<String>, // Private Key
}

impl JwkParamsPQ {
  /// Creates new JWK OKP Params.
  pub const fn new() -> Self {
    Self {
      public: String::new(),
      private: None,
    }
  }

  /// Returns a clone with _all_ private key components unset.
  pub fn to_public(&self) -> Self {
    Self {
      public: self.public.clone(),
      private: None,
    }
  }

  /// Returns `true` if _all_ private key components of the key are unset, `false` otherwise.
  pub fn is_public(&self) -> bool {
    self.private.is_none()
  }

  /// Returns `true` if _all_ private key components of the key are set, `false` otherwise.
  pub fn is_private(&self) -> bool {
    self.private.is_some()
  }

}
