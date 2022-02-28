// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_did::verifiable::VerifierOptions;
use serde::Deserialize;
use serde::Serialize;

/// Options to declare validation criteria in validation methods such as
/// [`CredentialValidator::full_validation`](super::CredentialValidator::full_validation()) and
/// [`Resolver::verify_credential`](crate::tangle::Resolver::verify_credential()).
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CredentialValidationOptions {
  /// Declares that the credential is **not** considered valid if it expires before this
  /// [`Timestamp`].
  pub earliest_expiry_date: Option<Timestamp>,
  /// Declares that the credential is **not** considered valid if it was issued later than this
  /// [`Timestamp`].
  pub latest_issuance_date: Option<Timestamp>,
  /// Declare that the credential's signature must be verified according to these
  /// [VerifierOptions].
  #[serde(default)]
  pub verifier_options: VerifierOptions,
}

impl CredentialValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self::default()
  }

  /// Declare that a credential may expire no later than the given `timestamp`.
  pub fn earliest_expiry_date(mut self, timestamp: Timestamp) -> Self {
    self.earliest_expiry_date = Some(timestamp);
    self
  }
  /// Declare that a credential may expire no later than the given `timestamp`.
  pub fn latest_issuance_date(mut self, timestamp: Timestamp) -> Self {
    self.latest_issuance_date = Some(timestamp);
    self
  }

  /// Declare that the signature of a credential is to be verified according to the given
  /// `options`.
  pub fn verifier_options(mut self, options: VerifierOptions) -> Self {
    self.verifier_options = options;
    self
  }
}

/// Declares how a credential subject must relate to the presentation holder.
///
/// See [`PresentationValidationOptions::subject_holder_relationship`](PresentationValidationOptions::
/// subject_holder_relationship()).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SubjectHolderRelationship {
  /// Declare that the holder must always match the subject.
  AlwaysSubject,
  /// Declare that the holder must match the subject on credentials with the nonTransferable property set.
  SubjectOnNonTransferable,
  /// Declares that the subject is not required to have any kind of relationship to the holder.  
  Any,
}

impl Default for SubjectHolderRelationship {
  fn default() -> Self {
    // TODO: Should the default be the most strict variant?
    Self::SubjectOnNonTransferable
  }
}

/// Declares when validation should return with an error.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FailFast {
  /// Declare that validation should fail after all errors have been found
  No,
  /// Declare that validation must fail upon the first error is found
  Yes,
}

/// Options to declare validation criteria for validation methods such as
/// [`PresentationValidator::full_validation`](super::PresentationValidator::full_validation()) and
/// [`Resolver::verify_presentation`](crate::tangle::Resolver::verify_presentation()).
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PresentationValidationOptions {
  /// Declares that the credentials of the presentation must all be
  /// validated according to these [`CredentialValidationOptions`].
  #[serde(default)]
  pub shared_validation_options: CredentialValidationOptions,
  /// Declares that the presentation's signature is to be verified according to these
  /// [`VerifierOptions`].
  #[serde(default)]
  pub presentation_verifier_options: VerifierOptions,
  /// Declares how the presentation's credential subjects must relate to the holder.
  #[serde(default)]
  pub subject_holder_relationship: SubjectHolderRelationship,
}

impl PresentationValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self::default()
  }
  /// Declare that all the presentation's credentials are all to be validated according to the
  /// given `options`.
  pub fn shared_validation_options(mut self, options: CredentialValidationOptions) -> Self {
    self.shared_validation_options = options;
    self
  }
  /// Declare that the presentation's signature is to be verified according to the given
  /// `options`.
  pub fn presentation_verifier_options(mut self, options: VerifierOptions) -> Self {
    self.presentation_verifier_options = options;
    self
  }

  /// Declare how the presentation's credential subjects must relate to the holder.
  pub fn subject_holder_relationship(mut self, options: SubjectHolderRelationship) -> Self {
    self.subject_holder_relationship = options;
    self
  }
}
