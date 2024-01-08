
use identity_document::verifiable::JwpVerificationOptions;
use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Timestamp;
use identity_document::verifiable::JwsVerificationOptions;

/// Criteria for validating a [`Presentation`](crate::presentation::Presentation).
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct JptPresentationValidationOptions {
    /// Options which affect the verification of the signature on the presentation.
    #[serde(default)]
    pub presentation_verifier_options: JwpVerificationOptions,

    /// The nonce to be placed in the Presentation Protected Header.
    #[serde(default)]
    pub nonce: Option<String>,

    /// Declares that the presentation is **not** considered valid if it expires before this
    /// [`Timestamp`].
    /// Uses the current datetime during validation if not set.
    #[serde(default)]
    pub earliest_expiry_date: Option<Timestamp>,

    /// Declares that the presentation is **not** considered valid if it was issued later than this
    /// [`Timestamp`].
    /// Uses the current datetime during validation if not set.
    #[serde(default)]
    pub latest_issuance_date: Option<Timestamp>, 
}

impl JptPresentationValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self::default()
  }

  /// Set options which affect the verification of the signature on the presentation.
  pub fn presentation_verifier_options(mut self, options: JwpVerificationOptions) -> Self {
    self.presentation_verifier_options = options;
    self
  }

  /// Declare that the presentation is **not** considered valid if it expires before this [`Timestamp`].
  /// Uses the current datetime during validation if not set.
  pub fn nonce(mut self, nonce: String) -> Self {
    self.nonce = Some(nonce);
    self
  }


  /// Declare that the presentation is **not** considered valid if it expires before this [`Timestamp`].
  /// Uses the current datetime during validation if not set.
  pub fn earliest_expiry_date(mut self, timestamp: Timestamp) -> Self {
    self.earliest_expiry_date = Some(timestamp);
    self
  }

  /// Declare that the presentation is **not** considered valid if it was issued later than this [`Timestamp`].
  /// Uses the current datetime during validation if not set.
  pub fn latest_issuance_date(mut self, timestamp: Timestamp) -> Self {
    self.latest_issuance_date = Some(timestamp);
    self
  }
}
