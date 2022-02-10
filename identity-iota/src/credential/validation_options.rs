// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_did::verifiable::VerifierOptions;

#[derive(Debug)]
pub struct CredentialValidationOptions {
  pub(crate) earliest_expiry_date: Timestamp,
  pub(crate) latest_issuance_date: Timestamp,
  pub(crate) verifier_options: VerifierOptions,
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
  pub fn new() -> Self {
    Self::default()
  }

  pub fn earliest_expiry_date(mut self, timestamp: Timestamp) -> Self {
    self.earliest_expiry_date = timestamp;
    self
  }

  pub fn latest_issuance_date(mut self, timestamp: Timestamp) -> Self {
    self.latest_issuance_date = timestamp;
    self
  }

  pub fn verifier_options(mut self, verifier_options: VerifierOptions) -> Self {
    self.verifier_options = verifier_options;
    self
  }

  pub fn get_verifier_options_mut(&mut self) -> &mut VerifierOptions {
    &mut self.verifier_options
  }

  //Todo: Should there also be an into_verifier_options method?
}

#[derive(Debug, Default)]
pub struct PresentationValidationOptions {
  pub(crate) common_validation_options: CredentialValidationOptions, // used when validating the credentials
  pub(crate) presentation_verifier_options: VerifierOptions,         /* used when verifying the holder's signature.
                                                                      * Todo: Find a better name. */
}

impl PresentationValidationOptions {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_common_validation_options(mut self, options: CredentialValidationOptions) -> Self {
    self.common_validation_options = options;
    self
  }

  pub fn with_presentation_verifier_options(mut self, options: VerifierOptions) -> Self {
    self.presentation_verifier_options = options;
    self
  }
  // Returns an option as we may want this field to be optional in the future.
  pub fn common_validation_options_mut(&mut self) -> Option<&mut CredentialValidationOptions> {
    Some(&mut self.common_validation_options)
  }
}
