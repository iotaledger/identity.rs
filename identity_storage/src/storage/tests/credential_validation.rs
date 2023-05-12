// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Duration;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::convert::FromJson;
use identity_credential::credential::Credential;
use identity_credential::validator::vc_jwt_validation::CredentialValidationOptions;
use identity_credential::validator::vc_jwt_validation::CredentialValidator;
use identity_credential::validator::vc_jwt_validation::ValidationError;
use identity_credential::validator::FailFast;
use once_cell::sync::Lazy;
use proptest::proptest;

use crate::storage::tests::test_utils::Setup;
use crate::storage::tests::test_utils::{self};
use crate::storage::JwkStorageDocumentExt;
use crate::storage::JwsSignatureOptions;

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

// A simple credential shared by some of the tests in this module
static SIMPLE_CREDENTIAL: Lazy<Credential> =
  Lazy::new(|| Credential::<Object>::from_json(SIMPLE_CREDENTIAL_JSON).unwrap());

#[tokio::test]
async fn invalid_expiration_date() {
  let Setup {
    issuer_doc,
    subject_doc,
    storage,
    kid,
  } = test_utils::setup().await;

  let issuance_date = Timestamp::parse("2020-01-01T00:00:00Z").unwrap();
  let expiration_date = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
  let credential = test_utils::generate_credential(&issuer_doc, &[subject_doc], issuance_date, expiration_date);

  let jws = issuer_doc
    .sign_credential(&credential, &storage, kid.as_ref(), &JwsSignatureOptions::default())
    .await
    .unwrap();

  let issued_on_or_before = issuance_date;
  // expires_on_or_after > expiration_date
  let expires_on_or_after = expiration_date.checked_add(Duration::seconds(1)).unwrap();
  let options = CredentialValidationOptions::default()
    .latest_issuance_date(issued_on_or_before)
    .earliest_expiry_date(expires_on_or_after);

  // validate and extract the nested error according to our expectations
  let validation_errors = CredentialValidator::new()
    .validate::<_, Object>(&jws, &issuer_doc, &options, FailFast::FirstError)
    .unwrap_err()
    .validation_errors;

  let error = match validation_errors.as_slice() {
    [validation_error] => validation_error,
    _ => unreachable!(),
  };

  assert!(matches!(error, &ValidationError::ExpirationDate));
}

#[test]
fn issued_on_or_before() {
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
