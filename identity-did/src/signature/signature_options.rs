// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::verification::MethodWrap;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SignatureOptions {
  #[serde(rename = "verificationMethod")]
  pub verification_method: String,
  #[serde(rename = "proofPurpose", skip_serializing_if = "Option::is_none")]
  pub proof_purpose: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain: Option<String>,
}

impl SignatureOptions {
  pub const fn new(verification_method: String) -> Self {
    Self {
      verification_method,
      proof_purpose: None,
      created: None,
      nonce: None,
      domain: None,
    }
  }

  pub const fn with_purpose(verification_method: String, proof_purpose: String) -> Self {
    Self {
      verification_method,
      proof_purpose: Some(proof_purpose),
      created: None,
      nonce: None,
      domain: None,
    }
  }
}

impl<T> From<MethodWrap<'_, T>> for SignatureOptions {
  fn from(other: MethodWrap<'_, T>) -> Self {
    Self::with_purpose(other.id().to_string(), other.scope().as_str().to_string())
  }
}
