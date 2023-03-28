// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_verification::MethodScope;

/// Holds additional options for verifying a JWS with
/// [`CoreDocument::verify_jws`](crate::document::CoreDocument::verify_jws()).
#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JwsVerificationOptions {
  /// A list of permitted extension parameters.
  ///
  /// [More Info](https://www.rfc-editor.org/rfc/rfc7515#section-4.1.11)
  pub crits: Option<Vec<String>>,
  /// Verify that the nonce set in the protected header matches this value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8555#section-6.5.2)
  pub nonce: Option<String>,
  /// Verify the signing verification method relation matches this.
  pub method_scope: Option<MethodScope>,
}

impl JwsVerificationOptions {
  /// Append values to the list of permitted extension parameters.
  pub fn critical(mut self, value: impl Into<String>) -> Self {
    self.crits.get_or_insert(Vec::new()).push(value.into());
    self
  }

  /// Set the expected value for the `nonce` parameter of the protected header.
  pub fn nonce(mut self, value: impl Into<String>) -> Self {
    self.nonce = Some(value.into());
    self
  }

  /// Set the scope of the verification methods that may be used to verify the given JWS.
  pub fn method_scope(mut self, value: MethodScope) -> Self {
    self.method_scope = Some(value);
    self
  }
}
