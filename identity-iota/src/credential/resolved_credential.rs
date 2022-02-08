// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_credential::credential::Credential;
use identity_did::did::DID;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::CompoundError;
use super::CredentialValidator;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::Error;
use crate::Result;
use delegate::delegate;

/// A verifiable credential whose associated DID documents have been resolved from the Tangle.
///
/// This struct enables low-level control over how a [`Credential`] gets validated by offering the following validation
/// units
/// - [`Self::verify_signature()`]
/// - [`Self::try_issued_before()`]
/// - [`Self::try_only_active_subject_documents`]
/// - [`Self::try_expires_after()`]
/// - [`Self::check_structure()`]
pub struct ResolvedCredential<T> {
  pub(crate) credential: Credential<T>,
  pub(crate) issuer: ResolvedIotaDocument,
  pub(crate) subjects: OneOrMany<ResolvedIotaDocument>,
}

impl<T: Serialize> ResolvedCredential<T> {
  /// Combines a `Credential` with [`ResolvedIotaDocument`]s belonging to the issuer and credential subjects.
  ///
  /// # Errors
  /// Fails if the credential's issuer property has an url that cannot be identified with the DID of the `issuer`
  /// argument, or the extracted DID's from the `subjects` are distinct from values in the credential subject
  /// property.
  // Todo: Find a better way to describe how this operation can fail.
  pub fn try_new(
    credential: Credential<T>,
    issuer: ResolvedIotaDocument,
    subjects: OneOrMany<ResolvedIotaDocument>,
  ) -> Result<Self> {
    // check that the issuer corresponds with the issuer stated in the credential.
    //  need to parse a valid IotaDID from the credential's issuer and check that the DID matches with the provided
    // resolved DID document
    let credential_issuer_did: Result<IotaDID> = credential.issuer.url().as_str().parse();
    if let Ok(did) = credential_issuer_did {
      if &did != issuer.document.id() {
        return Err(Error::InvalidCredentialPairing(CompoundError::UnrelatedIssuer));
      }
    } else {
      return Err(Error::InvalidCredentialPairing(CompoundError::UnrelatedIssuer));
    }

    // check that the subjects correspond to the credential's subjects
    for (position, subject) in subjects.iter().enumerate() {
      if !credential.credential_subject.iter().any(|credential_subject| {
        credential_subject
          .id
          .as_ref()
          // Todo: id().to_url().to_string().as_str() is there a better way?
          // will that even work?
          .filter(|url| url == &subject.document.id().to_url().to_string().as_str())
          .is_some()
      }) {
        return Err(Error::InvalidCredentialPairing(CompoundError::UnrelatedSubjects {
          position,
        }));
      }
    }

    Ok(Self {
      credential,
      issuer,
      subjects,
    })
  }

  /// Verify the signature using the issuer's DID document.
  ///
  /// # Terminology
  /// This method is a *validation unit*
  pub fn verify_signature(&self, options: &VerifierOptions) -> Result<()> {
    CredentialValidator::verify_credential_signature(&self.credential, std::slice::from_ref(&self.issuer), options)
      .map_err(Error::UnsuccessfulValidationUnit)
  }

  /// Returns an iterator over the resolved documents that have been deactivated.
  pub fn deactivated_subject_documents(&self) -> impl Iterator<Item = &ResolvedIotaDocument> + '_ {
    self
      .subjects
      .iter()
      .map(|doc| doc)
      .filter(|resolved_doc| !resolved_doc.document.active())
  }

  /// Unpacks [`Self`] into a triple corresponding to the credential, the issuer's [ResolvedIotaDocument] and the
  /// [`ResolvedIotaDocument`]s of the subjects respectively.
  pub fn de_assemble(self) -> (Credential<T>, ResolvedIotaDocument, OneOrMany<ResolvedIotaDocument>) {
    (self.credential, self.issuer, self.subjects)
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
          pub fn types_difference_left<'b>(&'b self, input_types: &'b [&str]) -> impl Iterator<Item = &String> + 'b;

          /// Returns an iterator of `types` that are in `input_types`, but not in this Credential.
          pub fn types_difference_right<'b>(&'b self, input_types: &'b [&str]) -> impl Iterator<Item= &str> + 'b;
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
      .ok_or(super::errors::StandaloneValidationError::ExpirationDate)
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
      .ok_or(super::errors::StandaloneValidationError::IssuanceDate)
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
        super::errors::StandaloneValidationError::DeactivatedSubjectDocument {
          did_url: deactivated_doc.document.id().to_url(),
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
      .map_err(super::errors::StandaloneValidationError::CredentialStructure)
      .map_err(Into::into)
  }
}

// Todo: Create tests for verify_signature and deactivated_subject_documents
