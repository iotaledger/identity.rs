// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_did::verifiable::VerifierOptions;

#[derive(Debug)]
pub struct CredentialValidationOptions {
  pub(super) earliest_expiry_date: Timestamp,
  pub(super) latest_issuance_date: Timestamp,
  pub(super) verifier_options: VerifierOptions,
  pub(super) fail_fast: bool,
}

impl Default for CredentialValidationOptions {
  fn default() -> Self {
    Self {
      earliest_expiry_date: Timestamp::now_utc(),
      latest_issuance_date: Timestamp::now_utc(),
      verifier_options: VerifierOptions::default(),
      fail_fast: true,
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

  pub fn fail_fast(mut self, value: bool) -> Self {
    self.fail_fast = value;
    self
  }

  pub fn get_verifier_options_mut(&mut self) -> &mut VerifierOptions {
    &mut self.verifier_options
  }

  //Todo: Should there also be an into_verifier_options method?
}

#[derive(Debug)]
pub struct PresentationValidationOptions {
  pub(super) shared_validation_options: CredentialValidationOptions, // used when validating the credentials
  pub(super) presentation_verifier_options: VerifierOptions,         /* used when verifying the holder's signature. */
  pub(super) fail_fast: bool,
}

impl PresentationValidationOptions {
  pub fn new() -> Self {
    Self {
      shared_validation_options: CredentialValidationOptions::default(),
      presentation_verifier_options: VerifierOptions::default(),
      fail_fast: true,
    }
  }

  pub fn shared_validation_options(mut self, options: CredentialValidationOptions) -> Self {
    self.shared_validation_options = options;
    self
  }

  pub fn presentation_verifier_options(mut self, options: VerifierOptions) -> Self {
    self.presentation_verifier_options = options;
    self
  }

  pub fn fail_fast(mut self, value: bool) -> Self {
    self.fail_fast = value;
    self
  }
}

impl Default for PresentationValidationOptions {
  fn default() -> Self {
    Self::new()
  }
}
