// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Url;

/// Options for creating a JSON Web Signature.
#[non_exhaustive]
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JwsSignatureOptions {
  /// Whether to attach the public key in the corresponding method
  /// to the JWS header.
  pub attach_jwk: bool,

  /// Whether to Base64url encode the payload or not.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7797#section-3)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub b64: Option<bool>,

  /// The Type value to be placed in the protected header.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.9)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub typ: Option<String>,

  /// Content Type to be placed in the protected header.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.10)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cty: Option<String>,

  /// The URL to be placed in the protected header.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8555#section-6.4.1)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<Url>,

  /// The nonce to be placed in the protected header.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8555#section-6.5.2)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<String>,

  /// The kid to set in the protected header.
  ///
  /// If unset, the kid of the JWK with which the JWS is produced is used.
  ///
  /// [More Info](https://www.rfc-editor.org/rfc/rfc7515#section-4.1.4)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub kid: Option<String>,

  /// Whether the payload should be detached from the JWS.
  ///
  /// [More Info](https://www.rfc-editor.org/rfc/rfc7515#appendix-F).
  pub detached_payload: bool,

  /// Additional header parameters.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub custom_header_parameters: Option<Object>,
}

impl JwsSignatureOptions {
  /// Creates a new [`JwsSignatureOptions`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Replace the value of the `attach_jwk` field.
  pub fn attach_jwk_to_header(mut self, value: bool) -> Self {
    self.attach_jwk = value;
    self
  }

  /// Replace the value of the `b64` field.
  pub fn b64(mut self, value: bool) -> Self {
    self.b64 = Some(value);
    self
  }

  /// Replace the value of the `typ` field.
  pub fn typ(mut self, value: impl Into<String>) -> Self {
    self.typ = Some(value.into());
    self
  }

  /// Replace the value of the `cty` field.
  pub fn cty(mut self, value: impl Into<String>) -> Self {
    self.cty = Some(value.into());
    self
  }

  /// Replace the value of the `url` field.
  pub fn url(mut self, value: Url) -> Self {
    self.url = Some(value);
    self
  }

  /// Replace the value of the `nonce` field.
  pub fn nonce(mut self, value: impl Into<String>) -> Self {
    self.nonce = Some(value.into());
    self
  }

  /// Replace the value of the `kid` field.
  pub fn kid(mut self, value: impl Into<String>) -> Self {
    self.kid = Some(value.into());
    self
  }

  /// Replace the value of the `detached_payload` field.
  pub fn detached_payload(mut self, value: bool) -> Self {
    self.detached_payload = value;
    self
  }

  /// Adds additional header parameters.
  pub fn custom_header_parameters(mut self, value: Object) -> Self {
    self.custom_header_parameters = Some(value);
    self
  }
}
