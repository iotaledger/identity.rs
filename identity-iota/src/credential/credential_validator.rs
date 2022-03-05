// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::credential::errors::CompoundCredentialValidationError;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;

use crate::Result;
use identity_core::common::Timestamp;
use identity_credential::credential::Credential;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::SignerContext;
use super::errors::ValidationError;
use super::CredentialValidationOptions;
use super::FailFast;

/// A struct for validating [`Credential`]s.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CredentialValidator;

type ValidationUnitResult = std::result::Result<(), ValidationError>;
type CredentialValidationResult = std::result::Result<(), CompoundCredentialValidationError>;

impl CredentialValidator {
  /// Validates a [`Credential`].
  ///
  /// The following properties are validated according to `options`:
  /// - The issuer's signature,
  /// - The expiration date,
  /// - The issuance date
  /// - The semantic structure.
  ///
  /// # Warning
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
  /// `credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
  /// These should be manually checked after validation, according to your requirements.
  ///
  /// # Errors
  /// An error is returned whenever a validated condition is not satisfied.
  pub fn validate<T: Serialize>(
    credential: &Credential<T>,
    issuer: &ResolvedIotaDocument,
    options: &CredentialValidationOptions,
    fail_fast: FailFast,
  ) -> CredentialValidationResult {
    Self::validate_with_trusted_issuers(credential, std::slice::from_ref(issuer), options, fail_fast)
  }

  /// Validates the semantic structure of the [`Credential`].
  ///
  /// # Warning
  /// This does not validate against the credential's schema nor the structure of the subject claims.
  pub fn check_structure<T>(credential: &Credential<T>) -> ValidationUnitResult {
    credential
      .check_structure()
      .map_err(ValidationError::CredentialStructure)
  }

  /// Validate that the [`Credential`] expires on or after the specified [`Timestamp`].
  pub fn check_expires_on_or_after<T>(credential: &Credential<T>, timestamp: Timestamp) -> ValidationUnitResult {
    let is_ok = if let Some(expiration_date) = credential.expiration_date {
      expiration_date >= timestamp
    } else {
      true
    };
    is_ok.then(|| ()).ok_or(ValidationError::ExpirationDate)
  }

  /// Validate that the [`Credential`] is issued on or before the specified [`Timestamp`].
  pub fn check_issued_on_or_before<T>(credential: &Credential<T>, timestamp: Timestamp) -> ValidationUnitResult {
    (credential.issuance_date <= timestamp)
      .then(|| ())
      .ok_or(ValidationError::IssuanceDate)
  }

  /// Verify the signature using the DID Document of a trusted issuer.
  ///
  /// # Errors
  /// This method immediately returns an error if
  /// the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
  /// to verify the credential's signature will be made and an error is returned upon failure.
  pub fn verify_signature<T: Serialize>(
    credential: &Credential<T>,
    trusted_issuers: &[ResolvedIotaDocument],
    options: &VerifierOptions,
  ) -> ValidationUnitResult {
    // try to extract the corresponding issuer from `trusted_issuers`
    let extracted_issuer_result: std::result::Result<&ResolvedIotaDocument, ValidationError> = {
      let issuer_did: Result<IotaDID> = credential.issuer.url().as_str().parse();
      match issuer_did {
        Ok(did) => {
          // if the issuer_did corresponds to one of the trusted issuers we use the corresponding DID Document to verify
          // the signature
          trusted_issuers
            .iter()
            .find(|issuer_doc| issuer_doc.document.id() == &did)
            .ok_or(ValidationError::DocumentMismatch(SignerContext::Issuer))
        }
        Err(error) => {
          // the issuer's url could not be parsed to a valid IotaDID
          Err(ValidationError::SignerUrl {
            source: error.into(),
            signer_ctx: SignerContext::Issuer,
          })
        }
      }
    };
    // use the extracted document to verify the signature
    extracted_issuer_result.and_then(|issuer| {
      issuer
        .document
        .verify_data(credential, options)
        .map_err(|error| ValidationError::Signature {
          source: error.into(),
          signer_ctx: SignerContext::Issuer,
        })
    })
  }

  // This method takes a slice of issuer's instead of a single issuer in order to better accommodate presentation
  // validation.
  pub(crate) fn validate_with_trusted_issuers<T: Serialize>(
    credential: &Credential<T>,
    issuers: &[ResolvedIotaDocument],
    options: &CredentialValidationOptions,
    fail_fast: FailFast,
  ) -> CredentialValidationResult {
    // Run all single concern validations in turn and fail immediately if `fail_fast` is true.
    let signature_validation =
      std::iter::once_with(|| Self::verify_signature(credential, issuers, &options.verifier_options));

    let expiry_date_validation = std::iter::once_with(|| {
      Self::check_expires_on_or_after(credential, options.earliest_expiry_date.unwrap_or_default())
    });

    let issuance_date_validation = std::iter::once_with(|| {
      Self::check_issued_on_or_before(credential, options.latest_issuance_date.unwrap_or_default())
    });

    let structure_validation = std::iter::once_with(|| Self::check_structure(credential));

    let validation_units_error_iter = issuance_date_validation
      .chain(expiry_date_validation)
      .chain(structure_validation)
      .chain(signature_validation)
      .filter_map(|result| result.err());
    let validation_errors: Vec<ValidationError> = match fail_fast {
      FailFast::FirstError => validation_units_error_iter.take(1).collect(),
      FailFast::AllErrors => validation_units_error_iter.collect(),
    };

    if validation_errors.is_empty() {
      Ok(())
    } else {
      Err(CompoundCredentialValidationError { validation_errors })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use identity_core::common::Duration;
  use identity_core::common::Object;
  use identity_core::common::OneOrMany;
  use proptest::proptest;

  use identity_core::common::Timestamp;

  use identity_core::convert::FromJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::SignatureOptions;

  use crate::credential::test_utils;
  use crate::credential::CredentialValidationOptions;
  use crate::document::IotaDocument;

  const LAST_RFC3339_COMPATIBLE_UNIX_TIMESTAMP: i64 = 253402300799; // 9999-12-31T23:59:59Z
  const FIRST_RFC3999_COMPATIBLE_UNIX_TIMESTAMP: i64 = -62167219200; // 0000-01-01T00:00:00Z

  const SIMPLE_CREDENTIAL_JSON: &str = r#"{
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "http://example.edu/credentials/3732",
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "https://example.edu/issuers/14",
    "issuanceDate": "2010-01-01T19:23:24Z",
    "expirationDate": "2020-01-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science in Mechanical Engineering"
      }
    }
  }"#;

  lazy_static::lazy_static! {
    // A simple credential shared by some of the tests in this module
    static ref SIMPLE_CREDENTIAL: Credential = Credential::<Object>::from_json(SIMPLE_CREDENTIAL_JSON).unwrap();
  }

  // Setup parameters shared by many of the tests in this module
  struct Setup {
    issuer_doc: IotaDocument,
    issuer_key: KeyPair,
    unsigned_credential: Credential,
    issuance_date: Timestamp,
    expiration_date: Timestamp,
  }
  impl Setup {
    fn new() -> Self {
      let (issuer_doc, issuer_key) = test_utils::generate_document_with_keys();
      let (subject_doc, _) = test_utils::generate_document_with_keys();
      let issuance_date = Timestamp::parse("2020-01-01T00:00:00Z").unwrap();
      let expiration_date = Timestamp::parse("2023-01-01T00:00:00Z").unwrap();
      let unsigned_credential =
        test_utils::generate_credential(&issuer_doc, &[subject_doc], issuance_date, expiration_date);
      Self {
        issuer_doc,
        issuer_key,
        unsigned_credential,
        issuance_date,
        expiration_date,
      }
    }
  }

  #[test]
  fn simple_expires_on_or_after_with_expiration_date() {
    let later_than_expiration_date = SIMPLE_CREDENTIAL
      .expiration_date
      .unwrap()
      .checked_add(Duration::minutes(1))
      .unwrap();
    assert!(CredentialValidator::check_expires_on_or_after(&SIMPLE_CREDENTIAL, later_than_expiration_date).is_err());
    // and now with an earlier date
    let earlier_date = Timestamp::parse("2019-12-27T11:35:30Z").unwrap();
    assert!(CredentialValidator::check_expires_on_or_after(&SIMPLE_CREDENTIAL, earlier_date).is_ok());
  }

  // test with a few timestamps that should be RFC3339 compatible
  proptest! {
    #[test]
    fn property_based_expires_after_with_expiration_date(seconds in 0..1_000_000_000_u32) {
      let after_expiration_date = SIMPLE_CREDENTIAL.expiration_date.unwrap().checked_add(Duration::seconds(seconds)).unwrap();
      let before_expiration_date = SIMPLE_CREDENTIAL.expiration_date.unwrap().checked_sub(Duration::seconds(seconds)).unwrap();
      assert!(CredentialValidator::check_expires_on_or_after(&SIMPLE_CREDENTIAL, after_expiration_date).is_err());
      assert!(CredentialValidator::check_expires_on_or_after(&SIMPLE_CREDENTIAL, before_expiration_date).is_ok());
    }
  }

  proptest! {
    #[test]
    fn property_based_expires_after_no_expiration_date(seconds in FIRST_RFC3999_COMPATIBLE_UNIX_TIMESTAMP..LAST_RFC3339_COMPATIBLE_UNIX_TIMESTAMP) {
      let mut credential = SIMPLE_CREDENTIAL.clone();
      credential.expiration_date = None;
      // expires after whatever the timestamp may be because the expires_after field is None.
      assert!(CredentialValidator::check_expires_on_or_after(&credential, Timestamp::from_unix(seconds).unwrap()).is_ok());
    }
  }

  #[test]
  fn test_full_validation_invalid_expiration_date() {
    let Setup {
      issuer_doc,
      issuer_key,
      unsigned_credential: mut credential,
      expiration_date,
      issuance_date,
    } = Setup::new();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    // declare the credential validation parameters
    let issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_on_or_before = issuance_date;
    // expires_on_or_after > expiration_date
    let expires_on_or_after = expiration_date.checked_add(Duration::seconds(1)).unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);
    // validate and extract the nested error according to our expectations

    let validation_errors = CredentialValidator::validate(&credential, &issuer, &options, FailFast::FirstError)
      .unwrap_err()
      .validation_errors;

    let error = match validation_errors.as_slice() {
      [validation_error] => validation_error,
      _ => unreachable!(),
    };

    assert!(matches!(error, &ValidationError::ExpirationDate));
  }

  #[test]
  fn simple_issued_on_or_before() {
    assert!(CredentialValidator::check_issued_on_or_before(
      &SIMPLE_CREDENTIAL,
      SIMPLE_CREDENTIAL
        .issuance_date
        .checked_sub(Duration::minutes(1))
        .unwrap()
    )
    .is_err());
    // and now with a later timestamp
    assert!(CredentialValidator::check_issued_on_or_before(
      &SIMPLE_CREDENTIAL,
      SIMPLE_CREDENTIAL
        .issuance_date
        .checked_add(Duration::minutes(1))
        .unwrap()
    )
    .is_ok());
  }

  proptest! {
    #[test]
    fn property_based_issued_before(seconds in 0 ..1_000_000_000_u32) {

      let earlier_than_issuance_date = SIMPLE_CREDENTIAL.issuance_date.checked_sub(Duration::seconds(seconds)).unwrap();
      let later_than_issuance_date = SIMPLE_CREDENTIAL.issuance_date.checked_add(Duration::seconds(seconds)).unwrap();
      assert!(CredentialValidator::check_issued_on_or_before(&SIMPLE_CREDENTIAL, earlier_than_issuance_date).is_err());
      assert!(CredentialValidator::check_issued_on_or_before(&SIMPLE_CREDENTIAL, later_than_issuance_date).is_ok());
    }
  }

  #[test]
  fn test_validate_credential_invalid_issuance_date() {
    let Setup {
      issuer_doc,
      issuer_key,
      unsigned_credential: mut credential,
      expiration_date,
      issuance_date,
    } = Setup::new();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    // declare the credential validation parameters
    let issuer = test_utils::mock_resolved_document(issuer_doc);
    // issued_on_or_before < issuance_date
    let issued_on_or_before = issuance_date.checked_sub(Duration::seconds(1)).unwrap();
    let expires_on_or_after = expiration_date;
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);

    // validate and extract the nested error according to our expectations
    let validation_errors = CredentialValidator::validate(&credential, &issuer, &options, FailFast::FirstError)
      .unwrap_err()
      .validation_errors;

    let error = match validation_errors.as_slice() {
      [validation_error] => validation_error,
      _ => unreachable!(),
    };

    assert!(matches!(error, &ValidationError::IssuanceDate));
  }

  #[test]
  fn test_full_validation() {
    let Setup {
      issuer_doc,
      issuer_key,
      unsigned_credential: mut credential,
      issuance_date,
      expiration_date,
    } = Setup::new();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // declare the credential validation parameters
    let issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_on_or_before = issuance_date.checked_add(Duration::days(14)).unwrap();
    let expires_on_or_after = expiration_date.checked_sub(Duration::hours(1)).unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);
    assert!(CredentialValidator::validate(&credential, &issuer, &options, FailFast::FirstError).is_ok());
  }

  #[test]
  fn test_matches_issuer_did_unrelated_issuer() {
    let Setup {
      issuer_doc,
      issuer_key,
      unsigned_credential: mut credential,
      issuance_date,
      expiration_date,
    } = Setup::new();
    let (other_doc, _) = test_utils::generate_document_with_keys();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();

    // the credential was not signed by this issuer
    let issuer = test_utils::mock_resolved_document(other_doc);

    // check that `verify_signature` returns the expected error
    assert!(matches!(
      CredentialValidator::verify_signature(&credential, std::slice::from_ref(&issuer), &VerifierOptions::default())
        .unwrap_err(),
      ValidationError::DocumentMismatch { .. }
    ));

    // also check that the full validation fails as expected
    let issued_on_or_before = issuance_date.checked_add(Duration::days(14)).unwrap();
    let expires_on_or_after = expiration_date.checked_sub(Duration::hours(1)).unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);

    // validate and extract the nested error according to our expectations
    let validation_errors = CredentialValidator::validate(&credential, &issuer, &options, FailFast::FirstError)
      .unwrap_err()
      .validation_errors;

    let error = match validation_errors.as_slice() {
      [validation_error] => validation_error,
      _ => unreachable!(),
    };

    assert!(matches!(error, ValidationError::DocumentMismatch { .. }));
  }

  #[test]
  fn test_verify_invalid_signature() {
    let Setup {
      issuer_doc,
      unsigned_credential: mut credential,
      issuance_date,
      expiration_date,
      ..
    } = Setup::new();

    let (_, other_keys) = test_utils::generate_document_with_keys();
    issuer_doc
      .sign_data(
        &mut credential,
        other_keys.private(), // sign with other keys
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    let issuer = test_utils::mock_resolved_document(issuer_doc);

    // run the validation unit
    assert!(matches!(
      CredentialValidator::verify_signature(&credential, std::slice::from_ref(&issuer), &VerifierOptions::default())
        .unwrap_err(),
      ValidationError::Signature { .. }
    ));

    // check that full_validation also fails as expected
    let issued_on_or_before = issuance_date.checked_add(Duration::days(14)).unwrap();
    let expires_on_or_after = expiration_date.checked_sub(Duration::hours(1)).unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);
    // validate and extract the nested error according to our expectations
    let validation_errors = CredentialValidator::validate(&credential, &issuer, &options, FailFast::FirstError)
      .unwrap_err()
      .validation_errors;

    let error = match validation_errors.as_slice() {
      [validation_error] => validation_error,
      _ => unreachable!(),
    };

    assert!(matches!(error, &ValidationError::Signature { .. }));
  }

  #[test]
  fn test_full_validation_invalid_structure() {
    let Setup {
      issuer_doc,
      issuer_key,
      unsigned_credential: mut credential,
      issuance_date,
      expiration_date,
    } = Setup::new();

    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // the credential now has no credential subjects which is not semantically correct
    credential.credential_subject = OneOrMany::default();

    // declare the credential validation parameters
    let issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_on_or_before = issuance_date.checked_add(Duration::days(14)).unwrap();
    let expires_on_or_after = expiration_date.checked_sub(Duration::hours(1)).unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);
    // validate and extract the nested error according to our expectations
    let validation_errors = CredentialValidator::validate(&credential, &issuer, &options, FailFast::FirstError)
      .unwrap_err()
      .validation_errors;

    let error = match validation_errors.as_slice() {
      [validation_error] => validation_error,
      _ => unreachable!(),
    };

    assert!(matches!(error, &ValidationError::CredentialStructure(_)));
  }

  #[test]
  fn test_full_validation_multiple_errors_fail_fast() {
    let Setup {
      issuer_doc,
      issuer_key,
      unsigned_credential: mut credential,
      issuance_date,
      expiration_date,
    } = Setup::new();

    let (other_issuer, _) = test_utils::generate_document_with_keys();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // the credential now has no credential subjects which is not semantically correct
    credential.credential_subject = OneOrMany::default();

    // declare the credential validation parameters
    // the credential was not issued by `other_issuer`
    let other_issuer_resolved_doc = test_utils::mock_resolved_document(other_issuer);
    // issued_on_or_before < issuance_date
    let issued_on_or_before = issuance_date.checked_sub(Duration::seconds(1)).unwrap();

    // expires_on_or_after > expiration_date
    let expires_on_or_after = expiration_date.checked_add(Duration::seconds(1)).unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);
    // validate and extract the nested error according to our expectations
    let validation_errors =
      CredentialValidator::validate(&credential, &other_issuer_resolved_doc, &options, FailFast::FirstError)
        .unwrap_err()
        .validation_errors;

    assert!(validation_errors.len() == 1);
  }

  #[test]
  fn test_full_validation_multiple_errors_accumulate_all_errors() {
    let Setup {
      issuer_doc,
      issuer_key,
      unsigned_credential: mut credential,
      issuance_date,
      expiration_date,
    } = Setup::new();

    let (other_issuer, _) = test_utils::generate_document_with_keys();
    issuer_doc
      .sign_data(
        &mut credential,
        issuer_key.private(),
        issuer_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    // the credential now has no credential subjects which is not semantically correct
    credential.credential_subject = OneOrMany::default();

    // declare the credential validation parameters
    // the credential was not issued by `other_issuer`
    let other_issuer_resolved_doc = test_utils::mock_resolved_document(other_issuer);
    // issued_on_or_before < issuance_date
    let issued_on_or_before = issuance_date.checked_sub(Duration::seconds(1)).unwrap();

    // expires_on_or_after > expiration_date
    let expires_on_or_after = expiration_date.checked_add(Duration::seconds(1)).unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);

    // validate and extract the nested error according to our expectations
    let validation_errors =
      CredentialValidator::validate(&credential, &other_issuer_resolved_doc, &options, FailFast::AllErrors)
        .unwrap_err()
        .validation_errors;

    assert!(validation_errors.len() >= 4);
  }
}
