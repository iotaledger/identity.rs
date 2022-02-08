// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::marker::PhantomData;

use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::Error;
use crate::Result;
use identity_core::common::OneOrMany;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use serde::Serialize;

use super::errors::AccumulatedCredentialValidationError;
use super::errors::AccumulatedPresentationValidationError;
use super::errors::StandaloneValidationError;
use super::CredentialValidationOptions;
use super::PresentationValidationOptions;

pub struct CredentialValidator<U: Borrow<Vec<ResolvedIotaDocument>>> {
  trusted_issuers: U,
  _future_proofing: PhantomData<u8>,
}

impl<U: Borrow<Vec<ResolvedIotaDocument>>> CredentialValidator<U> {

  /// Constructs a new [CredentialValidator] 
  pub fn new(trusted_issuers: U) -> Self {
    Self {
      trusted_issuers,
      _future_proofing: PhantomData
    }
  }

  /// Validate a `Credential`.
  ///
  /// # Errors
  /// Fails if any of the following conditions occur
  /// - The structure of the credential is not semantically valid
  /// - The expiration date does not meet the requirement set in `options`
  /// - The issuance date does not meet the requirement set in `options`
  /// - The issuer has not been specified as trust
  /// - The credential's signature cannot be verified using the issuer's DID Document
  ///
  /// Fails on the first encountered error if `fail_fast` is true, otherwise all
  /// errors will be accumulated in the returned error.
  pub fn validate_credential<T: Serialize>(
    &self,
    credential: &Credential<T>,
    options: &CredentialValidationOptions,
    fail_fast: bool,
  ) -> Result<()> {
    self
      .validate_credential_internal(credential, options, fail_fast)
      .map_err(Error::UnsuccessfulCredentialValidation)
  }

  /// Validate a `Presentation`
  ///
  /// # Errors
  /// Fails if any of the following conditions occur
  /// - The structure of the presentation is not semantically valid
  /// - The nonTransferable property is set in one of the credentials, but the credential's subject is not the holder of
  ///   the presentation.
  /// - Validation of any of the presentation's credentials fails.
  pub fn validate_presentation<T: Serialize, S: Serialize>(
    &self,
    presentation: &Presentation<T, S>,
    options: &PresentationValidationOptions,
    fail_fast: bool,
  ) -> Result<()> {
    self
      .validate_presentation_internal(presentation, options, fail_fast)
      .map_err(Error::UnsuccessfulPresentationValidation)
  }

  /// Validate the structure, expiration date and issuance date of the given credential.
  ///
  /// # Errors
  /// Fails on the first encountered error if `fail_fast` is true, otherwise all errors will be accumulated in the
  /// returned error.
  pub(crate) fn preliminary_credential_validation<T>(
    credential: &Credential<T>,
    options: &CredentialValidationOptions,
    fail_fast: bool,
  ) -> Result<(), OneOrMany<StandaloneValidationError>> {
    let mut errors = OneOrMany::empty();

    // check the structure of the credential
    if let Err(error) = credential.check_structure() {
      errors.push(StandaloneValidationError::CredentialStructure(error));
      if fail_fast {
        return Err(errors);
      }
    }

    // check that the expiry date complies with the time set in the validation options

    if let Err(error) = credential
      .expires_after(options.expires_after)
      .then(|| ())
      .ok_or(StandaloneValidationError::ExpirationDate)
    {
      errors.push(error);
      if fail_fast {
        return Err(errors);
      }
    }

    // check that the issuance date complies with the time set in the validation options

    if let Err(error) = credential
      .issued_before(options.issued_before)
      .then(|| ())
      .ok_or(StandaloneValidationError::IssuanceDate)
    {
      errors.push(error);
      if fail_fast {
        return Err(errors);
      }
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }

  // helper function to see the kind of error validation may yield
  fn validate_credential_internal<T: Serialize>(
    &self,
    credential: &Credential<T>,
    options: &CredentialValidationOptions,
    fail_fast: bool,
  ) -> Result<(), AccumulatedCredentialValidationError> {
    // first run the preliminary validation checks not requiring any DID Documents
    let validation_errors = Self::preliminary_credential_validation(credential, options, fail_fast)
      .err()
      .unwrap_or_default();
    let mut errors = AccumulatedCredentialValidationError { validation_errors };
    if (!errors.validation_errors.is_empty()) && fail_fast {
      return Err(errors);
    }

    // now check the issuer's signature
    let issuer_did: Result<IotaDID> = credential.issuer.url().as_str().parse();
    if let Ok(did) = issuer_did {
      let trusted_issuers: &Vec<ResolvedIotaDocument> = self.trusted_issuers.borrow();
      // if the issuer_did corresponds to one of the trusted issuers we use the corresponding DID Document to verify
      // the signature
      if let Err(issuer_proof_error) = trusted_issuers
        .into_iter()
        .find(|issuer_doc| issuer_doc.document.id() == &did)
        .ok_or(StandaloneValidationError::UntrustedIssuer)
        .and_then(|trusted_issuer_doc| {
          trusted_issuer_doc
            .document
            .verify_data(&credential, &options.verifier_options)
            .map_err(|error| StandaloneValidationError::IssuerProof { source: error.into() })
        })
      {
        // the credential issuer's url could be parsed to a valid IOTA DID, but verification failed for one of the
        // following reasons; Either the credential issuer is not trusted, or the credential issuer is trusted,
        // but the signature could not be verified using the issuer's resolved DID document
        errors.validation_errors.push(issuer_proof_error);
        if fail_fast {
          return Err(errors);
        }
      }
    } else {
      // the issuer's url could not be parsed to a valid IotaDID
      errors.validation_errors.push(StandaloneValidationError::IssuerUrl);
      if fail_fast {
        return Err(errors);
      }
    }

    if errors.validation_errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }

  /// Validates the structure and nonTransferable property of the [`Presentation`].
  ///
  /// # Errors
  /// Fails on the first encountered error if `fail_fast` is true, otherwise all errors will be accumulated in the
  /// returned error.
  pub(crate) fn preliminaty_presentation_validation<T, S>(
    presentation: &Presentation<T, S>,
    fail_fast: bool,
  ) -> Result<(), OneOrMany<StandaloneValidationError>> {
    let mut presentation_validation_errors = OneOrMany::<StandaloneValidationError>::empty();

    if let Err(error) = presentation.check_structure() {
      presentation_validation_errors.push(StandaloneValidationError::PresentationStructure(error));
      if fail_fast {
        return Err(presentation_validation_errors);
      }
    }
    if let Some((credential_position, _)) = presentation.non_transferable_violations().next() {
      presentation_validation_errors.push(StandaloneValidationError::NonTransferableViolation { credential_position });
      if fail_fast {
        return Err(presentation_validation_errors);
      }
    }

    if presentation_validation_errors.is_empty() {
      Ok(())
    } else {
      Err(presentation_validation_errors)
    }
  }

  // helper function to see the kind of error validation may yield
  fn validate_presentation_internal<T: Serialize, S: Serialize>(
    &self,
    presentation: &Presentation<T, S>,
    options: &PresentationValidationOptions,
    fail_fast: bool,
  ) -> Result<(), AccumulatedPresentationValidationError> {
    // first run some preliminary validation checks directly on the presentation
    let preliminary_presentation_validation_errors = Self::preliminaty_presentation_validation(presentation, fail_fast)
      .err()
      .unwrap_or_default();
    let mut presentation_resolution_error = AccumulatedPresentationValidationError {
      presentation_validation_errors: preliminary_presentation_validation_errors.into_vec(),
      credential_errors: BTreeMap::<usize, AccumulatedCredentialValidationError>::new(),
    };

    let AccumulatedPresentationValidationError {
      ref mut presentation_validation_errors,
      ref mut credential_errors,
    } = presentation_resolution_error;

    // if any of the preliminary validations failed and fail_fast is true we must return now
    if (!presentation_validation_errors.is_empty()) && fail_fast {
      return Err(presentation_resolution_error);
    }

    // validate the presentations credentials
    for (position, credential) in presentation.verifiable_credential.iter().enumerate() {
      if let Err(error) = self.validate_credential_internal(credential, &options.common_validation_options, fail_fast) {
        credential_errors.insert(position, error);
        if fail_fast {
          return Err(presentation_resolution_error);
        }
      }
    }

    if presentation_validation_errors.is_empty() && credential_errors.is_empty() {
      Ok(())
    } else {
      Err(presentation_resolution_error)
    }
  }
}
