// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_credential::credential::Credential;
use identity_credential::credential::Jwt;
use identity_credential::presentation::JwtPresentation;
use identity_credential::presentation::JwtPresentationBuilder;
use identity_credential::presentation::JwtPresentationOptions;
use identity_credential::validator::vc_jwt_validation::ValidationError;
use identity_credential::validator::FailFast;
use identity_credential::validator::JwtPresentationValidationOptions;
use identity_credential::validator::PresentationJwtValidator;
use identity_did::DID;
use identity_document::document::CoreDocument;

use crate::storage::tests::test_utils::generate_credential;
use crate::storage::tests::test_utils::setup_coredocument;
use crate::storage::tests::test_utils::setup_iotadocument;
use crate::storage::tests::test_utils::Setup;
use crate::JwkDocumentExt;
use crate::JwsSignatureOptions;

use super::test_utils::CredentialSetup;

#[tokio::test]
async fn test_presentation() {
  test_presentation_impl(setup_coredocument(None, None).await).await;
  test_presentation_impl(setup_iotadocument(None, None).await).await;
}
async fn test_presentation_impl<T>(setup: Setup<T, T>)
where
  T: JwkDocumentExt + AsRef<CoreDocument>,
{
  let credential: CredentialSetup = generate_credential(&setup.issuer_doc, &[&setup.subject_doc], None, None);
  let jws = sign_credential(&setup, &credential.credential).await;

  let presentation: JwtPresentation =
    JwtPresentationBuilder::new(setup.subject_doc.as_ref().id().to_url().into(), Object::new())
      .credential(jws)
      .build()
      .unwrap();

  let presentation_jwt = setup
    .subject_doc
    .sign_presentation(
      &presentation,
      &setup.subject_storage,
      &setup.subject_method_fragment,
      &JwsSignatureOptions::default(),
      &JwtPresentationOptions::default(),
    )
    .await
    .unwrap();

  let validator: PresentationJwtValidator = PresentationJwtValidator::new();
  validator
    .validate::<_, _, Object, Object>(
      &presentation_jwt,
      &setup.subject_doc,
      &vec![setup.issuer_doc],
      &JwtPresentationValidationOptions::default(),
      FailFast::FirstError,
    )
    .unwrap();
}

#[tokio::test]
async fn presentation_jws_error() {
  presentation_jws_error_impl(setup_coredocument(None, None).await).await;
  presentation_jws_error_impl(setup_iotadocument(None, None).await).await;
}

async fn presentation_jws_error_impl<T>(setup: Setup<T, T>)
where
  T: JwkDocumentExt + AsRef<CoreDocument> + Clone,
{
  let credential: CredentialSetup = generate_credential(&setup.issuer_doc, &[&setup.subject_doc], None, None);
  let jws = sign_credential(&setup, &credential.credential).await;

  let presentation: JwtPresentation =
    JwtPresentationBuilder::new(setup.subject_doc.as_ref().id().to_url().into(), Object::new())
      .credential(jws)
      .build()
      .unwrap();

  // Sign presentation using the issuer's method and try to verify it using the holder's document.
  // Since the holder's document doesn't include that verification method, Error is returned.

  let presentation_jwt = setup
    .issuer_doc
    .sign_presentation(
      &presentation,
      &setup.issuer_storage,
      &setup.issuer_method_fragment,
      &JwsSignatureOptions::default(),
      &JwtPresentationOptions::default(),
    )
    .await
    .unwrap();

  let validator: PresentationJwtValidator = PresentationJwtValidator::new();
  let validation_error: ValidationError = validator
    .validate::<_, _, Object, Object>(
      &presentation_jwt,
      &setup.subject_doc,
      &vec![setup.issuer_doc],
      &JwtPresentationValidationOptions::default(),
      FailFast::FirstError,
    )
    .err()
    .unwrap()
    .presentation_validation_errors
    .into_iter()
    .next()
    .unwrap();

  assert!(matches!(
    validation_error,
    ValidationError::PresentationJwsError(identity_document::Error::MethodNotFound)
  ));
}

async fn sign_credential<T>(setup: &Setup<T, T>, credential: &Credential) -> Jwt
where
  T: JwkDocumentExt + AsRef<CoreDocument>,
{
  setup
    .issuer_doc
    .sign_credential(
      credential,
      &setup.issuer_storage,
      &setup.issuer_method_fragment,
      &JwsSignatureOptions::default(),
    )
    .await
    .unwrap()
}
