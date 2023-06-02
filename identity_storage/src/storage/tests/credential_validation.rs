// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Duration;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_credential::credential::Credential;
use identity_credential::credential::Jwt;
use identity_credential::credential::RevocationBitmapStatus;
use identity_credential::credential::Status;
use identity_credential::credential::Subject;
use identity_credential::revocation::RevocationBitmap;
use identity_credential::revocation::RevocationDocumentExt;
use identity_credential::validator::vc_jwt_validation::CredentialValidationOptions;
use identity_credential::validator::vc_jwt_validation::CredentialValidator;
use identity_credential::validator::vc_jwt_validation::ValidationError;
use identity_credential::validator::FailFast;
use identity_credential::validator::StatusCheck;
use identity_credential::validator::SubjectHolderRelationship;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_document::service::Service;
use identity_document::verifiable::JwsVerificationOptions;
use once_cell::sync::Lazy;
use proptest::proptest;

use crate::storage::tests::test_utils;
use crate::storage::tests::test_utils::CredentialSetup;
use crate::storage::tests::test_utils::Setup;
use crate::storage::JwkDocumentExt;
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

// TODO: Move to identity_credential?
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

async fn invalid_expiration_or_issuance_date_impl<T>(setup: Setup<T, T>)
where
  T: JwkDocumentExt + AsRef<CoreDocument>,
{
  let Setup {
    issuer_doc,
    subject_doc,
    issuer_storage: storage,
    issuer_method_fragment: method_fragment,
    subject_storage: _,
    subject_method_fragment: _,
  } = setup;

  let CredentialSetup {
    credential,
    issuance_date,
    expiration_date,
  } = test_utils::generate_credential(&issuer_doc, &[&subject_doc], None, None);

  let jws = issuer_doc
    .sign_credential(
      &credential,
      &storage,
      method_fragment.as_ref(),
      &JwsSignatureOptions::default(),
    )
    .await
    .unwrap();

  // Test invalid expiration date.
  {
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

  // Test invalid issuance date.
  {
    // issued_on_or_before < issuance_date
    let issued_on_or_before = issuance_date.checked_sub(Duration::seconds(1)).unwrap();
    let expires_on_or_after = expiration_date;
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

    assert!(matches!(error, &ValidationError::IssuanceDate));
  }
}

#[tokio::test]
async fn invalid_expiration_or_issuance_date() {
  invalid_expiration_or_issuance_date_impl(test_utils::setup_coredocument(None, None).await).await;
  invalid_expiration_or_issuance_date_impl(test_utils::setup_iotadocument(None, None).await).await;
}

async fn full_validation_impl<T>(setup: Setup<T, T>)
where
  T: JwkDocumentExt + AsRef<CoreDocument>,
{
  let Setup {
    issuer_doc,
    subject_doc,
    issuer_storage: storage,
    issuer_method_fragment: method_fragment,
    subject_storage: _,
    subject_method_fragment: _,
  } = setup;

  let CredentialSetup {
    credential,
    issuance_date,
    expiration_date,
  } = test_utils::generate_credential(&issuer_doc, &[&subject_doc], None, None);

  let jwt: Jwt = issuer_doc
    .sign_credential(
      &credential,
      &storage,
      method_fragment.as_ref(),
      &JwsSignatureOptions::default(),
    )
    .await
    .unwrap();

  let issued_on_or_before: Timestamp = issuance_date.checked_add(Duration::days(14)).unwrap();
  let expires_on_or_after: Timestamp = expiration_date.checked_sub(Duration::hours(1)).unwrap();
  let options = CredentialValidationOptions::default()
    .latest_issuance_date(issued_on_or_before)
    .earliest_expiry_date(expires_on_or_after);
  assert!(CredentialValidator::new()
    .validate::<_, Object>(&jwt, &issuer_doc, &options, FailFast::FirstError)
    .is_ok());
}

#[tokio::test]
async fn full_validation() {
  full_validation_impl(test_utils::setup_coredocument(None, None).await).await;
  full_validation_impl(test_utils::setup_iotadocument(None, None).await).await;
}

async fn matches_issuer_did_unrelated_issuer_impl<T>(setup: Setup<T, T>)
where
  T: JwkDocumentExt + AsRef<CoreDocument>,
{
  let Setup {
    issuer_doc,
    subject_doc,
    issuer_storage: storage,
    issuer_method_fragment: method_fragment,
    subject_storage: _,
    subject_method_fragment: _,
  } = setup;

  let CredentialSetup { credential, .. } = test_utils::generate_credential(&issuer_doc, &[&subject_doc], None, None);

  let jwt: Jwt = issuer_doc
    .sign_credential(
      &credential,
      &storage,
      method_fragment.as_ref(),
      &JwsSignatureOptions::default(),
    )
    .await
    .unwrap();

  // the credential was not signed by this issuer
  // check that `verify_signature` returns the expected error
  let error = CredentialValidator::new()
    .verify_signature::<_, Object>(&jwt, &[&subject_doc], &JwsVerificationOptions::default())
    .unwrap_err();

  assert!(matches!(error, ValidationError::DocumentMismatch { .. }));

  // also check that the full validation fails as expected
  let options = CredentialValidationOptions::default();

  // validate and extract the nested error according to our expectations
  let validation_errors = CredentialValidator::new()
    .validate::<_, Object>(&jwt, &subject_doc, &options, FailFast::FirstError)
    .unwrap_err()
    .validation_errors;

  let error = match validation_errors.as_slice() {
    [validation_error] => validation_error,
    _ => unreachable!(),
  };

  assert!(matches!(error, ValidationError::DocumentMismatch { .. }));
}

#[tokio::test]
async fn matches_issuer_did_unrelated_issuer() {
  matches_issuer_did_unrelated_issuer_impl(test_utils::setup_coredocument(None, None).await).await;
  matches_issuer_did_unrelated_issuer_impl(test_utils::setup_iotadocument(None, None).await).await;
}

async fn verify_invalid_signature_impl<T>(setup: Setup<T, T>, other_setup: Setup<T, T>, fragment: &'static str)
where
  T: JwkDocumentExt + AsRef<CoreDocument>,
{
  let Setup {
    issuer_doc,
    subject_doc,
    ..
  } = setup;

  let Setup {
    issuer_doc: other_issuer_doc,
    issuer_storage: other_storage,
    ..
  } = other_setup;

  let CredentialSetup {
    credential,
    issuance_date,
    expiration_date,
  } = test_utils::generate_credential(&issuer_doc, &[&subject_doc], None, None);

  // Sign the credential with the *other* issuer.
  let jwt: Jwt = other_issuer_doc
    .sign_credential(&credential, &other_storage, fragment, &JwsSignatureOptions::default())
    .await
    .unwrap();

  let err = CredentialValidator::new()
    .verify_signature::<_, Object>(&jwt, &[&issuer_doc], &JwsVerificationOptions::default())
    .unwrap_err();

  // run the validation unit
  // we expect that the kid won't resolve to a method on the issuer_doc's document.
  assert!(matches!(err, ValidationError::Signature { .. }));

  // check that full_validation also fails as expected
  let issued_on_or_before = issuance_date.checked_add(Duration::days(14)).unwrap();
  let expires_on_or_after = expiration_date.checked_sub(Duration::hours(1)).unwrap();
  let options = CredentialValidationOptions::default()
    .latest_issuance_date(issued_on_or_before)
    .earliest_expiry_date(expires_on_or_after);

  // validate and extract the nested error according to our expectations
  let validation_errors = CredentialValidator::new()
    .validate::<_, Object>(&jwt, &issuer_doc, &options, FailFast::FirstError)
    .unwrap_err()
    .validation_errors;

  let error = match validation_errors.as_slice() {
    [validation_error] => validation_error,
    _ => unreachable!(),
  };

  // we expect that the kid won't resolve to a method on the issuer_doc's document.
  assert!(matches!(error, &ValidationError::Signature { .. }));
}

#[tokio::test]
async fn verify_invalid_signature() {
  // Ensure the fragment is the same on both documents so we can produce the signature verification error.
  let fragment = "signing-key";
  verify_invalid_signature_impl(
    test_utils::setup_coredocument(Some(fragment), None).await,
    test_utils::setup_coredocument(Some(fragment), None).await,
    fragment,
  )
  .await;
  verify_invalid_signature_impl(
    test_utils::setup_iotadocument(Some(fragment), None).await,
    test_utils::setup_iotadocument(Some(fragment), None).await,
    fragment,
  )
  .await;
}

async fn check_subject_holder_relationship_impl<T>(setup: Setup<T, T>)
where
  T: JwkDocumentExt + AsRef<CoreDocument>,
{
  let Setup { issuer_doc, .. } = setup;

  let mut credential: Credential = SIMPLE_CREDENTIAL.clone();

  // first ensure that holder_url is the subject and set the nonTransferable property
  let actual_holder_url = credential.credential_subject.first().unwrap().id.clone().unwrap();
  assert_eq!(credential.credential_subject.len(), 1);
  credential.non_transferable = Some(true);

  // checking with holder = subject passes for all defined subject holder relationships:
  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential,
    &actual_holder_url,
    SubjectHolderRelationship::AlwaysSubject
  )
  .is_ok());

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential,
    &actual_holder_url,
    SubjectHolderRelationship::SubjectOnNonTransferable
  )
  .is_ok());

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential,
    &actual_holder_url,
    SubjectHolderRelationship::Any
  )
  .is_ok());

  // check with a holder different from the subject of the credential:
  let issuer_url = Url::parse(issuer_doc.as_ref().id().as_str()).unwrap();
  assert!(actual_holder_url != issuer_url);

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential,
    &issuer_url,
    SubjectHolderRelationship::AlwaysSubject
  )
  .is_err());

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential,
    &issuer_url,
    SubjectHolderRelationship::SubjectOnNonTransferable
  )
  .is_err());

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential,
    &issuer_url,
    SubjectHolderRelationship::Any
  )
  .is_ok());

  let mut credential_transferable = credential.clone();

  credential_transferable.non_transferable = Some(false);

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential_transferable,
    &issuer_url,
    SubjectHolderRelationship::SubjectOnNonTransferable
  )
  .is_ok());

  credential_transferable.non_transferable = None;

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential_transferable,
    &issuer_url,
    SubjectHolderRelationship::SubjectOnNonTransferable
  )
  .is_ok());

  // two subjects (even when they are both the holder) should fail for all defined values except "Any"

  let mut credential_duplicated_holder = credential;
  credential_duplicated_holder
    .credential_subject
    .push(Subject::with_id(actual_holder_url));

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential_duplicated_holder,
    &issuer_url,
    SubjectHolderRelationship::AlwaysSubject
  )
  .is_err());

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential_duplicated_holder,
    &issuer_url,
    SubjectHolderRelationship::SubjectOnNonTransferable
  )
  .is_err());

  assert!(CredentialValidator::check_subject_holder_relationship(
    &credential_duplicated_holder,
    &issuer_url,
    SubjectHolderRelationship::Any
  )
  .is_ok());
}

#[tokio::test]
async fn check_subject_holder_relationship() {
  check_subject_holder_relationship_impl(test_utils::setup_coredocument(None, None).await).await;
  check_subject_holder_relationship_impl(test_utils::setup_iotadocument(None, None).await).await;
}

fn check_status_impl<T, F>(setup: Setup<T, T>, insert_service: F)
where
  T: JwkDocumentExt + AsRef<CoreDocument> + RevocationDocumentExt,
  F: Fn(&mut T, Service),
{
  let Setup {
    mut issuer_doc,
    subject_doc,
    ..
  } = setup;
  let CredentialSetup { mut credential, .. } =
    test_utils::generate_credential(&issuer_doc, &[&subject_doc], None, None);

  // 0: missing status always succeeds.
  for status_check in [StatusCheck::Strict, StatusCheck::SkipUnsupported, StatusCheck::SkipAll] {
    assert!(CredentialValidator::check_status(&credential, &[&issuer_doc], status_check).is_ok());
  }

  // 1: unsupported status type.
  credential.credential_status = Some(Status::new(
    Url::parse("https://example.com/").unwrap(),
    "UnsupportedStatus2023".to_owned(),
  ));
  for (status_check, expected) in [
    (StatusCheck::Strict, false),
    (StatusCheck::SkipUnsupported, true),
    (StatusCheck::SkipAll, true),
  ] {
    assert_eq!(
      CredentialValidator::check_status(&credential, &[&issuer_doc], status_check).is_ok(),
      expected
    );
  }

  // Add a RevocationBitmap status to the credential.
  let service_url: identity_did::DIDUrl = issuer_doc.as_ref().id().to_url().join("#revocation-service").unwrap();
  let index: u32 = 42;
  credential.credential_status = Some(RevocationBitmapStatus::new(service_url.clone(), index).into());

  // 2: missing service in DID Document.
  for (status_check, expected) in [
    (StatusCheck::Strict, false),
    (StatusCheck::SkipUnsupported, false),
    (StatusCheck::SkipAll, true),
  ] {
    assert_eq!(
      CredentialValidator::check_status(&credential, &[&issuer_doc], status_check).is_ok(),
      expected
    );
  }

  // Add a RevocationBitmap service to the issuer.
  let bitmap: RevocationBitmap = RevocationBitmap::new();

  insert_service(
    &mut issuer_doc,
    Service::builder(Object::new())
      .id(service_url.clone())
      .type_(RevocationBitmap::TYPE)
      .service_endpoint(bitmap.to_endpoint().unwrap())
      .build()
      .unwrap(),
  );

  // 3: un-revoked index always succeeds.
  for status_check in [StatusCheck::Strict, StatusCheck::SkipUnsupported, StatusCheck::SkipAll] {
    assert!(CredentialValidator::check_status(&credential, &[&issuer_doc], status_check).is_ok());
  }

  // 4: revoked index.
  <T as RevocationDocumentExt>::revoke_credentials(&mut issuer_doc, &service_url, &[index]).unwrap();
  for (status_check, expected) in [
    (StatusCheck::Strict, false),
    (StatusCheck::SkipUnsupported, false),
    (StatusCheck::SkipAll, true),
  ] {
    assert_eq!(
      CredentialValidator::check_status(&credential, &[&issuer_doc], status_check).is_ok(),
      expected
    );
  }
}

// Note: We don't test `IotaDocument` because it (intentionally) doesn't implement RevocationDocumentExt.
#[tokio::test]
async fn check_status() {
  check_status_impl(
    test_utils::setup_coredocument(None, None).await,
    |document: &mut CoreDocument, service: Service| {
      document.insert_service(service).unwrap();
    },
  );
}

async fn full_validation_fail_fast_impl<T, U>(setup: Setup<T, U>)
where
  T: JwkDocumentExt + AsRef<CoreDocument>,
  U: JwkDocumentExt + AsRef<CoreDocument>,
{
  let Setup {
    issuer_doc,
    subject_doc,
    issuer_storage: storage,
    issuer_method_fragment: method_fragment,
    subject_storage: _,
    subject_method_fragment: _,
  } = setup;

  let CredentialSetup {
    credential,
    issuance_date,
    expiration_date,
  } = test_utils::generate_credential(&issuer_doc, &[&subject_doc], None, None);

  let jws = issuer_doc
    .sign_credential(
      &credential,
      &storage,
      method_fragment.as_ref(),
      &JwsSignatureOptions::default(),
    )
    .await
    .unwrap();

  // issued_on_or_before < issuance_date
  let issued_on_or_before = issuance_date.checked_sub(Duration::seconds(1)).unwrap();
  // expires_on_or_after > expiration_date
  let expires_on_or_after = expiration_date.checked_add(Duration::seconds(1)).unwrap();
  let options = CredentialValidationOptions::default()
    .latest_issuance_date(issued_on_or_before)
    .earliest_expiry_date(expires_on_or_after);

  let validation_errors = CredentialValidator::new()
    .validate::<_, Object>(&jws, &issuer_doc, &options, FailFast::FirstError)
    .unwrap_err()
    .validation_errors;

  assert!(validation_errors.len() == 1);

  let validation_errors = CredentialValidator::new()
    .validate::<_, Object>(&jws, &issuer_doc, &options, FailFast::AllErrors)
    .unwrap_err()
    .validation_errors;

  assert!(validation_errors.len() >= 2);
}

#[tokio::test]
async fn full_validation_fail_fast() {
  full_validation_fail_fast_impl(test_utils::setup_coredocument(None, None).await).await;
  full_validation_fail_fast_impl(test_utils::setup_iotadocument(None, None).await).await;
}
