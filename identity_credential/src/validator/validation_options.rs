// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_document::verifiable::VerifierOptions;
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
  /// Default: [`StatusCheck::Strict`].
  #[serde(default)]
  pub status: StatusCheck,

  /// Options which affect the verification of the signature on the credential.
  #[serde(default)]
  pub verifier_options: VerifierOptions,
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
  pub fn status_check(mut self, status_check: StatusCheck) -> Self {
    self.status = status_check;
    self
  }

  /// Set options which affect the verification of the signature on the credential.
  pub fn verifier_options(mut self, options: VerifierOptions) -> Self {
    self.verifier_options = options;
    self
  }
}

/// Controls validation behaviour when checking whether or not a credential has been revoked by its
/// [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum StatusCheck {
  /// Validate the status if supported, reject any unsupported
  /// [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) types.
  ///
  /// Only `RevocationBitmap2022` is currently supported.
  ///
  /// This is the default.
  Strict = 0,
  /// Validate the status if supported, skip any unsupported
  /// [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) types.
  SkipUnsupported = 1,
  /// Skip all status checks.
  SkipAll = 2,
}

impl Default for StatusCheck {
  fn default() -> Self {
    Self::Strict
  }
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

/// Declares when validation should return if an error occurs.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FailFast {
  /// Return all errors that occur during validation.
  AllErrors,
  /// Return after the first error occurs.
  FirstError,
}
