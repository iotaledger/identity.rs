use crate::validator::JptCredentialValidationOptions;
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

  /// Options to declare validation criteria for [`Credential`](crate::credential::Credential)s.
  #[serde(default)]
  pub credential_validation_options: JptCredentialValidationOptions,
}

impl JptPresentationValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self::default()
  }

  /// Set options which affect the verification of the signature on the presentation.
  pub fn credential_validation_options(mut self, options: JptCredentialValidationOptions) -> Self {
    self.credential_validation_options = options;
    self
  }

  /// Declare that the presentation is **not** considered valid if it expires before this [`Timestamp`].
  /// Uses the current datetime during validation if not set.
  pub fn nonce(mut self, nonce: impl Into<String>) -> Self {
    self.nonce = Some(nonce.into());
    self
  }
}
