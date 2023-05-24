use identity_core::common::Timestamp;
use identity_document::verifiable::JwsVerificationOptions;

use crate::validator::vc_jwt_validation::CredentialValidationOptions;

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

fn bool_true() -> bool {
  true
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

  //todo expiry date
}

/// Declares how credential subjects must relate to the presentation holder during validation.
/// See [`PresentationValidationOptions::subject_holder_relationship()`].
///
/// See also the [Subject-Holder Relationship](https://www.w3.org/TR/vc-data-model/#subject-holder-relationships) section of the specification.
// Need to use serde_repr to make this work with duck typed interfaces in the Wasm bindings.
#[derive(Debug, Clone, Copy, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum SubjectHolderRelationship {
  /// The holder must always match the subject on all credentials, regardless of their [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property.
  /// This is the variant returned by [Self::default](Self::default()) and the default used in
  /// [`PresentationValidationOptions`].
  AlwaysSubject = 0,
  /// The holder must match the subject only for credentials where the [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property is `true`.
  SubjectOnNonTransferable = 1,
  /// Declares that the subject is not required to have any kind of relationship to the holder.
  Any = 2,
}

impl Default for SubjectHolderRelationship {
  fn default() -> Self {
    Self::AlwaysSubject
  }
}
