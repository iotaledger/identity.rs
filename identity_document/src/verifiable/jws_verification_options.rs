// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_verification::MethodScope;

/// Holds additional options for verifying a JWS with
/// [`CoreDocument::verify_jws`](crate::document::CoreDocument::verify_jws()).
#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JwsVerificationOptions {
  pub crits: Option<Vec<String>>,
  pub nonce: Option<String>,
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
