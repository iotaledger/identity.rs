// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_did::verifiable::VerifierOptions;

#[derive(Debug)]
/// Options to declare validation criteria in [super::CredentialValidator::full_validation()].
#[non_exhaustive]
pub struct CredentialValidationOptions {
  /// Declares that the [identity_credential::Credential] is **not** considered valid if it expires before this [Timestamp]. 
  pub earliest_expiry_date: Timestamp,
  /// Declares that the [identity_credential::Credential] is **not** considered valid if it was issued later than this [Timestamp].
  pub latest_issuance_date: Timestamp,
  /// Declare that the [identity_credential::Credential]'s signature must be verified according to these [VerifierOptions].
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

  /// Declare that a [identity_credential::Credential] may expire no later than the given `timestamp`.
  pub fn earliest_expiry_date(mut self, timestamp: Timestamp) -> Self {
    self.earliest_expiry_date = timestamp;
    self
  }
  /// Declare that a [identity_credential::Credential] may expire no later than the given `timestamp`.
  pub fn latest_issuance_date(mut self, timestamp: Timestamp) -> Self {
    self.latest_issuance_date = timestamp;
    self
  }

  /// Declare that the signature of a [identity_credential::Credential] is to be verified according to the given
  /// `options`.
  pub fn verifier_options(mut self, options: VerifierOptions) -> Self {
    self.verifier_options = options;
    self
  }
}

#[derive(Debug)]
#[non_exhaustive]
/// Options to declare validation criteria for [super::PresentationValidator::full_validation()].
pub struct PresentationValidationOptions {
  /// Declares that the [identity_credential::Credential] of the [identity_credential::Presentation] must all be validated according to these options. 
  pub shared_validation_options: CredentialValidationOptions, 
  /// Declares that the [identity_credential::Presentation]'s signature is to be verified according to these [VerifierOptions].
  pub presentation_verifier_options: VerifierOptions,       
}

impl PresentationValidationOptions {
  /// Constructor that sets all options to their defaults.
  pub fn new() -> Self {
    Self {
      shared_validation_options: CredentialValidationOptions::default(),
      presentation_verifier_options: VerifierOptions::default(),
    }
  }
  /// Declare that all the [identity_credential::Presentation]'s credentials are all to be validated according to the
  /// given `options`.
  pub fn shared_validation_options(mut self, options: CredentialValidationOptions) -> Self {
    self.shared_validation_options = options;
    self
  }
  /// Declare that the [identity_credential::Presentation]'s signature is to be verified according to the given
  /// `options`.
  pub fn presentation_verifier_options(mut self, options: VerifierOptions) -> Self {
    self.presentation_verifier_options = options;
    self
  }
}

impl Default for PresentationValidationOptions {
  fn default() -> Self {
    Self::new()
  }
}
