// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_credential::{
  presentation::{JwtPresentation, JwtPresentationBuilder, JwtPresentationOptions},
  validator::{FailFast, JwtPresentationValidationOptions, PresentationJwtValidator},
};
use identity_did::DID;
use identity_document::document::CoreDocument;

use crate::{
  storage::tests::test_utils::{generate_credential, setup_coredocument, Setup},
  JwkDocumentExt, JwsSignatureOptions,
};

#[tokio::test]
async fn test_presentation() {
  let issuer_setup: Setup<CoreDocument> = setup_coredocument(None).await;
  let holder_setup: Setup<CoreDocument> = setup_coredocument(None).await;
  let credential = generate_credential(&issuer_setup.issuer_doc, &[&issuer_setup.issuer_doc], None, None);
  let jws = issuer_setup
    .issuer_doc
    .sign_credential(
      &credential.credential,
      &issuer_setup.storage,
      &issuer_setup.method_fragment,
      &JwsSignatureOptions::default(),
    )
    .await
    .unwrap();

  let presentation: JwtPresentation = JwtPresentationBuilder::default()
    .holder(holder_setup.issuer_doc.id().to_url().into())
    .credential(jws)
    .build()
    .unwrap();

  let presentation_jwt = holder_setup
    .issuer_doc
    .sign_presentation(
      &presentation,
      &holder_setup.storage,
      &holder_setup.method_fragment,
      &JwsSignatureOptions::default(),
      &JwtPresentationOptions::default(),
    )
    .await
    .unwrap();

  let validator: PresentationJwtValidator = PresentationJwtValidator::new();
  validator
    .validate::<_, _, Object, Object>(
      &presentation_jwt,
      &holder_setup.issuer_doc,
      &vec![issuer_setup.issuer_doc],
      &JwtPresentationValidationOptions::default(),
      FailFast::FirstError,
    )
    .unwrap();
}
