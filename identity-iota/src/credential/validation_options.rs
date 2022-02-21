// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_did::verifiable::VerifierOptions;

#[derive(Debug)]
/// Options to declare validation criteria in
/// [CredentialValidator::full_validation](super::CredentialValidator::full_validation()).
#[non_exhaustive]
pub struct CredentialValidationOptions {
  /// Declares that the credential is **not** considered valid if it expires before this
  /// [Timestamp].
  pub earliest_expiry_date: Timestamp,
  /// Declares that the credential is **not** considered valid if it was issued later than this
  /// [Timestamp].
  pub latest_issuance_date: Timestamp,
  /// Declare that the credential's signature must be verified according to these
  /// [VerifierOptions].
  pub verifier_options: VerifierOptions,
}

impl Default for CredentialValidationOptions {
  fn default() -> Self {
    Self {
      earliest_expiry_date: Timestamp::now_utc(),
      latest_issuance_date: Timestamp::now_utc(),
      verifier_options: VerifierOptions::default(),
    }
  }
}

impl CredentialValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self::default()
  }

  /// Declare that a credential may expire no later than the given `timestamp`.
  pub fn earliest_expiry_date(mut self, timestamp: Timestamp) -> Self {
    self.earliest_expiry_date = timestamp;
    self
  }
  /// Declare that a credential may expire no later than the given `timestamp`.
  pub fn latest_issuance_date(mut self, timestamp: Timestamp) -> Self {
    self.latest_issuance_date = timestamp;
    self
  }

  /// Declare that the signature of a credential is to be verified according to the given
  /// `options`.
  pub fn verifier_options(mut self, options: VerifierOptions) -> Self {
    self.verifier_options = options;
    self
  }
}

#[derive(Debug)]
#[non_exhaustive]
/// Options to declare validation criteria for
/// [PresentationValidator::full_validation](super::PresentationValidator::full_validation()).
pub struct PresentationValidationOptions {
  /// Declares that the credentials of the presentation must all be
  /// validated according to these options.
  pub shared_validation_options: CredentialValidationOptions,
  /// Declares that the presentation's signature is to be verified according to these
  /// [VerifierOptions].
  pub presentation_verifier_options: VerifierOptions,
  pub(super) allow_non_transferable_violations: bool, // private as we may change the representation
  pub(super) holder_must_be_subject: bool,            /* private as we may change the representation
                                                       * note that holder_must_be_subject = true +
                                                       * allow_non_transferable_violations = true can lead to
                                                       * confusion
                                                       * it would be better to introduce an enum
                                                       * HolderSubjcetRelationShip {AlwaysSubject,
                                                       * SubjectOnNonTransferable, DoNotValidate}
                                                       * but it is not clear how we can expose that to javascript. */
}

impl PresentationValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self {
      shared_validation_options: CredentialValidationOptions::default(),
      presentation_verifier_options: VerifierOptions::default(),
      allow_non_transferable_violations: false,
      holder_must_be_subject: false, /* Todo: should the default be true (more restrictive, but our own invention
                                      * (not defined in the spec)) */
    }
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

  /// Declare whether a presentation may be considered valid despite there being credentials with the nonTransferable
  /// property set containing a subject different from the holder.  
  pub fn allow_non_transferable_violations(mut self, value: bool) -> Self {
    self.allow_non_transferable_violations = value;
    self
  }
  /// Declare whether credential subjects may be different from the holder.
  pub fn holder_must_be_subject(mut self, value: bool) -> Self {
    self.holder_must_be_subject = value;
    self
  }
}

impl Default for PresentationValidationOptions {
  fn default() -> Self {
    Self::new()
  }
}
