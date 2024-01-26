use identity_core::common::Url;
use serde::{Serialize, Deserialize};

//TODO: ZKP - JwpPresentationOptions

/// Options to be set in the JWT claims of a verifiable presentation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwpPresentationOptions {
  /// Sets the audience for presentation (`aud` property in JWP Presentation Header).
  /// Default: `None`.
  pub audience: Option<Url>,

  /// The nonce to be placed in the Presentation Protected Header.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<String>,
}

impl JwpPresentationOptions {

  /// Sets the audience for presentation (`aud` property in JWT claims).
  pub fn audience(mut self, audience: Url) -> Self {
    self.audience = Some(audience);
    self
  }

  /// Replace the value of the `nonce` field.
  pub fn nonce(mut self, value: impl Into<String>) -> Self {
    self.nonce = Some(value.into());
    self
  }
}

impl Default for JwpPresentationOptions {
  fn default() -> Self {
    Self {
      audience: None,
      nonce: None
    }
  }
}