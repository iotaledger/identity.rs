
use identity_core::common::Object;
use identity_core::common::Url;

//TODO: have to choose which options makes sense in the context of jwp

/// Options for creating a JSON Web Signature.
#[non_exhaustive]
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JwpOptions {

  /// The Type value to be placed in the protected header.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.9)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub typ: Option<String>,

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
  pub kid: Option<String>
}

impl JwpOptions {
  /// Creates a new [`JwsSignatureOptions`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Replace the value of the `typ` field.
  pub fn typ(mut self, value: impl Into<String>) -> Self {
    self.typ = Some(value.into());
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
  
}
