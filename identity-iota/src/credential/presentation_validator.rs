// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use identity_core::common::OneOrMany;
use identity_credential::credential::Credential;
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
  /// Constructs a new [PresentationValidator].
  pub fn new() -> Self {
    Self {}
  }

  /// Validates the semantic structure of the [Presentation].
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
  /// holder.
  ///
  /// The output of this iterator will always be a subset of [Self::holder_not_subject_iter()].
  pub fn non_transferable_violations<U, V>(presentation: &Presentation<U, V>) -> impl Iterator<Item = usize> + '_ {
    Self::holder_not_subject_iter_internal(presentation)
      .filter(|(_, credential)| credential.non_transferable.unwrap_or(false))
      .map(|(position, _)| position)
  }

  /// Validates that the nonTransferable property is met.
  ///
  /// # Errors
  /// Returns at the first credential requiring a nonTransferable property that is not met.
  ///
  /// If one needs to find *all* the nonTransferable violations of this presentation, then see
  /// [Self::non_transferable_violations()].
  ///
  /// # Terminology
  /// This is a *validation unit*
  pub fn check_non_transferable<U, V>(presentation: &Presentation<U, V>) -> Result<()> {
    Self::check_non_transferable_local_error(presentation).map_err(Error::UnsuccessfulValidationUnit)
  }

  fn check_non_transferable_local_error<U, V>(presentation: &Presentation<U, V>) -> ValidationUnitResult {
    if let Some(position) = Self::non_transferable_violations(presentation).next() {
      let err = ValidationError::InvalidHolderSubjectRelationship {
        credential_position: position,
      };
      Err(err)
    } else {
      Ok(())
    }
  }

  /// An iterator over indices corresponding to credentials where the credential subject id does not correspond to the
  /// presentation's holder.
  ///
  /// The output of this iterator is always a superset of [Self::non_transferable_violations()].
  pub fn holder_not_subject_iter<U, V>(presentation: &Presentation<U, V>) -> impl Iterator<Item = usize> + '_ {
    Self::holder_not_subject_iter_internal(presentation).map(|(position, _)| position)
  }

  fn holder_not_subject_iter_internal<U, V>(
    presentation: &Presentation<U, V>,
  ) -> impl Iterator<Item = (usize, &Credential<V>)> + '_ {
    presentation
      .verifiable_credential
      .as_ref()
      .iter()
      .enumerate()
      .filter(|(_, credential)| {
        match &credential.credential_subject {
          OneOrMany::One(ref credential_subject) => credential_subject.id != presentation.holder,
          OneOrMany::Many(subjects) => {
            // need to check the case where the Many variant holds a vector of exactly one subject
            if let &[ref credential_subject] = subjects.as_slice() {
              credential_subject.id != presentation.holder
            } else {
              // zero or > 1 subjects means that the holder is not the subject
              true
            }
          }
        }
      })
  }

  /// Verify the presentation's signature using the resolved document of the holder
  ///
  /// # Errors
  /// Fails if the supplied `holder` cannot be identified with the URL of the `presentation`'s holder
  /// property
  ///
  /// # Terminology
  /// This is a *validation unit*.
  pub fn verify_presentation_signature<U: Serialize, V: Serialize>(
    presentation: &Presentation<U, V>,
    holder: &ResolvedIotaDocument,
    options: &VerifierOptions,
  ) -> Result<()> {
    Self::verify_presentation_signature_local_error(presentation, holder, options)
      .map_err(Error::UnsuccessfulValidationUnit)
  }

  fn verify_presentation_signature_local_error<U: Serialize, V: Serialize>(
    presentation: &Presentation<U, V>,
    holder: &ResolvedIotaDocument,
    options: &VerifierOptions,
  ) -> ValidationUnitResult {
    let did: IotaDID = presentation
      .holder
      .as_ref()
      .ok_or(ValidationError::MissingPresentationHolder)
      .and_then(|value| {
        IotaDID::parse(value.as_str()).map_err(|error| ValidationError::HolderUrl { source: error.into() })
      })?;
    if &did != holder.document.id() {
      return Err(ValidationError::IncompatibleHolderDocument);
    }
    holder
      .document
      .verify_data(&presentation, options)
      .map_err(|error| ValidationError::HolderProof { source: error.into() })
  }

  /// Validate a `Presentation`
  ///
  /// Checks common concerns such as the holder's signature, the nonTransferable property, the semantic structure of the
  /// presentation and common concerns regarding credential validation (see
  /// [`CredentialValidator::full_validation()`]).
  ///
  /// # Errors
  /// Fails if any of the following conditions occur
  /// - The structure of the presentation is not semantically valid
  /// - The nonTransferable property is set in one of the credentials, but the credential's subject is not the holder of
  ///   the presentation.
  /// - The `holder` parameter does not correspond to the holder property of the presentation
  /// - The holder's signature cannot be verified
  /// - Validation of any of the presentation's credentials fails (see [CredentialValidator::full_validation()]).
  // Takes &self in case this method will need some pre-computed state in the future.
  pub fn full_validation<U: Serialize, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    holder: &ResolvedIotaDocument,
    issuers: &[ResolvedIotaDocument],
    fail_fast: bool,
  ) -> Result<()> {
    self
      .full_validation_local_error(presentation, options, holder, issuers, fail_fast)
      .map_err(Error::UnsuccessfulPresentationValidation)
  }

  fn full_validation_local_error<U: Serialize, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    holder: &ResolvedIotaDocument,
    issuers: &[ResolvedIotaDocument],
    fail_fast: bool,
  ) -> PresentationValidationResult {
    // first run the presentation specific validation units.
    // We set up an iterator over these functions and collect any encountered errors
    let structure_validation = std::iter::once_with(|| Self::check_structure_local_error(presentation));
    let signature_validation = std::iter::once_with(|| {
      Self::verify_presentation_signature_local_error(presentation, holder, &options.presentation_verifier_options)
    });

    // how the subject holder relationship is validated depends on the settings
    // if holder_must_be_subject is true then this is what we check for and the presence of the nonTransferable property
    // is irrelevant. Otherwise this check depends on the value of `allow_non_transferable` parameter.
    let number_of_credentials = presentation.verifiable_credential.len();
    let subject_holder_relationship_validation = {
      let (max_holder_not_subject_checks, max_non_transferable_violation_checks) = match (
        options.holder_must_be_subject,
        options.allow_non_transferable_violations,
      ) {
        (true, _) => (number_of_credentials, 0),
        (false, false) => (0, number_of_credentials),
        (false, true) => (0, 0),
      };
      Self::holder_not_subject_iter(presentation)
        .take(max_holder_not_subject_checks)
        .chain(Self::non_transferable_violations(presentation).take(max_non_transferable_violation_checks))
        .map(|credential_position| Err(ValidationError::InvalidHolderSubjectRelationship { credential_position }))
    };

    let presentation_validation_errors_iter = structure_validation
      .chain(subject_holder_relationship_validation)
      .chain(signature_validation)
      .filter_map(|result| result.err());

    let presentation_validation_errors: Vec<ValidationError> = if fail_fast {
      presentation_validation_errors_iter.take(1).collect()
    } else {
      presentation_validation_errors_iter.collect()
    };

    // now run full validations on the credentials and collect any encountered errors
    let credential_errors_iter = presentation
      .verifiable_credential
      .iter()
      .map(|credential| {
        CredentialValidator::new().full_validation_local_error(
          credential,
          &options.shared_validation_options,
          issuers,
          fail_fast,
        )
      })
      .enumerate()
      .filter_map(|(position, result)| result.err().map(|error| (position, error)));

    let credential_errors: BTreeMap<usize, AccumulatedCredentialValidationError> = credential_errors_iter
      .take(if (!presentation_validation_errors.is_empty()) && fail_fast {
        // we already encountered validation errors and we are supposed to fail fast so don't collect any more errors
        0
      } else if fail_fast {
        // we did not encounter any validation errors yet, but we are supposed to fail fast so collect at most one
        // credential error
        1
      } else {
        // we are supposed to collect all errors (if any) so collect all credential errors
        number_of_credentials
      })
      .collect();

    if !presentation_validation_errors.is_empty() || !credential_errors.is_empty() {
      Err(AccumulatedPresentationValidationError {
        presentation_validation_errors,
        credential_errors,
      })
    } else {
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::credential::test_utils;
  use crate::credential::CredentialValidationOptions;
  use crate::document::IotaDocument;
  use identity_core::common::Timestamp;
  use identity_core::common::Url;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::SignatureOptions;
  use identity_credential::presentation::PresentationBuilder;

  use super::*;

  fn build_presentation(holder: &IotaDocument, credentials: Vec<Credential>) -> Presentation {
    let mut builder = PresentationBuilder::default()
      .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs").unwrap())
      .holder(Url::parse(holder.id().as_ref()).unwrap());
    for credential in credentials {
      builder = builder.credential(credential);
    }
    builder.build().unwrap()
  }

  // setup code shared among many of these tests
  struct Setup {
    issuer_foo_doc: IotaDocument,
    issuer_foo_key: KeyPair,
    subject_foo_doc: IotaDocument,
    subject_foo_key: KeyPair,
    credential_foo: Credential,
    issuer_bar_doc: IotaDocument,
    issuer_bar_key: KeyPair,
    credential_bar: Credential,
  }

  impl Setup {
    // creates unsigned data necessary for many of these tests
    fn new() -> Self {
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
      let (issuer_bar_doc, issuer_bar_key, credential_bar) = test_utils::credential_setup();
      // set the nonTransferable option on the first credential
      credential_foo.non_transferable = Some(true);

      Self {
        issuer_foo_doc,
        issuer_foo_key,
        subject_foo_doc,
        subject_foo_key,
        credential_foo,
        issuer_bar_doc,
        issuer_bar_key,
        credential_bar,
      }
    }

    // creates signed data necessary for many of these tests
    fn new_with_signed_credentials() -> Self {
      let mut setup = Self::new();
      let Setup {
        ref mut credential_foo,
        ref issuer_foo_doc,
        ref issuer_foo_key,
        ref issuer_bar_doc,
        ref issuer_bar_key,
        ref mut credential_bar,
        ..
      } = setup;
      // sign the credential
      issuer_foo_doc
        .sign_data(
          credential_foo,
          issuer_foo_key.private(),
          issuer_foo_doc.default_signing_method().unwrap().id(),
          SignatureOptions::default(),
        )
        .unwrap();

      issuer_bar_doc
        .sign_data(
          credential_bar,
          issuer_bar_key.private(),
          issuer_bar_doc.default_signing_method().unwrap().id(),
          SignatureOptions::default(),
        )
        .unwrap();
      setup
    }
  }

  #[test]
  fn test_full_validation() {
    let Setup {
      subject_foo_doc,
      subject_foo_key,
      credential_foo,
      credential_bar,
      issuer_foo_doc,
      issuer_bar_doc,
      ..
    } = Setup::new_with_signed_credentials();

    let mut presentation = build_presentation(&subject_foo_doc, [credential_foo, credential_bar].to_vec());

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
    let fail_fast = true;
    assert!(validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
        fail_fast
      )
      .is_ok());
  }

  #[test]
  fn test_full_validation_invalid_holder_signature() {
    let Setup {
      issuer_foo_doc,
      issuer_bar_doc,
      subject_foo_doc,
      subject_foo_key,
      credential_foo,
      credential_bar,
      ..
    } = Setup::new_with_signed_credentials();

    let mut presentation = build_presentation(&subject_foo_doc, [credential_foo, credential_bar].to_vec());

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

    let fail_fast = true;
    let error = match validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
        fail_fast,
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
    let Setup {
      issuer_foo_doc,
      issuer_bar_doc,
      subject_foo_doc,
      subject_foo_key,
      credential_foo,
      credential_bar,
      ..
    } = Setup::new_with_signed_credentials();

    let mut presentation = build_presentation(&subject_foo_doc, [credential_foo, credential_bar].to_vec());
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
    let fail_fast = true;
    let error = match validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
        fail_fast,
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
    let Setup {
      issuer_foo_doc,
      subject_foo_doc,
      subject_foo_key,
      credential_foo,
      ..
    } = Setup::new_with_signed_credentials();

    // note we only extracted `credential_foo` from the setup routine. This is because we want another `credential_bar`
    // for this test.
    let (issuer_bar_doc, issuer_bar_key, mut credential_bar) = test_utils::credential_setup();
    // set the nonTransferable property on this credential
    credential_bar.non_transferable = Some(true);
    // sign the credential
    issuer_bar_doc
      .sign_data(
        &mut credential_bar,
        issuer_bar_key.private(),
        issuer_bar_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    // create a presentation where the subject of the first credential is the holder.
    // This violates the non transferable property of the second credential.
    let mut presentation = build_presentation(&subject_foo_doc, [credential_foo, credential_bar].to_vec());

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
    let fail_fast = true;

    let error = match validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
        fail_fast,
      )
      .unwrap_err()
    {
      Error::UnsuccessfulPresentationValidation(mut err) => err.presentation_validation_errors.pop().unwrap(),
      _ => unreachable!(),
    };

    assert!(matches!(
      error,
      ValidationError::InvalidHolderSubjectRelationship { credential_position: 1 }
    ));

    // Todo: Consider moving these last two checks out into separate tests.

    // check that the validation passes if we change the options to allow for nonTransferableViolations
    let options = presentation_validation_options.allow_non_transferable_violations(true);
    assert!(validator
      .full_validation(
        &presentation,
        &options,
        &resolved_holder_document,
        &trusted_issuers,
        fail_fast,
      )
      .is_ok());
    // finally check that full_validation now does not pass if we set holder_must_be_subject = true.
    let options = options.holder_must_be_subject(true);

    assert!(validator
      .full_validation(
        &presentation,
        &options,
        &resolved_holder_document,
        &trusted_issuers,
        fail_fast,
      )
      .is_err());
  }

  #[test]
  fn test_full_validation_multiple_errors_fail_fast() {
    // create a first credential
    let Setup {
      issuer_foo_doc,
      issuer_bar_doc,
      subject_foo_doc,
      subject_foo_key,
      credential_bar,
      credential_foo,
      ..
    } = Setup::new_with_signed_credentials();

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
    let fail_fast = true;

    let (presentation_validation_errors, credential_errors) = match validator
      .full_validation(
        &presentation,
        &presentation_validation_options,
        &resolved_holder_document,
        &trusted_issuers,
        fail_fast,
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
    let Setup {
      issuer_foo_doc,
      issuer_bar_doc,
      subject_foo_doc,
      subject_foo_key,
      credential_foo,
      credential_bar,
      ..
    } = Setup::new_with_signed_credentials();
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
    let fail_fast = false;

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
        fail_fast,
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
