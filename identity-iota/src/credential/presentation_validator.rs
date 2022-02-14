// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::ops::ControlFlow;

use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::AccumulatedCredentialValidationError;
use super::errors::AccumulatedPresentationValidationError;
use super::errors::ValidationError;
use super::CredentialValidationOptions;
use super::PresentationValidationOptions;
use crate::credential::credential_validator::CredentialValidator;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::Error;
use crate::Result;

/// A struct for validating the given [Presentation].
pub struct PresentationValidator<U, V, T: Borrow<Presentation<U, V>>> {
  pub presentation: T,
  _required: (PhantomData<U>, PhantomData<V>), // will not compile without this field
}

// Use the following pattern throughout: for each public method there is corresponding private method returning a more
// local error.
type ValidationUnitResult = std::result::Result<(), ValidationError>;
type PresentationValidationResult = std::result::Result<(), AccumulatedPresentationValidationError>;

impl<U, V, T: Borrow<Presentation<U, V>>> PresentationValidator<U, V, T> {
  /// Constructs a new [PresentationValidator]
  pub fn new(presentation: T) -> Self {
    Self {
      presentation,
      _required: (PhantomData::<U>, PhantomData::<V>),
    }
  }

  /// Validates the semantic structure of the `Presentation`.
  pub fn check_structure(&self) -> Result<()> {
    self
      .check_structure_local_error()
      .map_err(Error::UnsuccessfulValidationUnit)
  }

  fn check_structure_local_error(&self) -> ValidationUnitResult {
    self
      .presentation
      .borrow()
      .check_structure()
      .map_err(ValidationError::PresentationStructure)
  }

  // An iterator over the credentials (with their corresponding position in the presentation) that have the
  // `nonTransferable` property set, but the credential subject id does not correspond to URL of the presentation's
  // holder
  fn non_transferable_violations(&self) -> impl Iterator<Item = (usize, &Credential<V>)> + '_ {
    let presentation = self.presentation.borrow();
    presentation
      .verifiable_credential
      .as_ref()
      .iter()
      .enumerate()
      .filter(|(_, credential)| {
        if credential.non_transferable.filter(|value| *value).is_some() {
          if let OneOrMany::One(ref credential_subject) = credential.credential_subject {
            credential_subject.id != presentation.holder
          } else {
            // if the nonTransferable property is set the credential must have exactly one credentialSubject
            true
          }
        } else {
          false
        }
      })
  }

  /// Validates that the nonTransferable property is met.
  ///
  /// # Errors
  /// Returns at the first credential requiring a nonTransferable property that is not met.
  ///
  /// If one needs to find *all* the nonTransferable violations of this presentation, then see
  /// [Self::non_transferable_violations] .
  ///
  /// # Terminology
  ///
  /// This is a *validation unit*
  pub fn check_non_transferable(&self) -> Result<()> {
    self
      .check_non_transferable_local_error()
      .map_err(Error::UnsuccessfulValidationUnit)
  }

  fn check_non_transferable_local_error(&self) -> ValidationUnitResult {
    if let Some((position, _)) = self.non_transferable_violations().next() {
      let err = ValidationError::NonTransferableViolation {
        credential_position: position,
      };
      Err(err)
    } else {
      Ok(())
    }
  }
}

impl<U: Serialize, V: Serialize, T: Borrow<Presentation<U, V>>> PresentationValidator<U, V, T> {
  /// Verify the presentation's signature using the resolved document of the holder
  ///
  /// # Errors
  /// Fails if the supplied `resolved_holder_document` cannot be identified with the URL of the `presentation`'s holder
  /// property
  pub fn verify_presentation_signature_local_error(
    &self,
    resolved_holder_document: &ResolvedIotaDocument,
    options: &VerifierOptions,
  ) -> ValidationUnitResult {
    let presentation = self.presentation.borrow();
    let did: IotaDID = presentation
      .holder
      .as_ref()
      .ok_or(ValidationError::MissingPresentationHolder)
      .and_then(|value| {
        IotaDID::parse(value.as_str()).map_err(|error| ValidationError::HolderUrl { source: error.into() })
      })?;
    if &did != resolved_holder_document.document.id() {
      return Err(ValidationError::IncompatibleHolderDocument);
    }
    resolved_holder_document
      .document
      .verify_data(&presentation, options)
      .map_err(|error| ValidationError::HolderProof { source: error.into() })
  }

  /// Validate a `Presentation`
  ///
  /// Checks common concerns such as the holder's signature, the nonTransferable property, the semantic structure of the
  /// presentation and common concerns regarding credential validation (see
  /// [`CredentialValidator::full_validation()`]). # Security
  /// It is the callers responsibility to ensure that all supplied resolved DID Documents are up to date. Furthermore
  /// most applications will want to apply their own domain specific validations as this method only covers common
  /// concerns.
  /// # Errors
  /// Fails if any of the following conditions occur
  /// - The structure of the presentation is not semantically valid
  /// - The nonTransferable property is set in one of the credentials, but the credential's subject is not the holder of
  ///   the presentation.
  /// - Validation of any of the presentation's credentials fails.
  pub fn full_validation(
    &self,
    options: &PresentationValidationOptions,
    resolved_holder_document: &ResolvedIotaDocument,
    trusted_issuers: &[ResolvedIotaDocument],
  ) -> Result<()> {
    self
      .full_validation_local_error(options, resolved_holder_document, trusted_issuers)
      .map_err(Error::UnsuccessfulPresentationValidation)
  }

  fn full_validation_local_error(
    &self,
    options: &PresentationValidationOptions,
    resolved_holder_document: &ResolvedIotaDocument,
    trusted_issuers: &[ResolvedIotaDocument],
  ) -> PresentationValidationResult {
    let fail_fast = options.fail_fast;

    // setup error container
    let mut compound_error = AccumulatedPresentationValidationError {
      presentation_validation_errors: Vec::new(),
      credential_errors: BTreeMap::<usize, AccumulatedCredentialValidationError>::new(),
    };

    let AccumulatedPresentationValidationError {
      ref mut presentation_validation_errors,
      ref mut credential_errors,
    } = compound_error;

    if let Err(structure_error) = self.check_structure_local_error() {
      presentation_validation_errors.push(structure_error);
      if fail_fast {
        return Err(compound_error);
      }
    }

    if let Err(non_transferable_error) = self.check_non_transferable_local_error() {
      presentation_validation_errors.push(non_transferable_error);
      if fail_fast {
        return Err(compound_error);
      }
    }

    if let Err(signature_error) =
      self.verify_presentation_signature_local_error(resolved_holder_document, &options.presentation_verifier_options)
    {
      presentation_validation_errors.push(signature_error);
      if fail_fast {
        return Err(compound_error);
      }
    }

    // now validate the credentials
    for (position, credential) in self.presentation.borrow().verifiable_credential.iter().enumerate() {
      if let Err(credential_error) = CredentialValidator::new(credential)
        .full_validation_local_error(&CredentialValidationOptions::default(), trusted_issuers)
      {
        credential_errors.insert(position, credential_error);
        if fail_fast {
          return Err(compound_error);
        }
      }
    }

    Ok(())
  }
}
