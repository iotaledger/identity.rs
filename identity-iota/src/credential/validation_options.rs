// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_did::verifiable::VerifierOptions;

pub struct CredentialValidationOptions {
  pub(crate) expires_after: Timestamp,
  pub(crate) issued_before: Timestamp,
  pub(crate) verifier_options: VerifierOptions,
}

impl Default for CredentialValidationOptions {
  fn default() -> Self {
    Self {
      expires_after: Timestamp::now_utc(),
      issued_before: Timestamp::now_utc(),
      verifier_options: VerifierOptions::default(),
    }
  }
}

impl CredentialValidationOptions {
  pub fn expires_after(mut self, timestamp: Timestamp) -> Self {
    self.expires_after = timestamp;
    self
  }

  pub fn issued_before(mut self, timestamp: Timestamp) -> Self {
    self.issued_before = timestamp;
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

pub struct PresentationValidationOptions {
  pub(crate) common_validation_options: CredentialValidationOptions, // used when validating the credentials
  pub(crate) presentation_verifier_options: VerifierOptions,         /* used when verifying the holder's signature.
                                                                      * Todo: Find a better name. */
}

impl Default for PresentationValidationOptions {
  fn default() -> Self {
    Self {
      common_validation_options: CredentialValidationOptions::default(),
      presentation_verifier_options: VerifierOptions::default(),
    }
  }
}

impl PresentationValidationOptions {
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
