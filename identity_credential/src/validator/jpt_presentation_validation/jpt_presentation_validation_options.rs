use identity_document::verifiable::JwpVerificationOptions;
use serde::Deserialize;
use serde::Serialize;

/// Criteria for validating a [`Presentation`](crate::presentation::Presentation).
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct JptPresentationValidationOptions {

    /// The nonce to be placed in the Presentation Protected Header.
    #[serde(default)]
    pub nonce: Option<String>,

    /// Validation behaviour for [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status).
    ///
    /// Default: [`StatusCheck::Strict`](crate::validator::StatusCheck::Strict).
    #[serde(default)]
    pub status: crate::validator::StatusCheck,

    /// Options which affect the verification of the proof on the credential.
    #[serde(default)]
    pub verification_options: JwpVerificationOptions,

}

impl JptPresentationValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self::default()
  }

  /// Declare that the presentation is **not** considered valid if it expires before this [`Timestamp`].
  /// Uses the current datetime during validation if not set.
  pub fn nonce(mut self, nonce: impl Into<String>) -> Self {
    self.nonce = Some(nonce.into());
    self
  }


  /// Sets the validation behaviour for [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status).
  pub fn status_check(mut self, status_check: crate::validator::StatusCheck) -> Self {
    self.status = status_check;
    self
  }

  /// Set options which affect the verification of the JWP proof.
  pub fn verification_options(mut self, options: JwpVerificationOptions) -> Self {
    self.verification_options = options;
    self
  }

}
