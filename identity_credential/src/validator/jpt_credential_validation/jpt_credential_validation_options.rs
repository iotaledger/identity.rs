
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_document::verifiable::JwpVerificationOptions;
use identity_document::verifiable::JwsVerificationOptions;
use serde::Deserialize;
use serde::Serialize;

use crate::validator::SubjectHolderRelationship;

/// Options to declare validation criteria for [`Credential`](crate::credential::Credential)s.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JptCredentialValidationOptions {
  /// Declares that the credential is **not** considered valid if it expires before this
  /// [`Timestamp`].
  /// Uses the current datetime during validation if not set.
  #[serde(default)]
  pub earliest_expiry_date: Option<Timestamp>,

  /// Declares that the credential is **not** considered valid if it was issued later than this
  /// [`Timestamp`].
  /// Uses the current datetime during validation if not set.
  #[serde(default)]
  pub latest_issuance_date: Option<Timestamp>,

  /// Options which affect the verification of the proof on the credential.
  #[serde(default)]
  pub verification_options: JwpVerificationOptions,
}

impl JptCredentialValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self::default()
  }

  /// Declare that the credential is **not** considered valid if it expires before this [`Timestamp`].
  /// Uses the current datetime during validation if not set.
  pub fn earliest_expiry_date(mut self, timestamp: Timestamp) -> Self {
    self.earliest_expiry_date = Some(timestamp);
    self
  }

  /// Declare that the credential is **not** considered valid if it was issued later than this [`Timestamp`].
  /// Uses the current datetime during validation if not set.
  pub fn latest_issuance_date(mut self, timestamp: Timestamp) -> Self {
    self.latest_issuance_date = Some(timestamp);
    self
  }

  /// Set options which affect the verification of the JWS signature.
  pub fn verification_options(mut self, options: JwpVerificationOptions) -> Self {
    self.verification_options = options;
    self
  }
}
