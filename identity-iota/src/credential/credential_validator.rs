// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::credential::errors::AccumulatedCredentialValidationError;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::Error;
use crate::Result;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_credential::credential::Credential;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::ValidationError;
use super::CredentialValidationOptions;

/// A struct for validating [Credential]s.
#[non_exhaustive]
pub struct CredentialValidator;

// Use the following pattern throughout: for each public method there is corresponding private method returning a more
// local error.
type ValidationUnitResult = std::result::Result<(), ValidationError>;
type CredentialValidationResult = std::result::Result<(), AccumulatedCredentialValidationError>;

impl Default for CredentialValidator {
  fn default() -> Self {
    Self::new()
  }
}
impl CredentialValidator {
  /// Constructs a new [CredentialValidator]
  pub fn new() -> Self {
    Self {}
  }

  /// Validates the semantic structure of the [Credential].
  pub fn check_structure<T>(credential: &Credential<T>) -> Result<()> {
    Self::check_structure_local_error(credential).map_err(Error::UnsuccessfulValidationUnit)
  }

  fn check_structure_local_error<T>(credential: &Credential<T>) -> ValidationUnitResult {
    credential
      .check_structure()
      .map_err(ValidationError::CredentialStructure)
  }

  /// Validate that the [Credential] does not expire before the specified [Timestamp].
  pub fn check_not_expired_before<T>(credential: &Credential<T>, timestamp: Timestamp) -> Result<()> {
    Self::check_not_expired_before_local_error(credential, timestamp).map_err(Error::UnsuccessfulValidationUnit)
  }

  fn check_not_expired_before_local_error<T>(credential: &Credential<T>, timestamp: Timestamp) -> ValidationUnitResult {
    let is_ok = if let Some(expiration_date) = credential.expiration_date {
      expiration_date >= timestamp
    } else {
      true
    };
    is_ok.then(|| ()).ok_or(ValidationError::ExpirationDate)
  }

  /// Validate that the [Credential] is issued no later than the specified [Timestamp].
  pub fn check_not_issued_after<T>(credential: &Credential<T>, timestamp: Timestamp) -> Result<()> {
    Self::check_not_issued_after_local_error(credential, timestamp).map_err(Error::UnsuccessfulValidationUnit)
  }

  fn check_not_issued_after_local_error<T>(credential: &Credential<T>, timestamp: Timestamp) -> ValidationUnitResult {
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
  ) -> Result<()> {
    Self::verify_signature_local_error(credential, trusted_issuers, options).map_err(Error::UnsuccessfulValidationUnit)
  }

  fn verify_signature_local_error<T: Serialize>(
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
            .ok_or(ValidationError::IncompatibleIssuerDocuments)
        }
        Err(error) => {
          // the issuer's url could not be parsed to a valid IotaDID
          Err(ValidationError::IssuerUrl { source: error.into() })
        }
      }
    };
    // use the extracted document to verify the signature
    extracted_issuer_result.and_then(|issuer| {
      issuer
        .document
        .verify_data(credential, options)
        .map_err(|error| ValidationError::IssuerProof { source: error.into() })
    })
  }

  /// Validates a [Credential].
  ///
  /// Common concerns are checked such as the credential's signature, expiration date, issuance date and semantic
  /// structure.
  ///
  ///
  /// # Errors
  /// Fails if any of the following conditions occur
  /// - The structure of the credential is not semantically valid
  /// - The expiration date does not meet the requirement set in `options`
  /// - The issuance date does not meet the requirement set in `options`
  /// - The `issuer` parameter does not provide a DID corresponding to the URL of the credential's issuer.
  /// - The credential's signature cannot be verified using the issuer's DID Document
  ///
  /// Fails on the first encountered error if `fail_fast` is true, otherwise all
  /// errors will be accumulated in the returned error.
  // Takes &self in case this method will need some pre-computed state in the future.
  pub fn validate<T: Serialize>(
    &self,
    credential: &Credential<T>,
    options: &CredentialValidationOptions,
    issuer: &ResolvedIotaDocument,
    fail_fast: bool,
  ) -> Result<()> {
    self
      .full_validation_local_error(credential, options, std::slice::from_ref(issuer), fail_fast)
      .map_err(Error::UnsuccessfulCredentialValidation)
  }

  // This method takes a slice of issuer's instead of a single issuer in order to better accommodate presentation
  // validation.
  pub(super) fn full_validation_local_error<T: Serialize>(
    &self,
    credential: &Credential<T>,
    options: &CredentialValidationOptions,
    issuers: &[ResolvedIotaDocument],
    fail_fast: bool,
  ) -> CredentialValidationResult {
    let signature_validation =
      std::iter::once_with(|| Self::verify_signature_local_error(credential, issuers, &options.verifier_options));

    let expiry_date_validation =
      std::iter::once_with(|| Self::check_not_expired_before_local_error(credential, options.earliest_expiry_date));

    let issuance_date_validation =
      std::iter::once_with(|| Self::check_not_issued_after_local_error(credential, options.latest_issuance_date));

    let structure_validation = std::iter::once_with(|| Self::check_structure_local_error(credential));

    let validation_units_error_iter = issuance_date_validation
      .chain(expiry_date_validation)
      .chain(structure_validation)
      .chain(signature_validation)
      .filter_map(|result| result.err());
    let validation_errors: OneOrMany<ValidationError> = if fail_fast {
      validation_units_error_iter.take(1).collect()
    } else {
      validation_units_error_iter.collect()
    };
    if validation_errors.is_empty() {
      Ok(())
    } else {
      Err(AccumulatedCredentialValidationError { validation_errors })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::proptest;

  use identity_core::common::Timestamp;

  use identity_core::convert::FromJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::SignatureOptions;

  use crate::credential::test_utils;
  use crate::credential::CredentialValidationOptions;
  use crate::document::IotaDocument;

  fn deserialize_credential(credential_str: &str) -> Credential {
    Credential::from_json(credential_str).unwrap()
  }

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
  fn simple_expires_after_with_expiration_date() {
    let credential = deserialize_credential(SIMPLE_CREDENTIAL_JSON);
    let expected_expiration_date = Timestamp::parse("2020-01-01T19:23:24Z").unwrap();
    // check that this credential has the expected expiration date
    assert_eq!(
      credential.expiration_date.unwrap(),
      expected_expiration_date,
      "the expiration date of the parsed credential does not match our expectation"
    );
    // now that we are sure that our parsed credential has the expected expiration date set we can start testing the
    // expires_after method with a later date
    let later_date = Timestamp::parse("2020-02-01T15:10:21Z").unwrap();
    assert!(CredentialValidator::check_not_expired_before(&credential, later_date).is_err());
    // and now with an earlier date
    let earlier_date = Timestamp::parse("2019-12-27T11:35:30Z").unwrap();
    assert!(CredentialValidator::check_not_expired_before(&credential, earlier_date).is_ok());
  }

  // test with a few timestamps that should be RFC3339 compatible
  proptest! {
    #[test]
    fn property_based_expires_after_with_expiration_date(seconds in 0..1_000_000_000_i64) {
      let credential = deserialize_credential(SIMPLE_CREDENTIAL_JSON);
      let expected_expiration_date = Timestamp::parse("2020-01-01T19:23:24Z").unwrap();
      // check that this credential has the expected expiration date
      assert_eq!(credential.expiration_date.unwrap(), expected_expiration_date, "the expiration date of the parsed credential does not match our expectation");
      let after_expiration_date = Timestamp::from_unix(expected_expiration_date.to_unix() + seconds).unwrap();
      let before_expiration_date = Timestamp::from_unix(expected_expiration_date.to_unix() - seconds).unwrap();
      assert!(CredentialValidator::check_not_expired_before(&credential, after_expiration_date).is_err());
      assert!(CredentialValidator::check_not_expired_before(&credential, before_expiration_date).is_ok());
    }
  }

  proptest! {
    #[test]
    fn property_based_expires_after_no_expiration_date(seconds in FIRST_RFC3999_COMPATIBLE_UNIX_TIMESTAMP..LAST_RFC3339_COMPATIBLE_UNIX_TIMESTAMP) {
      let mut credential = deserialize_credential(SIMPLE_CREDENTIAL_JSON);
      credential.expiration_date = None;
      // expires after whatever the timestamp may be because the expires_after field is None.
      assert!(CredentialValidator::check_not_expired_before(&credential, Timestamp::from_unix(seconds).unwrap()).is_ok());
    }
  }

  #[test]
  fn test_full_validation_invalid_expiration_date() {
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
    let issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2023-02-01T00:00:00Z").unwrap(); // note that expires_after > expiration_date
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let fail_fast = true;
    let error = match validator
      .validate(&credential, &options, &issuer, fail_fast)
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
  fn simple_issued_before() {
    let credential = deserialize_credential(SIMPLE_CREDENTIAL_JSON);
    let expected_issuance_date = Timestamp::parse("2010-01-01T19:23:24Z").unwrap();
    // check that this credential has the expected issuance date
    assert_eq!(
      credential.issuance_date, expected_issuance_date,
      "the issuance date of the parsed credential does not match our expectation"
    );
    // now that we are sure that our parsed credential has the expected issuance date set we can start testing issued
    // before with an earlier timestamp
    assert!(CredentialValidator::check_not_issued_after(
      &credential,
      Timestamp::parse("2010-01-01T19:22:09Z").unwrap()
    )
    .is_err());
    // and now with a later timestamp
    assert!(CredentialValidator::check_not_issued_after(
      &credential,
      Timestamp::parse("2010-01-01T20:00:00Z").unwrap()
    )
    .is_ok());
  }

  proptest! {
    #[test]
    fn property_based_issued_before(seconds in 0 ..1_000_000_000_i64) {
      let credential = deserialize_credential(SIMPLE_CREDENTIAL_JSON);
      let expected_issuance_date = Timestamp::parse("2010-01-01T19:23:24Z").unwrap();
      // check that this credential has the expected issuance date
      assert_eq!(credential.issuance_date, expected_issuance_date, "the issuance date of the parsed credential does not match our expectation");
      let earlier_than_issuance_date = Timestamp::from_unix(expected_issuance_date.to_unix() - seconds).unwrap();
      let later_than_issuance_date = Timestamp::from_unix(expected_issuance_date.to_unix() + seconds).unwrap();
      assert!(CredentialValidator::check_not_issued_after(&credential, earlier_than_issuance_date).is_err());
      assert!(CredentialValidator::check_not_issued_after(&credential, later_than_issuance_date).is_ok());
    }
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
    let issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2019-02-01T00:00:00Z").unwrap(); // note that issued_before < issuance_date
    let expires_after = Timestamp::parse("2022-02-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);

    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let fail_fast = true;
    let error = match validator
      .validate(&credential, &options, &issuer, fail_fast)
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
  fn test_full_validation() {
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
    let issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-12-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    let validator = CredentialValidator::new();
    let fail_fast = true;
    assert!(validator.validate(&credential, &options, &issuer, fail_fast).is_ok());
  }

  #[test]
  fn test_matches_issuer_did_unrelated_issuer() {
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

    // check that the validation unit returns the expected error
    let issuer = test_utils::mock_resolved_document(other_doc); // the credential was not signed by this issuer

    assert!(matches!(
      CredentialValidator::verify_signature(&credential, std::slice::from_ref(&issuer), &VerifierOptions::default())
        .unwrap_err(),
      Error::UnsuccessfulValidationUnit(ValidationError::IncompatibleIssuerDocuments)
    ));

    // also check that the full validation fails as expected
    let validator = CredentialValidator::new();
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-12-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);

    // validate and extract the nested error according to our expectations
    let fail_fast = true;
    let error = match validator
      .validate(&credential, &options, &issuer, fail_fast)
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

    assert!(matches!(error, ValidationError::IncompatibleIssuerDocuments));
  }

  #[test]
  fn test_verify_invalid_signature() {
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
    let issuer = test_utils::mock_resolved_document(issuer_doc);

    // run the validation unit
    assert!(matches!(
      CredentialValidator::verify_signature(&credential, std::slice::from_ref(&issuer), &VerifierOptions::default())
        .unwrap_err(),
      Error::UnsuccessfulValidationUnit(ValidationError::IssuerProof { .. })
    ));

    // check that full_validation also fails as expected
    let validator = CredentialValidator::new();
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-12-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    // validate and extract the nested error according to our expectations
    let fail_fast = true;
    let error = match validator
      .validate(&credential, &options, &issuer, fail_fast)
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
  fn test_full_validation_invalid_structure() {
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
    let issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-02-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let fail_fast = true;
    let error = match validator
      .validate(&credential, &options, &issuer, fail_fast)
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
  fn test_full_validation_multiple_errors_fail_fast() {
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
    let other_issuer_resolved_doc = test_utils::mock_resolved_document(other_issuer); // the credential was not issued by `other_issuer`
    let issued_before = Timestamp::parse("2019-02-01T00:00:00Z").unwrap(); // issued_before < issuance_date
    let expires_after = Timestamp::parse("2024-02-01T00:00:00Z").unwrap(); // expires_after > expiration_date
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    // fail_fast = true is default.
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let fail_fast = true;
    let error = match validator
      .validate(&credential, &options, &other_issuer_resolved_doc, fail_fast)
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
  fn test_full_validation_multiple_errors_accumulate_all_errors() {
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
    let other_issuer_resolved_doc = test_utils::mock_resolved_document(other_issuer); // other_issuer did not issue the credential [second error]
    let issued_before = Timestamp::parse("2019-02-01T00:00:00Z").unwrap(); // issued_before < issuance_date [third error]
    let expires_after = Timestamp::parse("2024-02-01T00:00:00Z").unwrap(); // expires_after > expiration_date [fourth error]
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    let validator = CredentialValidator::new();
    // validate and extract the nested error according to our expectations
    let fail_fast = false;
    let error = match validator
      .validate(&credential, &options, &other_issuer_resolved_doc, fail_fast)
      .unwrap_err()
    {
      Error::UnsuccessfulCredentialValidation(accumulated_validation_error) => {
        accumulated_validation_error.validation_errors
      }
      _ => unreachable!(),
    };

    assert!(error.len() >= 4);
  }
}
