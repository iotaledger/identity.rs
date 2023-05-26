use identity_core::common::Timestamp;
use identity_document::verifiable::JwsVerificationOptions;

use crate::validator::{vc_jwt_validation::CredentialValidationOptions, SubjectHolderRelationship};

/// Criteria for validating a [`Presentation`](crate::presentation::Presentation), such as with
/// [`PresentationValidator::validate`](crate::validator::PresentationValidator::validate()).
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct JwtPresentationValidationOptions {
  /// Options which affect the validation of *all* credentials in the presentation.
  #[serde(default)]
  pub shared_validation_options: CredentialValidationOptions,
  /// Options which affect the verification of the signature on the presentation.
  #[serde(default)]
  pub presentation_verifier_options: JwsVerificationOptions,
  /// Declares how the presentation's credential subjects must relate to the holder.
  /// Default: [`SubjectHolderRelationship::AlwaysSubject`].
  #[serde(default)]
  pub subject_holder_relationship: SubjectHolderRelationship,

  // /// Determines if the JWT expiration date claim `exp` should be skipped during validation.
  // /// Default: false.
  // #[serde(default)]
  // pub skip_exp: bool,
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
}

impl JwtPresentationValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self::default()
  }

  /// Set options which affect the validation of *all* credentials in the presentation.
  pub fn shared_validation_options(mut self, options: CredentialValidationOptions) -> Self {
    self.shared_validation_options = options;
    self
  }
  /// Set options which affect the verification of the signature on the presentation.
  pub fn presentation_verifier_options(mut self, options: JwsVerificationOptions) -> Self {
    self.presentation_verifier_options = options;
    self
  }

  /// Declares how the presentation's holder must relate to the credential subjects.
  pub fn subject_holder_relationship(mut self, options: SubjectHolderRelationship) -> Self {
    self.subject_holder_relationship = options;
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
