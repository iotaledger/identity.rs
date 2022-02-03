// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use identity_core::common::Timestamp;
use identity_credential::credential::Credential;
use identity_did::did::DID;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use crate::document::ResolvedIotaDocument;
use crate::tangle::TangleRef;
use crate::Result;
use delegate::delegate;

/// A verifiable credential whose associated DID documents have been resolved from the Tangle.
pub struct ResolvedCredential<T> {
  pub credential: Credential<T>,
  pub issuer: ResolvedIotaDocument,
  pub subjects: BTreeMap<String, ResolvedIotaDocument>,
}

impl<T: Serialize> ResolvedCredential<T> {
  /// Verify the signature using the issuer's DID document.
  ///
  /// # Terminology
  /// This method is a *validation unit*
  pub fn verify_signature(&self, options: &VerifierOptions) -> Result<()> {
    self
      .issuer
      .document
      .verify_data(&self.credential, options)
      .map_err(|err| super::errors::ValidationError::IssuerProof { source: err.into() })
      .map_err(Into::into)
  }

  /// Returns an iterator over the resolved documents that have been deactivated.
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

  /// Validate that the [ResolvedCredential] expires after the specified [Timestamp].
  ///
  /// # Terminology
  /// This is a *validation unit*
  pub fn try_expires_after(&self, timestamp: Timestamp) -> Result<()> {
    self
      .expires_after(timestamp)
      .then(|| ())
      .ok_or(super::errors::ValidationError::ExpirationDate)
      .map_err(Into::into)
  }

  /// Validate that the [ResolvedCredential] is issued before the specified [Timestamp].
  ///
  /// # Terminology
  /// This is a *validation unit*
  pub fn try_issued_before(&self, timestamp: Timestamp) -> Result<()> {
    self
      .issued_before(timestamp)
      .then(|| ())
      .ok_or(super::errors::ValidationError::IssuanceDate)
      .map_err(Into::into)
  }

  /// Validates that all the contained resolved subject documents are active.
  ///
  /// # Errors
  ///  Returns an error on the first deactivated subject document encountered.
  ///
  /// # Terminology
  /// This is a *validation unit*.
  pub fn try_only_active_subject_documents(&self) -> Result<()> {
    if let Some(deactivated_doc) = self.deactivated_subject_documents().next() {
      Err(
        // Todo: Should this method document that it allocates on failure since it is considered part of the
        // low-level validation API?
        super::errors::ValidationError::DeactivatedSubjectDocument {
          did_url: deactivated_doc.did().to_url(),
        }
        .into(),
      )
    } else {
      Ok(())
    }
  }

  /// Validates the semantic structure of the `Credential`.
  ///
  /// # Terminology
  /// This is a *validation unit*
  pub fn check_structure(&self) -> Result<()> {
    self
      .credential
      .check_structure()
      .map_err(super::errors::ValidationError::CredentialStructure)
      .map_err(Into::into)
  }
}

// Todo: Create tests for verify_signature and deactivated_subject_documents
