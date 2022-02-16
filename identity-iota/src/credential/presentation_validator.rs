// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use identity_core::common::OneOrMany;
use identity_credential::presentation::Presentation;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::AccumulatedCredentialValidationError;
use super::errors::AccumulatedPresentationValidationError;
use super::errors::ValidationError;
use super::PresentationValidationOptions;
use crate::credential::credential_validator::CredentialValidator;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::Error;
use crate::Result;

/// A struct for validating [Presentation]s.
#[non_exhaustive]
pub struct PresentationValidator;

// Use the following pattern throughout: for each public method there is corresponding private method returning a more
// local error.
type ValidationUnitResult = std::result::Result<(), ValidationError>;
type PresentationValidationResult = std::result::Result<(), AccumulatedPresentationValidationError>;

impl Default for PresentationValidator {
  fn default() -> Self {
    Self::new()
  }
}
impl PresentationValidator {
  /// Constructs a new [PresentationValidator]
  pub fn new() -> Self {
    Self {}
  }

  /// Validates the semantic structure of the `Presentation`.
  ///
  /// # Terminology
  /// This is a *validation unit*.
  pub fn check_structure<U, V>(presentation: &Presentation<U, V>) -> Result<()> {
    Self::check_structure_local_error(presentation).map_err(Error::UnsuccessfulValidationUnit)
  }

  fn check_structure_local_error<U, V>(presentation: &Presentation<U, V>) -> ValidationUnitResult {
    presentation
      .check_structure()
      .map_err(ValidationError::PresentationStructure)
  }

  /// An iterator over the indices corresponding to the credentials that have the
  /// `nonTransferable` property set, but the credential subject id does not correspond to URL of the presentation's
  /// holder
  pub fn non_transferable_violations<U, V>(presentation: &Presentation<U, V>) -> impl Iterator<Item = usize> + '_ {
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
      .map(|(position, _)| position)
  }

  /// Validates that the nonTransferable property is met.
  ///
  /// # Errors
  /// Returns at the first credential requiring a nonTransferable property that is not met.
  ///
  /// If one needs to find *all* the nonTransferable violations of this presentation, then see
  /// [Self::non_transferable_violations].
  ///
  /// # Terminology
  /// This is a *validation unit*
  pub fn check_non_transferable<U, V>(presentation: &Presentation<U, V>) -> Result<()> {
    Self::check_non_transferable_local_error(presentation).map_err(Error::UnsuccessfulValidationUnit)
  }

  fn check_non_transferable_local_error<U, V>(presentation: &Presentation<U, V>) -> ValidationUnitResult {
    if let Some(position) = Self::non_transferable_violations(presentation).next() {
      let err = ValidationError::NonTransferableViolation {
        credential_position: position,
      };
      Err(err)
    } else {
      Ok(())
    }
  }

  /// Verify the presentation's signature using the resolved document of the holder
  ///
  /// # Errors
  /// Fails if the supplied `resolved_holder_document` cannot be identified with the URL of the `presentation`'s holder
  /// property
  ///
  /// # Terminology
  /// This is a *validation unit*.
  pub fn verify_presentation_signature<U: Serialize, V: Serialize>(
    presentation: &Presentation<U, V>,
    resolved_holder_document: &ResolvedIotaDocument,
    options: &VerifierOptions,
  ) -> Result<()> {
    Self::verify_presentation_signature_local_error(presentation, resolved_holder_document, options)
      .map_err(Error::UnsuccessfulValidationUnit)
  }

  fn verify_presentation_signature_local_error<U: Serialize, V: Serialize>(
    presentation: &Presentation<U, V>,
    resolved_holder_document: &ResolvedIotaDocument,
    options: &VerifierOptions,
  ) -> ValidationUnitResult {
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
  // Takes &self in case this method will need some pre-computed state in the future.
  pub fn full_validation<U: Serialize, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    resolved_holder_document: &ResolvedIotaDocument,
    trusted_issuers: &[ResolvedIotaDocument],
  ) -> Result<()> {
    self
      .full_validation_local_error(presentation, options, resolved_holder_document, trusted_issuers)
      .map_err(Error::UnsuccessfulPresentationValidation)
  }

  fn full_validation_local_error<U: Serialize, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    resolved_holder_document: &ResolvedIotaDocument,
    trusted_issuers: &[ResolvedIotaDocument],
  ) -> PresentationValidationResult {
    let fail_fast = options.fail_fast;
    let mut observed_error = false;

    // setup error container
    let mut compound_error = AccumulatedPresentationValidationError {
      presentation_validation_errors: Vec::new(),
      credential_errors: BTreeMap::<usize, AccumulatedCredentialValidationError>::new(),
    };

    let AccumulatedPresentationValidationError {
      ref mut presentation_validation_errors,
      ref mut credential_errors,
    } = compound_error;

    if let Err(structure_error) = Self::check_structure_local_error(presentation) {
      presentation_validation_errors.push(structure_error);
      observed_error = true;
      if fail_fast {
        return Err(compound_error);
      }
    }
    // collect all nonTransferable violations (if any)
    presentation_validation_errors.extend(
      Self::non_transferable_violations(presentation)
        .map(|credential_position| ValidationError::NonTransferableViolation { credential_position }),
    );
    if !presentation_validation_errors.is_empty() {
      observed_error = true;
      if fail_fast {
        return Err(compound_error);
      }
    }

    if let Err(signature_error) = Self::verify_presentation_signature_local_error(
      presentation,
      resolved_holder_document,
      &options.presentation_verifier_options,
    ) {
      presentation_validation_errors.push(signature_error);
      observed_error = true;
      if fail_fast {
        return Err(compound_error);
      }
    }

    // now validate the credentials
    for (position, credential) in presentation.verifiable_credential.iter().enumerate() {
      if let Err(credential_error) = CredentialValidator::new().full_validation_local_error(
        credential,
        &options.shared_validation_options,
        trusted_issuers,
      ) {
        observed_error = true;
        credential_errors.insert(position, credential_error);
        if fail_fast {
          return Err(compound_error);
        }
      }
    }
    // if fail_fast was false, but we observed an error we return our error
    if observed_error {
      Err(compound_error)
    } else {
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Timestamp;
  use identity_core::common::Url;
  use identity_core::crypto::SignatureOptions;
  use identity_credential::presentation::PresentationBuilder;

  use crate::credential::test_utils;
  use crate::credential::CredentialValidationOptions;

  use super::*;
  #[test]
  fn test_full_validation() {
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
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = test_utils::credential_setup();
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
      .earliest_expiry_date(expires_after)
      .latest_issuance_date(issued_before);
    let presentation_verifier_options =
      VerifierOptions::default().challenge("475a7984-1bb5-4c4c-a56f-822bccd46440".to_owned());
    let presentation_validation_options = PresentationValidationOptions::default()
      .shared_validation_options(credential_validation_options)
      .presentation_verifier_options(presentation_verifier_options);

    let validator = PresentationValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);
    assert!(validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
      )
      .is_ok());
  }

  #[test]
  fn test_full_validation_invalid_holder_signature() {
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
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = test_utils::credential_setup();
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

    // check the validation unit
    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];
    let presentation_verifier_options = VerifierOptions::default().challenge("another challenge".to_owned()); //validate with another challenge

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    // Todo match on the exact error here
    assert!(PresentationValidator::verify_presentation_signature(
      &presentation,
      &resolved_holder_document,
      &presentation_verifier_options
    )
    .is_err());

    // now check that full_validation also fails
    let validator = PresentationValidator::new();
    let issued_before = Timestamp::parse("2030-01-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2021-01-01T00:00:00Z").unwrap();
    let credential_validation_options = CredentialValidationOptions::default()
      .earliest_expiry_date(expires_after)
      .latest_issuance_date(issued_before);

    let presentation_validation_options = PresentationValidationOptions::default()
      .shared_validation_options(credential_validation_options)
      .presentation_verifier_options(presentation_verifier_options);

    let error = match validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
      )
      .unwrap_err()
    {
      Error::UnsuccessfulPresentationValidation(mut err) => err.presentation_validation_errors.pop().unwrap(),
      _ => unreachable!(),
    };

    assert!(matches!(error, ValidationError::HolderProof { .. }));
  }

  #[test]
  fn test_full_validation_invalid_credential() {
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
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = test_utils::credential_setup();
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
      .earliest_expiry_date(expires_after)
      .latest_issuance_date(issued_before);
    let presentation_verifier_options = VerifierOptions::default().challenge("some challenge".to_owned());
    let presentation_validation_options = PresentationValidationOptions::default()
      .shared_validation_options(credential_validation_options)
      .presentation_verifier_options(presentation_verifier_options);

    let validator = PresentationValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    let error = match validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
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
  fn test_non_transferable_property_violation() {
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
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = test_utils::credential_setup();
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

    // check that the check_non_transferable validation unit fails
    // Todo: match on the exact error
    assert!(PresentationValidator::check_non_transferable(&presentation).is_err());

    // check that full_validation also fails with the expected error
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
    let validator = PresentationValidator::new();
    let issued_before = Timestamp::parse("2020-02-02T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2021-01-01T00:00:00Z").unwrap();
    let credential_validation_options = CredentialValidationOptions::default()
      .earliest_expiry_date(expires_after)
      .latest_issuance_date(issued_before);
    let presentation_verifier_options = VerifierOptions::default().challenge("some challenge".to_owned());
    let presentation_validation_options = PresentationValidationOptions::default()
      .shared_validation_options(credential_validation_options)
      .presentation_verifier_options(presentation_verifier_options);

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    let error = match validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
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
  fn test_full_validation_multiple_errors_fail_fast() {
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
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = test_utils::credential_setup();
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
      .earliest_expiry_date(expires_after)
      .latest_issuance_date(issued_before);
    let presentation_verifier_options = VerifierOptions::default().challenge("another challenge".to_owned()); // verify with another challenge
    let presentation_validation_options = PresentationValidationOptions::default()
      .shared_validation_options(credential_validation_options)
      .presentation_verifier_options(presentation_verifier_options);

    let validator = PresentationValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    let (presentation_validation_errors, credential_errors) = match validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
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
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = test_utils::credential_setup();
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
      .earliest_expiry_date(expires_after)
      .latest_issuance_date(issued_before)
      .fail_fast(false);
    let presentation_verifier_options = VerifierOptions::default().challenge("another challenge".to_owned()); // verify with another challenge
    let presentation_validation_options = PresentationValidationOptions::default()
      .shared_validation_options(credential_validation_options)
      .presentation_verifier_options(presentation_verifier_options)
      .fail_fast(false);

    let validator = PresentationValidator::new();

    let trusted_issuers = [
      test_utils::mock_resolved_document(issuer_foo_doc),
      test_utils::mock_resolved_document(issuer_bar_doc),
    ];

    let resolved_holder_document = test_utils::mock_resolved_document(subject_foo_doc);

    let (presentation_validation_errors, credential_errors) = match validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
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
}
