// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Duration;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_credential::validator::vc_jwt_validation::CredentialValidationOptions;
use identity_credential::validator::vc_jwt_validation::CredentialValidator;
use identity_credential::validator::vc_jwt_validation::ValidationError;
use identity_credential::validator::FailFast;

use crate::storage::tests::test_utils::Setup;
use crate::storage::tests::test_utils::{self};
use crate::storage::JwkStorageDocumentExt;
use crate::storage::JwsSignatureOptions;

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
