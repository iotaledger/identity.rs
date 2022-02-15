// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::ControlFlow;

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

/// A struct for validating the given [Credential].
pub struct CredentialValidator<U, T: Borrow<Credential<U>>> {
  pub credential: T,
  _required: PhantomData<U>, // will not compile without this field
}

// Use the following pattern throughout: for each public method there is corresponding private method returning a more
// local error.
type ValidationUnitResult = std::result::Result<(), ValidationError>;
type CredentialValidationResult = std::result::Result<(), AccumulatedCredentialValidationError>;

impl<U, T: Borrow<Credential<U>>> CredentialValidator<U, T> {
  pub fn new(credential: T) -> Self {
    Self {
      credential,
      _required: PhantomData::<U>,
    }
  }

  //---------------------------------------------------------- validation units
  //---------------------------------------------------------- -------------------------------------------------------------------------

  /// Validates the semantic structure of the `Credential`.
  ///
  /// # Terminology
  /// This is a *validation unit*
  pub fn check_structure(&self) -> Result<()> {
    self
      .check_structure_local_error()
      .map_err(Error::UnsuccessfulValidationUnit)
  }

  fn check_structure_local_error(&self) -> ValidationUnitResult {
    self
      .credential
      .borrow()
      .check_structure()
      .map_err(ValidationError::CredentialStructure)
  }

  /// Validate that the [ResolvedCredential] does not expire before the specified [Timestamp].
  ///
  /// # Terminology
  /// This is a *validation unit*
  pub fn earliest_expiry_date(&self, timestamp: Timestamp) -> Result<()> {
    self
      .earliest_expiry_date_local_error(timestamp)
      .map_err(Error::UnsuccessfulValidationUnit)
  }

  fn earliest_expiry_date_local_error(&self, timestamp: Timestamp) -> ValidationUnitResult {
    let is_ok = if let Some(expiration_date) = self.credential.borrow().expiration_date {
      expiration_date >= timestamp
    } else {
      true
    };
    is_ok.then(|| ()).ok_or(ValidationError::ExpirationDate)
  }

  /// Validate that the [ResolvedCredential] is issued no later than the specified [Timestamp].
  ///
  /// # Terminology
  /// This is a *validation unit*
  pub fn latest_issuance_date(&self, timestamp: Timestamp) -> Result<()> {
    self
      .latest_issuance_date_local_error(timestamp)
      .map_err(Error::UnsuccessfulValidationUnit)
  }

  fn latest_issuance_date_local_error(&self, timestamp: Timestamp) -> ValidationUnitResult {
    (self.credential.borrow().issuance_date <= timestamp)
      .then(|| ())
      .ok_or(ValidationError::IssuanceDate)
  }
}

impl<U: Serialize, T: Borrow<Credential<U>>> CredentialValidator<U, T> {
  /// Verify the signature using the issuer's DID document.
  ///
  /// # Security
  /// The caller must assure that all DID Documents passed in as `trusted_issuers` are up to date.
  ///
  /// # Terminology
  /// This method is a *validation unit*
  pub fn verify_signature(&self, trusted_issuers: &[ResolvedIotaDocument], options: &VerifierOptions) -> Result<()> {
    self
      .verify_signature_local_error(trusted_issuers, options)
      .map_err(Error::UnsuccessfulValidationUnit)
  }

  fn verify_signature_local_error(
    &self,
    trusted_issuers: &[ResolvedIotaDocument],
    options: &VerifierOptions,
  ) -> ValidationUnitResult {
    let credential = self.credential.borrow();
    let issuer_did: Result<IotaDID> = credential.issuer.url().as_str().parse();
    match issuer_did {
      Ok(did) => {
        // if the issuer_did corresponds to one of the trusted issuers we use the corresponding DID Document to verify
        // the signature
        if let Err(issuer_proof_error) = trusted_issuers
          .iter()
          .find(|issuer_doc| issuer_doc.document.id() == &did)
          .ok_or(ValidationError::UntrustedIssuer)
          .and_then(|trusted_issuer_doc| {
            trusted_issuer_doc
              .document
              .verify_data(credential, options)
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
        Err(ValidationError::IssuerUrl { source: error.into() })
      }
    }
  }

  /// Validates the `Credential`.
  ///
  /// Common concerns are checked such as the credential's signature, expiration date, issuance date and semantic
  /// structure.
  ///
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
  pub fn full_validation(
    &self,
    options: &CredentialValidationOptions,
    trusted_issuers: &[ResolvedIotaDocument],
  ) -> Result<()> {
    self
      .full_validation_local_error(options, trusted_issuers)
      .map_err(Error::UnsuccessfulCredentialValidation)
  }

  pub(super) fn full_validation_local_error(
    &self,
    options: &CredentialValidationOptions,
    trusted_issuers: &[ResolvedIotaDocument],
  ) -> CredentialValidationResult {
    let signature_validation =
      std::iter::once_with(|| self.verify_signature_local_error(trusted_issuers, &options.verifier_options));

    let expiry_date_validation =
      std::iter::once_with(|| self.earliest_expiry_date_local_error(options.earliest_expiry_date));

    let issuance_date_validation =
      std::iter::once_with(|| self.latest_issuance_date_local_error(options.latest_issuance_date));

    let structure_validation = std::iter::once_with(|| self.check_structure_local_error());

    let mut validation_errors = OneOrMany::empty();
    issuance_date_validation
      .chain(expiry_date_validation)
      .chain(structure_validation)
      .chain(signature_validation)
      .try_for_each(|res| {
        if let Err(error) = res {
          validation_errors.push(error);
          if options.fail_fast {
            return ControlFlow::Break(());
          }
          ControlFlow::Continue(())
        } else {
          ControlFlow::Continue(())
        }
      });
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
    let validator = CredentialValidator::new(credential);
    assert!(validator.earliest_expiry_date(later_date).is_err());
    // and now with an earlier date
    let earlier_date = Timestamp::parse("2019-12-27T11:35:30Z").unwrap();
    assert!(validator.earliest_expiry_date(earlier_date).is_ok());
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
      let validator = CredentialValidator::new(credential);
      assert!(validator.earliest_expiry_date(after_expiration_date).is_err());
      assert!(validator.earliest_expiry_date(before_expiration_date).is_ok());
    }
  }

  proptest! {
    #[test]
    fn property_based_expires_after_no_expiration_date(seconds in FIRST_RFC3999_COMPATIBLE_UNIX_TIMESTAMP..LAST_RFC3339_COMPATIBLE_UNIX_TIMESTAMP) {
      let mut credential = deserialize_credential(SIMPLE_CREDENTIAL_JSON);
      credential.expiration_date = None;
      // expires after whatever the timestamp may be because the expires_after field is None.
      let validator = CredentialValidator::new(credential);
      assert!(validator.earliest_expiry_date(Timestamp::from_unix(seconds).unwrap()).is_ok());
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
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2023-02-01T00:00:00Z").unwrap(); // note that expires_after > expiration_date
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    let validator = CredentialValidator::new(credential);
    // validate and extract the nested error according to our expectations
    let error = match validator.full_validation(&options, &[trusted_issuer]).unwrap_err() {
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
    let validator = CredentialValidator::new(credential);
    assert!(validator
      .latest_issuance_date(Timestamp::parse("2010-01-01T19:22:09Z").unwrap())
      .is_err());
    // and now with a later timestamp
    assert!(validator
      .latest_issuance_date(Timestamp::parse("2010-01-01T20:00:00Z").unwrap())
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
      let validator = CredentialValidator::new(credential);
      assert!(validator.latest_issuance_date(earlier_than_issuance_date).is_err());
      assert!(validator.latest_issuance_date(later_than_issuance_date).is_ok());
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
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2019-02-01T00:00:00Z").unwrap(); // note that issued_before < issuance_date
    let expires_after = Timestamp::parse("2022-02-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);

    let validator = CredentialValidator::new(&credential);
    // validate and extract the nested error according to our expectations
    let error = match validator.full_validation(&options, &[trusted_issuer]).unwrap_err() {
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
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-12-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    let validator = CredentialValidator::new(&credential);
    assert!(validator.full_validation(&options, &[trusted_issuer]).is_ok());
  }

  #[test]
  fn test_verify_signature_untrusted_issuer() {
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
    let trusted_issuer = test_utils::mock_resolved_document(other_doc); // the trusted issuer did not sign the credential
    let trusted_issuers = &[trusted_issuer];

    let validator = CredentialValidator::new(credential);

    assert!(matches!(
      validator
        .verify_signature(trusted_issuers, &VerifierOptions::default())
        .unwrap_err(),
      Error::UnsuccessfulValidationUnit(ValidationError::UntrustedIssuer)
    ));

    // also check that the full validation fails as expected
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-12-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);

    // validate and extract the nested error according to our expectations
    let error = match validator.full_validation(&options, trusted_issuers).unwrap_err() {
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
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let trusted_issuers = &[trusted_issuer];

    // run the validation unit
    let validator = CredentialValidator::new(&credential);
    assert!(matches!(
      validator
        .verify_signature(trusted_issuers, &VerifierOptions::default())
        .unwrap_err(),
      Error::UnsuccessfulValidationUnit(ValidationError::IssuerProof { .. })
    ));

    // check that full_validation also fails as expected
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-12-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    // validate and extract the nested error according to our expectations
    let error = match validator.full_validation(&options, trusted_issuers).unwrap_err() {
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
    let trusted_issuer = test_utils::mock_resolved_document(issuer_doc);
    let issued_before = Timestamp::parse("2020-02-01T00:00:00Z").unwrap();
    let expires_after = Timestamp::parse("2022-02-01T00:00:00Z").unwrap();
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    let validator = CredentialValidator::new(credential);
    // validate and extract the nested error according to our expectations
    let error = match validator.full_validation(&options, &[trusted_issuer]).unwrap_err() {
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
    let trusted_issuer = test_utils::mock_resolved_document(other_issuer); // trusted issuer did not issue the credential
    let issued_before = Timestamp::parse("2019-02-01T00:00:00Z").unwrap(); // issued_before < issuance_date
    let expires_after = Timestamp::parse("2024-02-01T00:00:00Z").unwrap(); // expires_after > expiration_date
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after);
    // fail_fast = true is default.
    let validator = CredentialValidator::new(credential);
    // validate and extract the nested error according to our expectations
    let error = match validator.full_validation(&options, &[trusted_issuer]).unwrap_err() {
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
    let trusted_issuer = test_utils::mock_resolved_document(other_issuer); // trusted issuer did not issue the credential [second error]
    let issued_before = Timestamp::parse("2019-02-01T00:00:00Z").unwrap(); // issued_before < issuance_date [third error]
    let expires_after = Timestamp::parse("2024-02-01T00:00:00Z").unwrap(); // expires_after > expiration_date [fourth error]
    let options = CredentialValidationOptions::default()
      .latest_issuance_date(issued_before)
      .earliest_expiry_date(expires_after)
      .fail_fast(false);
    let validator = CredentialValidator::new(credential);
    // validate and extract the nested error according to our expectations
    let error = match validator.full_validation(&options, &[trusted_issuer]).unwrap_err() {
      Error::UnsuccessfulCredentialValidation(accumulated_validation_error) => {
        accumulated_validation_error.validation_errors
      }
      _ => unreachable!(),
    };

    assert!(error.len() >= 4);
  }
}
