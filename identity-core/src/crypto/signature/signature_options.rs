// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Customizable properties of a DID Document signature.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SignatureOptions {
  /// The unique identifier of the DID method used to create this signature.
  #[serde(rename = "verificationMethod")]
  pub verification_method: String,
  /// The intended purpose of the signature.
  #[serde(rename = "proofPurpose", skip_serializing_if = "Option::is_none")]
  pub proof_purpose: Option<String>,
  /// A timestamp of when the signature was created.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<String>,
  /// The signature `nonce` property.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<String>,
  /// The signature `domain` property.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain: Option<String>,
}

impl SignatureOptions {
  /// Creates a new [`SignatureOptions`] instance with the given `method`.
  pub fn new(method: impl Into<String>) -> Self {
    Self {
      verification_method: method.into(),
      proof_purpose: None,
      created: None,
      nonce: None,
      domain: None,
    }
  }

  /// Creates a new [`SignatureOptions`] instance with the given `method` and
  /// `purpose`.
  pub fn with_purpose(method: impl Into<String>, purpose: impl Into<String>) -> Self {
    Self {
      verification_method: method.into(),
      proof_purpose: Some(purpose.into()),
      created: None,
      nonce: None,
      domain: None,
    }
  }
}
