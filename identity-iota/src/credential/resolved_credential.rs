// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_credential::credential::Credential;
use identity_did::did::DID;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use crate::did::IotaDIDUrl;
use crate::document::ResolvedIotaDocument;
use crate::tangle::TangleRef;

use delegate::delegate;

/// A verifiable credential whose associated DID documents have been resolved from the Tangle.
pub struct ResolvedCredential<T> {
  pub credential: Credential<T>,
  pub issuer: ResolvedIotaDocument,
  pub subjects: BTreeMap<String, ResolvedIotaDocument>,
}

impl<T: Serialize> ResolvedCredential<T> {
  /// Verify the signature using the issuer's DID document.
  pub fn verify_signature(&self, options: &VerifierOptions) -> Result<(), CredentialValidationUnitError> {
    self
      .issuer
      .document
      .verify_data(&self.credential, options)
      .map_err(|err| CredentialValidationUnitError::InvalidProof { source: Box::new(err) })
  }

  /// Returns an iterator over the resolved documents that have been deactivated
  pub fn deactivated_subject_documents(&self) -> impl Iterator<Item = &ResolvedIotaDocument> + '_ {
    self
      .subjects
      .iter()
      .map(|(_, doc)| doc)
      .filter(|resolved_doc| !resolved_doc.document.active())
  }
  delegate! {
      to self.credential {
          /// Checks whether this Credential expires after the given `Timestamp`.
          /// True is returned in the case of no expiration date.
          pub fn expires_after(&self, timestamp: Timestamp) -> bool;

          /// Checks whether the issuance date of this Credential is before the given `Timestamp`.
          pub fn issued_before(&self, timestamp: Timestamp) -> bool;

          /// Checks whether this Credential's types match the input.
          pub fn matches_types(&self, other: &[&str]) -> bool;

          /// Returns an iterator of the `types` of this Credential that are not in `input_types`.
          pub fn types_difference_left<'a>(&'a self, input_types: &'a [&str]) -> impl Iterator<Item = &String> + 'a;

          /// Returns an iterator of `types` that are in `input_types`, but not in this Credential.
          pub fn types_difference_right<'a>(&'a self, input_types: &'a [&str]) -> impl Iterator<Item= &str> + 'a;
      }
  }

  pub fn try_expires_after(&self, timestamp: Timestamp) -> Result<(), CredentialValidationUnitError> {
    self
      .expires_after(timestamp)
      .then(|| ())
      .ok_or(CredentialValidationUnitError::InvalidExpirationDate)
  }

  pub fn try_issued_before(&self, timestamp: Timestamp) -> Result<(), CredentialValidationUnitError> {
    self
      .issued_before(timestamp)
      .then(|| ())
      .ok_or(CredentialValidationUnitError::InvalidIssuanceDate)
  }

  /// Checks that all the contained resolved subject documents are active.
  ///
  /// # Errors   
  /// If the `fail_fast` parameter is set then at most one [`ValidationUnitError`] can be returned in the error case,
  /// otherwise the `OneOrMany::Many` variant is used and there will be an entry for every deactivated subject document.
  pub fn try_only_active_subject_documents(&self, fail_fast: bool) -> Result<(), OneOrMany<CredentialValidationUnitError>> {
    let mut iter = self.deactivated_subject_documents().peekable();

    if iter.peek().is_none() {
      Ok(())
    } else if fail_fast {
      let error: OneOrMany<CredentialValidationUnitError> = iter
        .take(1)
        .map(|deactivated_doc| deactivated_doc.did().to_url())
        .map(|url| CredentialValidationUnitError::DeactivatedSubjectDocument { did_url: url })
        .collect();
      Err(error)
    } else {
      let errors: OneOrMany<CredentialValidationUnitError> = iter
        .map(|deactivated_doc| deactivated_doc.did().to_url())
        .map(|url| CredentialValidationUnitError::DeactivatedSubjectDocument { did_url: url })
        .collect();
      Err(errors)
    }
  }
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CredentialValidationUnitError {
  /// Indicates that the expiration date of the credential is not considered valid.
  #[error("credential validation failed: the expiration date does not satisfy the validation criterea")]
  InvalidExpirationDate,
  /// Indicates that the issuance date of the credential is not considered valid.
  #[error("credential validation failed: the issuance date does not satisfy the validation criterea")]
  InvalidIssuanceDate,
  /// The DID document corresponding to `did` has been deactivated.
  #[error("credential validation failed: encountered deactivated subject document")]
  //Todo: Should the did_url be included in the error message?
  DeactivatedSubjectDocument { did_url: IotaDIDUrl },
  /// The proof verification failed.
  #[error("credential validation failed: could not verify the issuer's signature: {source}")]
  InvalidProof {
    source: Box<dyn std::error::Error>, // Todo: Put an actual error type here
  },
  /// Indicates that the structure of the `Credential` is not spec compliant 
  #[error("credential validation failed: the credentials structure does not comply with the spec: {source}")]
  InvalidStructure {
    source: Box<dyn std::error::Error>, // Todo: Put an actual error type here 
  },
}

#[derive(Debug)]
pub enum CredentialResolutionError {
  /// Caused by a failure to resolve a DID Document.
  DIDResolution {
    source: Box<dyn std::error::Error>, //Todo: specify an actual error type here
  },
  /// Caused by attempting to resolve a [`Credential`] that does not meet one or more specified validation rules.
  Validation {
    validation_errors: OneOrMany<CredentialValidationUnitError>,
  },
}

impl std::fmt::Display for CredentialResolutionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::DIDResolution { source } => {
        write!(
          f,
          "credential resolution failed: could not resolve DID document: {}",
          source
        )
      }
      Self::Validation { validation_errors } => {
        // Todo: Refactor the following imperative code once https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
        // becomes stable.
        let mut combined_validation_errors_string = String::new();
        let separator = ",";
        let separator_len = separator.len();
        for validation_error in validation_errors.iter() {
          let error_string = validation_error.to_string();
          combined_validation_errors_string.reserve(error_string.len() + separator_len);
          combined_validation_errors_string.push_str(&error_string);
          combined_validation_errors_string.push_str(separator);
        }
        write!(
          f,
          "credential resolution failed: The following validation errors were encountered: {}",
          combined_validation_errors_string
        )
      }
    }
  }
}

impl std::error::Error for CredentialResolutionError {} 

impl From<CredentialValidationUnitError> for CredentialResolutionError {
  fn from(error: CredentialValidationUnitError) -> Self {
    Self::Validation {
      validation_errors: OneOrMany::One(error),
    }
  }
}

impl From<OneOrMany<CredentialValidationUnitError>> for CredentialResolutionError {
  fn from(validation_errors: OneOrMany<CredentialValidationUnitError>) -> Self {
    Self::Validation { validation_errors }
  }
}

// Todo: Create tests for verify_signature and deactivated_subject_documents
