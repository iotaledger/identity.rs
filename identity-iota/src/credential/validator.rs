// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::marker::PhantomData;

use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::Error;
use crate::Result;
use identity_core::common::OneOrMany;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::AccumulatedCredentialValidationError;
use super::errors::AccumulatedPresentationValidationError;
use super::errors::StandaloneValidationError;
use super::CredentialValidationOptions;
use super::PresentationValidationOptions;

pub struct CredentialValidator {
  _future_proofing: PhantomData<u8>,
}

impl CredentialValidator {
  /// Constructs a new [CredentialValidator]
  pub fn new() -> Self {
    Self {
      _future_proofing: PhantomData,
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
    trusted_issuers: &[ResolvedIotaDocument],
    fail_fast: bool,
  ) -> Result<()> {
    self
      .validate_credential_internal(credential, options, trusted_issuers, fail_fast)
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
    trusted_issuers: &[ResolvedIotaDocument],
    resolved_holder_document: &ResolvedIotaDocument,
    fail_fast: bool,
  ) -> Result<()> {
    self
      .validate_presentation_internal(
        presentation,
        options,
        trusted_issuers,
        resolved_holder_document,
        fail_fast,
      )
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

  /// Checks the [Credential]'s signature.
  ///
  /// If the Credential's issuer corresponds to one of the `trusted_issuer`'s then the signature will be verified using
  /// this issuer's DID document.
  pub(crate) fn verify_credential_signature<T: Serialize>(
    credential: &Credential<T>,
    trusted_issuers: &[ResolvedIotaDocument],
    options: &VerifierOptions,
  ) -> Result<(), StandaloneValidationError> {
    let issuer_did: Result<IotaDID> = credential.issuer.url().as_str().parse();
    if let Ok(did) = issuer_did {
      // if the issuer_did corresponds to one of the trusted issuers we use the corresponding DID Document to verify
      // the signature
      if let Err(issuer_proof_error) = trusted_issuers
        .into_iter()
        .find(|issuer_doc| issuer_doc.document.id() == &did)
        .ok_or(StandaloneValidationError::UntrustedIssuer)
        .and_then(|trusted_issuer_doc| {
          trusted_issuer_doc
            .document
            .verify_data(&credential, &options)
            .map_err(|error| StandaloneValidationError::IssuerProof { source: error.into() })
        })
      {
        // the credential issuer's url could be parsed to a valid IOTA DID, but verification failed for one of the
        // following reasons; Either the credential issuer is not trusted, or the credential issuer is trusted,
        // but the signature could not be verified using the issuer's resolved DID document
        return Err(issuer_proof_error);
      }
      Ok(())
    } else {
      // the issuer's url could not be parsed to a valid IotaDID
      return Err(StandaloneValidationError::IssuerUrl);
    }
  }

  // helper function to see the kind of error validation may yield
  fn validate_credential_internal<T: Serialize>(
    &self,
    credential: &Credential<T>,
    options: &CredentialValidationOptions,
    trusted_issuers: &[ResolvedIotaDocument],
    fail_fast: bool,
  ) -> Result<(), AccumulatedCredentialValidationError> {
    // first run the preliminary validation checks not requiring any DID Documents
    let mut validation_errors: OneOrMany<StandaloneValidationError> =
      Self::preliminary_credential_validation(credential, options, fail_fast)
        .err()
        .unwrap_or_default();
    // now check the credential's signature
    if validation_errors.is_empty() || !fail_fast {
      if let Err(proof_error) =
        Self::verify_credential_signature(credential, trusted_issuers, &options.verifier_options)
      {
        validation_errors.push(proof_error);
      }
    }
    if validation_errors.is_empty() {
      Ok(())
    } else {
      Err(AccumulatedCredentialValidationError { validation_errors })
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

  /// Verify the presentation's signature using the resolved document of the holder
  ///
  /// # Errors
  /// Fails if the supplied `resolved_holder_document` cannot be identified with the URL of the `presentation`'s holder
  /// property
  pub(crate) fn verify_presentation_signature<T: Serialize, S: Serialize>(
    presentation: &Presentation<T, S>,
    resolved_holder_document: &ResolvedIotaDocument,
    options: &VerifierOptions,
  ) -> Result<(), StandaloneValidationError> {
    let did: IotaDID = presentation
      .holder
      .as_ref()
      .ok_or(StandaloneValidationError::HolderUrl)
      .and_then(|value| IotaDID::parse(value.as_str()).map_err(|_| StandaloneValidationError::HolderUrl))?;
    if &did != resolved_holder_document.document.id() {
      return Err(StandaloneValidationError::IncompatibleHolderDocument);
    }
    resolved_holder_document
      .document
      .verify_data(&presentation, options)
      .map_err(|error| StandaloneValidationError::HolderProof { source: error.into() })
  }

  // helper function to see the kind of error validation may yield
  fn validate_presentation_internal<T: Serialize, S: Serialize>(
    &self,
    presentation: &Presentation<T, S>,
    options: &PresentationValidationOptions,
    trusted_issuers: &[ResolvedIotaDocument],
    resolved_holder_document: &ResolvedIotaDocument,
    fail_fast: bool,
  ) -> Result<(), AccumulatedPresentationValidationError> {
    // first run some preliminary validation checks directly on the presentation
    let preliminary_presentation_validation_errors: OneOrMany<StandaloneValidationError> =
      Self::preliminaty_presentation_validation(presentation, fail_fast)
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

    // now check the holder's signature
    if presentation_validation_errors.is_empty() || (!fail_fast) {
      if let Err(proof_error) = Self::verify_presentation_signature(
        presentation,
        resolved_holder_document,
        &options.common_validation_options.verifier_options,
      ) {
        presentation_validation_errors.push(proof_error);
      }
    }

    // if any of the preliminary validations failed and fail_fast is true we must return now
    if (!presentation_validation_errors.is_empty()) && fail_fast {
      return Err(presentation_resolution_error);
    }

    // validate the presentations credentials
    for (position, credential) in presentation.verifiable_credential.iter().enumerate() {
      if let Err(error) = self.validate_credential_internal(
        credential,
        &options.common_validation_options,
        trusted_issuers,
        fail_fast,
      ) {
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

#[cfg(test)]
mod tests {
  use identity_core::common::Timestamp;
  use identity_core::crypto::SignatureOptions;

  use crate::credential::CredentialValidationOptions;

  use super::super::test_utils;
  use super::CredentialValidator;
  #[test]
  fn test_validate_credential() {
    // setup
    let (issuer_doc, issuer_key) = test_utils::generate_document_with_keys();
    let (subject_doc, _) = test_utils::generate_document_with_keys();
    let issuance_date = Timestamp::parse("2020-01-01T00:00:00Z").unwrap();
    let expiration_date = Timestamp::parse("2023-01-01T00:00:00Z").unwrap();
    let mut credential = test_utils::generate_credential(&issuer_doc, &[subject_doc], issuance_date, expiration_date);
    issuer_doc.sign_data(
      &mut credential,
      issuer_key.private(),
      issuer_doc.default_signing_method().unwrap().id(),
      SignatureOptions::default(),
    );
    // validate credential
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-12-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .issued_before(issued_before)
      .expires_after(expires_after);
    let validator = CredentialValidator::new();
    assert!(validator
      .validate_credential(&credential, &options, &[trusted_issuer], true)
      .is_ok());
  }

  fn test_validate_credential_invalid_signature() {}

  fn test_validate_credential_untrusted_issuer() {}

  fn test_validate_credential_invalid_expiration_date() {}

  fn test_validate_credential_invalid_issuance_date() {}

  fn test_validate_credential_invalid_structure() {}

  fn test_validate_credential_multiple_errors() {}

  fn test_validate_presentation() {}

  fn test_validate_presentation_invalid_holder_signature() {}

  fn test_validate_presentation_invalid_credential() {}

  fn test_validate_presentation_non_transferable_property_violation() {}

  fn test_validate_presentation_multiple_errors() {}
}
