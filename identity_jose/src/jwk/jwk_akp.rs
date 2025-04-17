// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0


// =============================================================================
// Algorithm Key Pair (AKP) key parameters for Post-quantum algorithm
// =============================================================================

use zeroize::Zeroize;

use super::JwkType;

/// Parameters for Post-Quantum algorithm keys
///
/// [More Info](https://datatracker.ietf.org/doc/html/draft-ietf-cose-dilithium-06)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize, Zeroize)]
#[zeroize(drop)]
pub struct JwkParamsAKP {
  /// The public key as a base64url-encoded value.
  #[serde(rename = "pub")]
  pub public: String, // Public Key
  /// The private key as a base64url-encoded value.
  #[serde(skip_serializing_if = "Option::is_none", rename = "priv")]
  pub private: Option<String>, // Private Key
}

impl JwkParamsAKP {
  /// Creates new JWK AKP Params.
  pub const fn new() -> Self {
    Self {
      public: String::new(),
      private: None,
    }
  }

  /// Returns the key type `kty`.
  pub const fn kty(&self) -> JwkType {
    JwkType::Akp
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
