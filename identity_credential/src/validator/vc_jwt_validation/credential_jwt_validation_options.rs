// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// TODO: Replace or update the equivalent types in the parent module.
use identity_core::common::Timestamp;
use identity_document::verifiable::JwsVerificationOptions;
use serde::Deserialize;
use serde::Serialize;

/// Options to declare validation criteria for credentials.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialValidationOptions {
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

  /// Validation behaviour for [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status).
  ///
  /// Default: [`StatusCheck::Strict`](crate::validator::StatusCheck::Strict).
  #[serde(default)]
  pub status: crate::validator::StatusCheck,

  /// Options which affect the verification of the signature on the credential.
  #[serde(default)]
  pub verification_options: JwsVerificationOptions,
}

impl CredentialValidationOptions {
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

  /// Sets the validation behaviour for [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status).
  pub fn status_check(mut self, status_check: crate::validator::StatusCheck) -> Self {
    self.status = status_check;
    self
  }

  /// Set options which affect the verification of the JWS signature.
  pub fn verification_options(mut self, options: JwsVerificationOptions) -> Self {
    self.verification_options = options;
    self
  }
}
