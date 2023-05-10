// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JwsSignatureOptions {
  /// Whether to attach the public key in the corresponding method
  /// to the JWS header.
  pub attach_jwk: bool,

  #[serde(skip_serializing_if = "Option::is_none")]
  /// Whether to Base64url encode the payload or not.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7797#section-3)
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

  /// A list of permitted extension parameters to be attached to the protected header.
  ///
  ///[More Info](https://tools.ietf.org/html/rfc7515#section-4.1.11)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub crit: Option<Vec<String>>,

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
  /// Whether the payload should be detached from the JWS.
  ///
  /// [More Info](https://www.rfc-editor.org/rfc/rfc7515#appendix-F).
  pub detached_payload: bool,
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
  ///
  /// Setting this to `false` will also add `"b64"` to the `crit` parameters, while
  /// setting `true` will omit the parameter from the header and the string from the `crit` parameters,
  /// as recommended in <https://datatracker.ietf.org/doc/html/rfc7797#section-7>.
  pub fn b64(mut self, value: bool) -> Self {
    self.b64 = Some(value);
    if !value {
      self.add_crit("b64".to_owned())
    } else {
      self
    }
  }

  /// Replace the value of the `typ` field.
  pub fn typ(mut self, value: String) -> Self {
    self.typ = Some(value);
    self
  }

  /// Replace the value of the `cty` field.
  pub fn cty(mut self, value: String) -> Self {
    self.cty = Some(value);
    self
  }

  /// Append a value to the list of permitted extensions.
  pub fn add_crit(mut self, value: impl Into<String>) -> Self {
    let mut crits = self.crit.unwrap_or_default();
    crits.push(value.into());
    self.crit = Some(crits);
    self
  }

  /// Replace the value of the `url` field.
  pub fn url(mut self, value: Url) -> Self {
    self.url = Some(value);
    self
  }

  /// Replace the value of the `nonce` field.
  pub fn nonce(mut self, value: String) -> Self {
    self.nonce = Some(value);
    self
  }

  /// Replace the value of the `detached_payload` field.
  pub fn detached_payload(mut self, value: bool) -> Self {
    self.detached_payload = value;
    self
  }
}
