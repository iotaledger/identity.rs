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
use super::errors::ValidationError;
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
  /// # Security
  /// It is the callers responsibility to ensure that the trusted issuers have up to date DID Documents. Furthermore
  /// most applications will want to apply their own domain specific validations as this method only covers common
  /// concerns. See the [Errors](#Errors) section to get an overview of what gets validated.
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
  ///
  /// # Security
  /// It is the callers responsibility to ensure that all supplied resolved DID Documents are up to date. Furthermore
  /// most applications will want to apply their own domain specific validations as this method only covers common
  /// concerns. See the [Errors](#Errors) section to get an overview of what gets validated.
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
  ) -> Result<(), OneOrMany<ValidationError>> {
    let mut errors = OneOrMany::empty();

    // check the structure of the credential
    if let Err(error) = credential.check_structure() {
      errors.push(ValidationError::CredentialStructure(error));
      if fail_fast {
        return Err(errors);
      }
    }

    // check that the expiry date complies with the time set in the validation options

    if let Err(error) = credential
      .expires_after(options.expires_after)
      .then(|| ())
      .ok_or(ValidationError::ExpirationDate)
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
      .ok_or(ValidationError::IssuanceDate)
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
  ) -> Result<(), ValidationError> {
    let issuer_did: Result<IotaDID> = credential.issuer.url().as_str().parse();
    match issuer_did {
      Ok(did) => {
        // if the issuer_did corresponds to one of the trusted issuers we use the corresponding DID Document to verify
        // the signature
        if let Err(issuer_proof_error) = trusted_issuers
          .into_iter()
          .find(|issuer_doc| issuer_doc.document.id() == &did)
          .ok_or(ValidationError::UntrustedIssuer)
          .and_then(|trusted_issuer_doc| {
            trusted_issuer_doc
              .document
              .verify_data(&credential, &options)
              .map_err(|error| ValidationError::IssuerProof { source: error.into() })
          })
        {
          // the credential issuer's url could be parsed to a valid IOTA DID, but verification failed for one of the
          // following reasons; Either the credential issuer is not trusted, or the credential issuer is trusted,
          // but the signature could not be verified using the issuer's resolved DID document
          return Err(issuer_proof_error);
        }
        Ok(())
      }
      Err(error) => {
        // the issuer's url could not be parsed to a valid IotaDID
        return Err(ValidationError::IssuerUrl { source: error.into() });
      }
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
    let mut validation_errors: OneOrMany<ValidationError> =
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
  ) -> Result<(), OneOrMany<ValidationError>> {
    let mut presentation_validation_errors = OneOrMany::<ValidationError>::empty();

    if let Err(error) = presentation.check_structure() {
      presentation_validation_errors.push(ValidationError::PresentationStructure(error));
      if fail_fast {
        return Err(presentation_validation_errors);
      }
    }
    if let Some((credential_position, _)) = presentation.non_transferable_violations().next() {
      presentation_validation_errors.push(ValidationError::NonTransferableViolation { credential_position });
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
  ) -> Result<(), ValidationError> {
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
    let preliminary_presentation_validation_errors: OneOrMany<ValidationError> =
      Self::preliminaty_presentation_validation(presentation, fail_fast)
        .err()
        .unwrap_or_default();

    let mut compound_error = AccumulatedPresentationValidationError {
      presentation_validation_errors: preliminary_presentation_validation_errors.into_vec(),
      credential_errors: BTreeMap::<usize, AccumulatedCredentialValidationError>::new(),
    };

    let AccumulatedPresentationValidationError {
      ref mut presentation_validation_errors,
      ref mut credential_errors,
    } = compound_error;

    // now check the holder's signature
    if presentation_validation_errors.is_empty() || (!fail_fast) {
      if let Err(proof_error) = Self::verify_presentation_signature(
        presentation,
        resolved_holder_document,
        &options.presentation_verifier_options,
      ) {
        presentation_validation_errors.push(proof_error);
      }
    }

    // if any of the preliminary validations failed and fail_fast is true we must return now
    if (!presentation_validation_errors.is_empty()) && fail_fast {
      return Err(compound_error);
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
          return Err(compound_error);
        }
      }
    }

    if presentation_validation_errors.is_empty() && credential_errors.is_empty() {
      Ok(())
    } else {
      Err(compound_error)
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Timestamp;
  use identity_core::common::Url;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::SignatureOptions;
  use identity_credential::presentation::PresentationBuilder;

  use crate::credential::CredentialValidationOptions;
  use crate::document::IotaDocument;

  use super::super::test_utils;
  use super::*;

  // generates a triple: issuer document, issuer's keys, unsigned credential issued by issuer
  fn credential_setup() -> (IotaDocument, KeyPair, Credential) {
    let (issuer_doc, issuer_key) = test_utils::generate_document_with_keys();
    let (subject_doc, _) = test_utils::generate_document_with_keys();
    let issuance_date = Timestamp::parse("2020-01-01T00:00:00Z").unwrap();
    let expiration_date = Timestamp::parse("2023-01-01T00:00:00Z").unwrap();
    let credential = test_utils::generate_credential(&issuer_doc, &[subject_doc], issuance_date, expiration_date);
    (issuer_doc, issuer_key, credential)
  }

  #[test]
  fn test_validate_credential() {
    // setup
    let (issuer_doc, issuer_key, mut credential) = credential_setup();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // declare the credential validation parameters
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

  #[test]
  fn test_validate_credential_invalid_signature() {
    // setup
    let (issuer_doc, _, mut credential) = credential_setup();
    let (_, other_keys) = test_utils::generate_document_with_keys();
    issuer_doc
      .sign_data(
        &mut credential,
        other_keys.private(), // sign with other keys
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // declare the credential validation parameters
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-12-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .issued_before(issued_before)
      .expires_after(expires_after);
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let error = match validator
      .validate_credential(&credential, &options, &[trusted_issuer], true)
      .unwrap_err()
    {
      Error::UnsuccessfulCredentialValidation(accumulated_validation_error) => {
        match accumulated_validation_error.validation_errors {
          OneOrMany::One(validation_error) => validation_error,
          _ => unreachable!(),
        }
      }
      _ => unreachable!(),
    };

    assert!(matches!(error, ValidationError::IssuerProof { .. }));
  }

  #[test]
  fn test_validate_credential_untrusted_issuer() {
    // setup
    let (issuer_doc, issuer_key, mut credential) = credential_setup();
    let (other_doc, _) = test_utils::generate_document_with_keys();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // declare the credential validation parameters
    let trusted_issuer = test_utils::mock_resolved_document(other_doc); // the trusted issuer did not sign the credential
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-12-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .issued_before(issued_before)
      .expires_after(expires_after);
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let error = match validator
      .validate_credential(&credential, &options, &[trusted_issuer], true)
      .unwrap_err()
    {
      Error::UnsuccessfulCredentialValidation(accumulated_validation_error) => {
        match accumulated_validation_error.validation_errors {
          OneOrMany::One(validation_error) => validation_error,
          _ => unreachable!(),
        }
      }
      _ => unreachable!(),
    };

    assert!(matches!(error, ValidationError::UntrustedIssuer));
  }

  #[test]
  fn test_validate_credential_invalid_expiration_date() {
    // setup
    let (issuer_doc, issuer_key, mut credential) = credential_setup();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    // declare the credential validation parameters
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2023-02-01T00:00:00Z").unwrap(); // note that expires_after > expiration_date
    let options = CredentialValidationOptions::default()
      .issued_before(issued_before)
      .expires_after(expires_after);
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let error = match validator
      .validate_credential(&credential, &options, &[trusted_issuer], true)
      .unwrap_err()
    {
      Error::UnsuccessfulCredentialValidation(accumulated_validation_error) => {
        match accumulated_validation_error.validation_errors {
          OneOrMany::One(validation_error) => validation_error,
          _ => unreachable!(),
        }
      }
      _ => unreachable!(),
    };

    assert!(matches!(error, ValidationError::ExpirationDate));
  }

  #[test]
  fn test_validate_credential_invalid_issuance_date() {
    // setup
    let (issuer_doc, issuer_key, mut credential) = credential_setup();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    // declare the credential validation parameters
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2019-02-01T00:00:00Z").unwrap(); // note that issued_before < issuance_date
    let expires_after = Timestamp::parse("2022-02-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .issued_before(issued_before)
      .expires_after(expires_after);

    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let error = match validator
      .validate_credential(&credential, &options, &[trusted_issuer], true)
      .unwrap_err()
    {
      Error::UnsuccessfulCredentialValidation(accumulated_validation_error) => {
        match accumulated_validation_error.validation_errors {
          OneOrMany::One(validation_error) => validation_error,
          _ => unreachable!(),
        }
      }
      _ => unreachable!(),
    };

    assert!(matches!(error, ValidationError::IssuanceDate));
  }

  #[test]
  fn test_validate_credential_invalid_structure() {
    // setup
    let (issuer_doc, issuer_key, mut credential) = credential_setup();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    credential.credential_subject = OneOrMany::default(); // the credential now has no credential subjects which is not semantically correct

    // declare the credential validation parameters
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-02-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .issued_before(issued_before)
      .expires_after(expires_after);
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let error = match validator
      .validate_credential(&credential, &options, &[trusted_issuer], true)
      .unwrap_err()
    {
      Error::UnsuccessfulCredentialValidation(accumulated_validation_error) => {
        match accumulated_validation_error.validation_errors {
          OneOrMany::One(validation_error) => validation_error,
          _ => unreachable!(),
        }
      }
      _ => unreachable!(),
    };

    assert!(matches!(error, ValidationError::CredentialStructure(_)));
  }

  #[test]
  fn test_validate_credential_multiple_errors_fail_fast() {
    // setup
    let (issuer_doc, issuer_key, mut credential) = credential_setup();
    let (other_issuer, _) = test_utils::generate_document_with_keys();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    credential.credential_subject = OneOrMany::default(); // the credential now has no credential subjects which is not semantically correct

    // declare the credential validation parameters
    let trusted_issuer = test_utils::mock_resolved_document(other_issuer); // trusted issuer did not issue the credential
    let issued_before = Timestamp::parse("2019-02-01T00:00:00Z").unwrap(); // issued_before < issuance_date
    let expires_after = Timestamp::parse("2024-02-01T00:00:00Z").unwrap(); // expires_after > expiration_date
    let options = CredentialValidationOptions::default()
      .issued_before(issued_before)
      .expires_after(expires_after);
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let error = match validator
      .validate_credential(&credential, &options, &[trusted_issuer], true)
      .unwrap_err()
    {
      Error::UnsuccessfulCredentialValidation(accumulated_validation_error) => {
        accumulated_validation_error.validation_errors
      }
      _ => unreachable!(),
    };

    assert!(error.len() == 1);
  }

  #[test]
  fn test_validate_credential_multiple_errors_accumulate_all_errors() {
    // setup
    let (issuer_doc, issuer_key, mut credential) = credential_setup();
    let (other_issuer, _) = test_utils::generate_document_with_keys();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    credential.credential_subject = OneOrMany::default(); // the credential now has no credential subjects which is not semantically correct [first error]

    // declare the credential validation parameters
    let trusted_issuer = test_utils::mock_resolved_document(other_issuer); // trusted issuer did not issue the credential [second error]
    let issued_before = Timestamp::parse("2019-02-01T00:00:00Z").unwrap(); // issued_before < issuance_date [third error]
    let expires_after = Timestamp::parse("2024-02-01T00:00:00Z").unwrap(); // expires_after > expiration_date [fourth error]
    let options = CredentialValidationOptions::default()
      .issued_before(issued_before)
      .expires_after(expires_after);
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let error = match validator
      .validate_credential(&credential, &options, &[trusted_issuer], false)
      .unwrap_err()
    {
      Error::UnsuccessfulCredentialValidation(accumulated_validation_error) => {
        accumulated_validation_error.validation_errors
      }
      _ => unreachable!(),
    };

    assert!(error.len() >= 4);
  }

  #[test]
  fn test_validate_presentation() {
    // create a first credential
    let (issuer_foo_doc, issuer_foo_key) = test_utils::generate_document_with_keys();
    let (subject_foo_doc, subject_foo_key) = test_utils::generate_document_with_keys();
    let issuance_date = Timestamp::parse("2019-01-01T00:00:00Z").unwrap();
    let expiration_date = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
    let mut credential_foo = test_utils::generate_credential(
      &issuer_foo_doc,
      std::slice::from_ref(&subject_foo_doc),
      issuance_date,
      expiration_date,
    );
    // set the nonTransferable option on the first credential
    credential_foo.non_transferable = Some(true);
    // sign the credential
    issuer_foo_doc
      .sign_data(
        &mut credential_foo,
        issuer_foo_key.private(),
        issuer_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // create and sign a second credential
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = credential_setup();
    issuer_bar_doc
      .sign_data(
        &mut credential_bar,
        issuer_bar_key.private(),
        issuer_bar_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    let mut presentation: Presentation = PresentationBuilder::default()
      .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs").unwrap())
      .holder(Url::parse(subject_foo_doc.id().as_ref()).unwrap())
      .credential(credential_foo)
      .credential(credential_bar)
      .build()
      .unwrap();
    // sign the presentation using subject_foo's document and private key

    subject_foo_doc
      .sign_data(
        &mut presentation,
        subject_foo_key.private(),
        subject_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::new().challenge("475a7984-1bb5-4c4c-a56f-822bccd46440".to_owned()),
      )
      .unwrap();

    // validate the presentation
    let issued_before = Timestamp::parse("2030-01-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2021-01-01T00:00:00Z").unwrap();
    let credential_validation_options = CredentialValidationOptions::default()
      .expires_after(expires_after)
      .issued_before(issued_before);
    let presentation_verifier_options =
      VerifierOptions::default().challenge("475a7984-1bb5-4c4c-a56f-822bccd46440".to_owned());
    let presentation_validation_options = PresentationValidationOptions::default()
      .with_common_validation_options(credential_validation_options)
      .with_presentation_verifier_options(presentation_verifier_options);

    let validator = CredentialValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);
    assert!(validator
      .validate_presentation(
        &presentation,
        &presentation_validation_options,
        &trusted_issuers,
        &resolved_holder_document,
        true
      )
      .is_ok());
  }

  #[test]
  fn test_validate_presentation_invalid_holder_signature() {
    // create a first credential
    let (issuer_foo_doc, issuer_foo_key) = test_utils::generate_document_with_keys();
    let (subject_foo_doc, subject_foo_key) = test_utils::generate_document_with_keys();
    let issuance_date = Timestamp::parse("2019-01-01T00:00:00Z").unwrap();
    let expiration_date = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
    let mut credential_foo = test_utils::generate_credential(
      &issuer_foo_doc,
      std::slice::from_ref(&subject_foo_doc),
      issuance_date,
      expiration_date,
    );
    // set the nonTransferable option on the first credential
    credential_foo.non_transferable = Some(true);
    // sign the credential
    issuer_foo_doc
      .sign_data(
        &mut credential_foo,
        issuer_foo_key.private(),
        issuer_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // create and sign a second credential
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = credential_setup();
    issuer_bar_doc
      .sign_data(
        &mut credential_bar,
        issuer_bar_key.private(),
        issuer_bar_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    let mut presentation: Presentation = PresentationBuilder::default()
      .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs").unwrap())
      .holder(Url::parse(subject_foo_doc.id().as_ref()).unwrap())
      .credential(credential_foo)
      .credential(credential_bar)
      .build()
      .unwrap();
    // sign the presentation using subject_foo's document and private key

    subject_foo_doc
      .sign_data(
        &mut presentation,
        subject_foo_key.private(),
        subject_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::new().challenge("some challenge".to_owned()),
      )
      .unwrap();

    // validate the presentation
    let issued_before = Timestamp::parse("2030-01-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2021-01-01T00:00:00Z").unwrap();
    let credential_validation_options = CredentialValidationOptions::default()
      .expires_after(expires_after)
      .issued_before(issued_before);
    let presentation_verifier_options = VerifierOptions::default().challenge("another challenge".to_owned()); //validate with another challenge
    let presentation_validation_options = PresentationValidationOptions::default()
      .with_common_validation_options(credential_validation_options)
      .with_presentation_verifier_options(presentation_verifier_options);

    let validator = CredentialValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    let error = match validator
      .validate_presentation(
        &presentation,
        &presentation_validation_options,
        &trusted_issuers,
        &resolved_holder_document,
        true,
      )
      .unwrap_err()
    {
      Error::UnsuccessfulPresentationValidation(mut err) => err.presentation_validation_errors.pop().unwrap(),
      _ => unreachable!(),
    };

    assert!(matches!(error, ValidationError::HolderProof { .. }));
  }

  #[test]
  fn test_validate_presentation_invalid_credential() {
    // create a first credential
    let (issuer_foo_doc, issuer_foo_key) = test_utils::generate_document_with_keys();
    let (subject_foo_doc, subject_foo_key) = test_utils::generate_document_with_keys();
    let issuance_date = Timestamp::parse("2019-01-01T00:00:00Z").unwrap();
    let expiration_date = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
    let mut credential_foo = test_utils::generate_credential(
      &issuer_foo_doc,
      std::slice::from_ref(&subject_foo_doc),
      issuance_date,
      expiration_date,
    );
    // set the nonTransferable option on the first credential
    credential_foo.non_transferable = Some(true);
    // sign the credential
    issuer_foo_doc
      .sign_data(
        &mut credential_foo,
        issuer_foo_key.private(),
        issuer_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // create and sign a second credential
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = credential_setup();
    issuer_bar_doc
      .sign_data(
        &mut credential_bar,
        issuer_bar_key.private(),
        issuer_bar_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    let mut presentation: Presentation = PresentationBuilder::default()
      .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs").unwrap())
      .holder(Url::parse(subject_foo_doc.id().as_ref()).unwrap())
      .credential(credential_foo)
      .credential(credential_bar)
      .build()
      .unwrap();
    // sign the presentation using subject_foo's document and private key

    subject_foo_doc
      .sign_data(
        &mut presentation,
        subject_foo_key.private(),
        subject_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::new().challenge("some challenge".to_owned()),
      )
      .unwrap();

    // validate the presentation
    let issued_before = Timestamp::parse("2019-01-02T00:00:00Z").unwrap(); // only the first credential is issued before this date
    let expires_after = Timestamp::parse("2021-01-01T00:00:00Z").unwrap();
    let credential_validation_options = CredentialValidationOptions::default()
      .expires_after(expires_after)
      .issued_before(issued_before);
    let presentation_verifier_options = VerifierOptions::default().challenge("some challenge".to_owned());
    let presentation_validation_options = PresentationValidationOptions::default()
      .with_common_validation_options(credential_validation_options)
      .with_presentation_verifier_options(presentation_verifier_options);

    let validator = CredentialValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    let error = match validator
      .validate_presentation(
        &presentation,
        &presentation_validation_options,
        &trusted_issuers,
        &resolved_holder_document,
        true,
      )
      .unwrap_err()
    {
      Error::UnsuccessfulPresentationValidation(err) => {
        assert!(err.presentation_validation_errors.is_empty());
        err.credential_errors
      }
      _ => unreachable!(),
    };

    assert!(matches!(
      error.get(&1),
      Some(_) // the credential at position 1 fails validation
    ));
  }

  #[test]
  fn test_validate_presentation_non_transferable_property_violation() {
    // create a first credential
    let (issuer_foo_doc, issuer_foo_key) = test_utils::generate_document_with_keys();
    let (subject_foo_doc, subject_foo_key) = test_utils::generate_document_with_keys();
    let issuance_date = Timestamp::parse("2019-01-01T00:00:00Z").unwrap();
    let expiration_date = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
    let mut credential_foo = test_utils::generate_credential(
      &issuer_foo_doc,
      std::slice::from_ref(&subject_foo_doc),
      issuance_date,
      expiration_date,
    );
    // set the nonTransferable option on the first credential
    credential_foo.non_transferable = Some(true);
    // sign the credential
    issuer_foo_doc
      .sign_data(
        &mut credential_foo,
        issuer_foo_key.private(),
        issuer_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // create and sign a second credential
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = credential_setup();
    issuer_bar_doc
      .sign_data(
        &mut credential_bar,
        issuer_bar_key.private(),
        issuer_bar_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    // create a presentation where the subject of the second credential is the holder.
    // This violates the non transferable property of the first credential.
    let mut presentation: Presentation = PresentationBuilder::default()
      .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs").unwrap())
      .holder(
        credential_bar
          .credential_subject
          .first()
          .and_then(|subject| subject.id.as_ref())
          .unwrap()
          .clone(),
      )
      .credential(credential_foo)
      .credential(credential_bar)
      .build()
      .unwrap();
    // sign the presentation using subject_foo's document and private key.

    subject_foo_doc
      .sign_data(
        &mut presentation,
        subject_foo_key.private(),
        subject_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::new().challenge("some challenge".to_owned()),
      )
      .unwrap();

    // validate the presentation
    let issued_before = Timestamp::parse("2020-02-02T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2021-01-01T00:00:00Z").unwrap();
    let credential_validation_options = CredentialValidationOptions::default()
      .expires_after(expires_after)
      .issued_before(issued_before);
    let presentation_verifier_options = VerifierOptions::default().challenge("some challenge".to_owned());
    let presentation_validation_options = PresentationValidationOptions::default()
      .with_common_validation_options(credential_validation_options)
      .with_presentation_verifier_options(presentation_verifier_options);

    let validator = CredentialValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    let error = match validator
      .validate_presentation(
        &presentation,
        &presentation_validation_options,
        &trusted_issuers,
        &resolved_holder_document,
        true,
      )
      .unwrap_err()
    {
      Error::UnsuccessfulPresentationValidation(mut err) => err.presentation_validation_errors.pop().unwrap(),
      _ => unreachable!(),
    };

    assert!(matches!(
      error,
      ValidationError::NonTransferableViolation { credential_position: 0 }
    ));
  }

  #[test]
  fn test_validate_presentation_multiple_errors_accumulate_errors() {
    // create a first credential
    let (issuer_foo_doc, issuer_foo_key) = test_utils::generate_document_with_keys();
    let (subject_foo_doc, subject_foo_key) = test_utils::generate_document_with_keys();
    let issuance_date = Timestamp::parse("2019-01-01T00:00:00Z").unwrap();
    let expiration_date = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
    let mut credential_foo = test_utils::generate_credential(
      &issuer_foo_doc,
      std::slice::from_ref(&subject_foo_doc),
      issuance_date,
      expiration_date,
    );
    // set the nonTransferable option on the first credential
    credential_foo.non_transferable = Some(true);
    // sign the credential
    issuer_foo_doc
      .sign_data(
        &mut credential_foo,
        issuer_foo_key.private(),
        issuer_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // create and sign a second credential
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = credential_setup();
    issuer_bar_doc
      .sign_data(
        &mut credential_bar,
        issuer_bar_key.private(),
        issuer_bar_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    // create a presentation where the subject of the second credential is the holder.
    // This violates the non transferable property of the first credential.
    let mut presentation: Presentation = PresentationBuilder::default()
      .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs").unwrap())
      .holder(
        credential_bar
          .credential_subject
          .first()
          .and_then(|subject| subject.id.as_ref())
          .unwrap()
          .clone(),
      )
      .credential(credential_foo)
      .credential(credential_bar)
      .build()
      .unwrap();
    // sign the presentation using subject_foo's document and private key

    subject_foo_doc
      .sign_data(
        &mut presentation,
        subject_foo_key.private(),
        subject_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::new().challenge("some challenge".to_owned()),
      )
      .unwrap();

    // validate the presentation
    let issued_before = Timestamp::parse("2010-02-02T00:00:00Z").unwrap(); // both credentials were issued after this
    let expires_after = Timestamp::parse("2050-01-01T00:00:00Z").unwrap(); // both credentials expire before this
    let credential_validation_options = CredentialValidationOptions::default()
      .expires_after(expires_after)
      .issued_before(issued_before);
    let presentation_verifier_options = VerifierOptions::default().challenge("another challenge".to_owned()); // verify with another challenge
    let presentation_validation_options = PresentationValidationOptions::default()
      .with_common_validation_options(credential_validation_options)
      .with_presentation_verifier_options(presentation_verifier_options);

    let validator = CredentialValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    let (presentation_validation_errors, credential_errors) = match validator
      .validate_presentation(
        &presentation,
        &presentation_validation_options,
        &trusted_issuers,
        &resolved_holder_document,
        false,
      )
      .unwrap_err()
    {
      Error::UnsuccessfulPresentationValidation(err) => (err.presentation_validation_errors, err.credential_errors),
      _ => unreachable!(),
    };

    assert!(
      presentation_validation_errors.len()
        + credential_errors
          .values()
          .map(|error| error.validation_errors.len())
          .sum::<usize>()
        >= 6
    );
  }

  #[test]
  fn test_validate_presentation_multiple_errors_fail_fast() {
    // create a first credential
    let (issuer_foo_doc, issuer_foo_key) = test_utils::generate_document_with_keys();
    let (subject_foo_doc, subject_foo_key) = test_utils::generate_document_with_keys();
    let issuance_date = Timestamp::parse("2019-01-01T00:00:00Z").unwrap();
    let expiration_date = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
    let mut credential_foo = test_utils::generate_credential(
      &issuer_foo_doc,
      std::slice::from_ref(&subject_foo_doc),
      issuance_date,
      expiration_date,
    );
    // set the nonTransferable option on the first credential
    credential_foo.non_transferable = Some(true);
    // sign the credential
    issuer_foo_doc
      .sign_data(
        &mut credential_foo,
        issuer_foo_key.private(),
        issuer_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // create and sign a second credential
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = credential_setup();
    issuer_bar_doc
      .sign_data(
        &mut credential_bar,
        issuer_bar_key.private(),
        issuer_bar_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    // create a presentation where the subject of the second credential is the holder.
    // This violates the non transferable property of the first credential.
    let mut presentation: Presentation = PresentationBuilder::default()
      .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs").unwrap())
      .holder(
        credential_bar
          .credential_subject
          .first()
          .and_then(|subject| subject.id.as_ref())
          .unwrap()
          .clone(),
      )
      .credential(credential_foo)
      .credential(credential_bar)
      .build()
      .unwrap();
    // sign the presentation using subject_foo's document and private key

    subject_foo_doc
      .sign_data(
        &mut presentation,
        subject_foo_key.private(),
        subject_foo_doc.default_signing_method().unwrap().id(),
        SignatureOptions::new().challenge("some challenge".to_owned()),
      )
      .unwrap();

    // validate the presentation
    let issued_before = Timestamp::parse("2010-02-02T00:00:00Z").unwrap(); // both credentials were issued after this
    let expires_after = Timestamp::parse("2050-01-01T00:00:00Z").unwrap(); // both credentials expire before this
    let credential_validation_options = CredentialValidationOptions::default()
      .expires_after(expires_after)
      .issued_before(issued_before);
    let presentation_verifier_options = VerifierOptions::default().challenge("another challenge".to_owned()); // verify with another challenge
    let presentation_validation_options = PresentationValidationOptions::default()
      .with_common_validation_options(credential_validation_options)
      .with_presentation_verifier_options(presentation_verifier_options);

    let validator = CredentialValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    let (presentation_validation_errors, credential_errors) = match validator
      .validate_presentation(
        &presentation,
        &presentation_validation_options,
        &trusted_issuers,
        &resolved_holder_document,
        true,
      )
      .unwrap_err()
    {
      Error::UnsuccessfulPresentationValidation(err) => (err.presentation_validation_errors, err.credential_errors),
      _ => unreachable!(),
    };

    assert!(
      presentation_validation_errors.len()
        + credential_errors
          .values()
          .map(|error| error.validation_errors.len())
          .sum::<usize>()
        == 1
    );
  }
}
