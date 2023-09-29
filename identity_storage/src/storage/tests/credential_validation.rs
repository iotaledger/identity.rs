// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Duration;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_credential::credential::Jwt;
use identity_credential::credential::RevocationBitmapStatus;
use identity_credential::credential::Status;
use identity_credential::revocation::RevocationBitmap;
use identity_credential::revocation::RevocationDocumentExt;
use identity_credential::validator::FailFast;
use identity_credential::validator::JwtCredentialValidationOptions;
use identity_credential::validator::JwtCredentialValidator;
use identity_credential::validator::JwtCredentialValidatorUtils;
use identity_credential::validator::JwtValidationError;
use identity_credential::validator::StatusCheck;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_document::service::Service;
use identity_document::verifiable::JwsVerificationOptions;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use once_cell::sync::Lazy;

use crate::storage::tests::test_utils;
use crate::storage::tests::test_utils::CredentialSetup;
use crate::storage::tests::test_utils::Setup;
use crate::storage::JwkDocumentExt;
use crate::storage::JwsSignatureOptions;

static JWT_CREDENTIAL_VALIDATOR_ED25519: Lazy<JwtCredentialValidator<EdDSAJwsVerifier>> =
  Lazy::new(|| JwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default()));

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
    .create_credential_jwt(
      &credential,
      &storage,
      method_fragment.as_ref(),
      &JwsSignatureOptions::default(),
      None,
    )
    .await
    .unwrap();

  // Test invalid expiration date.
  {
    let issued_on_or_before = issuance_date;
    // expires_on_or_after > expiration_date
    let expires_on_or_after = expiration_date.checked_add(Duration::seconds(1)).unwrap();
    let options = JwtCredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);

    // validate and extract the nested error according to our expectations
    let validation_errors = JWT_CREDENTIAL_VALIDATOR_ED25519
      .validate::<_, Object>(&jws, &issuer_doc, &options, FailFast::FirstError)
      .unwrap_err()
      .validation_errors;

    let error = match validation_errors.as_slice() {
      [validation_error] => validation_error,
      _ => unreachable!(),
    };

    assert!(matches!(error, &JwtValidationError::ExpirationDate));
  }

  // Test invalid issuance date.
  {
    // issued_on_or_before < issuance_date
    let issued_on_or_before = issuance_date.checked_sub(Duration::seconds(1)).unwrap();
    let expires_on_or_after = expiration_date;
    let options = JwtCredentialValidationOptions::default()
      .latest_issuance_date(issued_on_or_before)
      .earliest_expiry_date(expires_on_or_after);

    // validate and extract the nested error according to our expectations
    let validation_errors = JWT_CREDENTIAL_VALIDATOR_ED25519
      .validate::<_, Object>(&jws, &issuer_doc, &options, FailFast::FirstError)
      .unwrap_err()
      .validation_errors;

    let error = match validation_errors.as_slice() {
      [validation_error] => validation_error,
      _ => unreachable!(),
    };

    assert!(matches!(error, &JwtValidationError::IssuanceDate));
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
    .create_credential_jwt(
      &credential,
      &storage,
      method_fragment.as_ref(),
      &JwsSignatureOptions::default(),
      None,
    )
    .await
    .unwrap();

  let issued_on_or_before: Timestamp = issuance_date.checked_add(Duration::days(14)).unwrap();
  let expires_on_or_after: Timestamp = expiration_date.checked_sub(Duration::hours(1)).unwrap();
  let options = JwtCredentialValidationOptions::default()
    .latest_issuance_date(issued_on_or_before)
    .earliest_expiry_date(expires_on_or_after);
  assert!(JWT_CREDENTIAL_VALIDATOR_ED25519
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
    .create_credential_jwt(
      &credential,
      &storage,
      method_fragment.as_ref(),
      &JwsSignatureOptions::default(),
      None,
    )
    .await
    .unwrap();

  // the credential was not signed by this issuer
  // check that `verify_signature` returns the expected error
  let error = JWT_CREDENTIAL_VALIDATOR_ED25519
    .verify_signature::<_, Object>(&jwt, &[&subject_doc], &JwsVerificationOptions::default())
    .unwrap_err();

  assert!(matches!(error, JwtValidationError::DocumentMismatch { .. }));

  // also check that the full validation fails as expected
  let options = JwtCredentialValidationOptions::default();

  // validate and extract the nested error according to our expectations
  let validation_errors = JWT_CREDENTIAL_VALIDATOR_ED25519
    .validate::<_, Object>(&jwt, &subject_doc, &options, FailFast::FirstError)
    .unwrap_err()
    .validation_errors;

  let error = match validation_errors.as_slice() {
    [validation_error] => validation_error,
    _ => unreachable!(),
  };

  assert!(matches!(error, JwtValidationError::DocumentMismatch { .. }));
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
    .create_credential_jwt(
      &credential,
      &other_storage,
      fragment,
      &JwsSignatureOptions::default(),
      None,
    )
    .await
    .unwrap();

  let err = JWT_CREDENTIAL_VALIDATOR_ED25519
    .verify_signature::<_, Object>(&jwt, &[&issuer_doc], &JwsVerificationOptions::default())
    .unwrap_err();

  // run the validation unit
  // we expect that the kid won't resolve to a method on the issuer_doc's document.
  assert!(matches!(err, JwtValidationError::Signature { .. }));

  // check that full_validation also fails as expected
  let issued_on_or_before = issuance_date.checked_add(Duration::days(14)).unwrap();
  let expires_on_or_after = expiration_date.checked_sub(Duration::hours(1)).unwrap();
  let options = JwtCredentialValidationOptions::default()
    .latest_issuance_date(issued_on_or_before)
    .earliest_expiry_date(expires_on_or_after);

  // validate and extract the nested error according to our expectations
  let validation_errors = JWT_CREDENTIAL_VALIDATOR_ED25519
    .validate::<_, Object>(&jwt, &issuer_doc, &options, FailFast::FirstError)
    .unwrap_err()
    .validation_errors;

  let error = match validation_errors.as_slice() {
    [validation_error] => validation_error,
    _ => unreachable!(),
  };

  // we expect that the kid won't resolve to a method on the issuer_doc's document.
  assert!(matches!(error, &JwtValidationError::Signature { .. }));
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
    assert!(JwtCredentialValidatorUtils::check_status(&credential, &[&issuer_doc], status_check).is_ok());
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
      JwtCredentialValidatorUtils::check_status(&credential, &[&issuer_doc], status_check).is_ok(),
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
      JwtCredentialValidatorUtils::check_status(&credential, &[&issuer_doc], status_check).is_ok(),
      expected
    );
  }

  // Add a RevocationBitmap service to the issuer.
  let bitmap: RevocationBitmap = RevocationBitmap::new();

  insert_service(&mut issuer_doc, bitmap.to_service(service_url.clone()).unwrap());

  // 3: un-revoked index always succeeds.
  for status_check in [StatusCheck::Strict, StatusCheck::SkipUnsupported, StatusCheck::SkipAll] {
    assert!(JwtCredentialValidatorUtils::check_status(&credential, &[&issuer_doc], status_check).is_ok());
  }

  // 4: revoked index.
  <T as RevocationDocumentExt>::revoke_credentials(&mut issuer_doc, &service_url, &[index]).unwrap();
  for (status_check, expected) in [
    (StatusCheck::Strict, false),
    (StatusCheck::SkipUnsupported, false),
    (StatusCheck::SkipAll, true),
  ] {
    assert_eq!(
      JwtCredentialValidatorUtils::check_status(&credential, &[&issuer_doc], status_check).is_ok(),
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
    .create_credential_jwt(
      &credential,
      &storage,
      method_fragment.as_ref(),
      &JwsSignatureOptions::default(),
      None,
    )
    .await
    .unwrap();

  // issued_on_or_before < issuance_date
  let issued_on_or_before = issuance_date.checked_sub(Duration::seconds(1)).unwrap();
  // expires_on_or_after > expiration_date
  let expires_on_or_after = expiration_date.checked_add(Duration::seconds(1)).unwrap();
  let options = JwtCredentialValidationOptions::default()
    .latest_issuance_date(issued_on_or_before)
    .earliest_expiry_date(expires_on_or_after);

  let validation_errors = JWT_CREDENTIAL_VALIDATOR_ED25519
    .validate::<_, Object>(&jws, &issuer_doc, &options, FailFast::FirstError)
    .unwrap_err()
    .validation_errors;

  assert!(validation_errors.len() == 1);

  let validation_errors = JWT_CREDENTIAL_VALIDATOR_ED25519
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
