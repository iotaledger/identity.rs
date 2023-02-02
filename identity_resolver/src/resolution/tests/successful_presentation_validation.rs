// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_credential::presentation::Presentation;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_credential::validator::SubjectHolderRelationship;
use identity_did::CoreDID;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_document::verifiable::VerifierOptions;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::Resolver;

use super::valid_presentation_data::HOLDER_FOO_DOC_JSON;
use super::valid_presentation_data::ISSUER_BAR_DOC_JSON;
use super::valid_presentation_data::ISSUER_IOTA_DOC_JSON;
use super::valid_presentation_data::PRESENTATION_JSON;

// Setup mock handlers:
#[derive(Debug, thiserror::Error)]
#[error("the document could not be resolved")]
struct ResolutionError;
// returns the deserialization of JSON (if the did matches) otherwise an error.
async fn resolve<D, DOC>(did: D, json: &str) -> Result<DOC, ResolutionError>
where
  D: DID + Send + Sync + 'static + Eq,
  DOC: AsRef<CoreDocument> + Send + Sync + 'static + DeserializeOwned,
{
  let doc: DOC = DOC::from_json(json).unwrap();
  (doc.as_ref().id().as_str() == did.as_str())
    .then_some(doc)
    .ok_or(ResolutionError)
}

#[derive(Deserialize)]
#[serde(transparent)]
pub(super) struct FooDocument(CoreDocument);
#[derive(Deserialize)]
#[serde(transparent)]
pub(super) struct BarDocument(CoreDocument);

impl AsRef<CoreDocument> for FooDocument {
  fn as_ref(&self) -> &CoreDocument {
    &self.0
  }
}

impl AsRef<CoreDocument> for BarDocument {
  fn as_ref(&self) -> &CoreDocument {
    &self.0
  }
}

impl From<FooDocument> for CoreDocument {
  fn from(value: FooDocument) -> Self {
    value.0
  }
}

impl From<BarDocument> for CoreDocument {
  fn from(value: BarDocument) -> Self {
    value.0
  }
}

enum DocumentTypes {
  Foo(FooDocument),
  Bar(BarDocument),
  Iota(IotaDocument),
}
impl AsRef<CoreDocument> for DocumentTypes {
  fn as_ref(&self) -> &CoreDocument {
    match self {
      Self::Foo(doc) => doc.as_ref(),
      Self::Bar(doc) => doc.as_ref(),
      Self::Iota(doc) => doc.as_ref(),
    }
  }
}
impl From<FooDocument> for DocumentTypes {
  fn from(value: FooDocument) -> Self {
    Self::Foo(value)
  }
}
impl From<BarDocument> for DocumentTypes {
  fn from(value: BarDocument) -> Self {
    Self::Bar(value)
  }
}
impl From<IotaDocument> for DocumentTypes {
  fn from(value: IotaDocument) -> Self {
    Self::Iota(value)
  }
}

async fn resolve_foo(did: CoreDID) -> Result<FooDocument, ResolutionError> {
  resolve(did, HOLDER_FOO_DOC_JSON).await
}

async fn resolve_iota(did: IotaDID) -> Result<IotaDocument, ResolutionError> {
  resolve(did, ISSUER_IOTA_DOC_JSON).await
}

async fn resolve_bar(did: CoreDID) -> Result<BarDocument, ResolutionError> {
  resolve(did, ISSUER_BAR_DOC_JSON).await
}

async fn check_verify_presentation<DOC>(mut resolver: Resolver<DOC>)
where
  DOC: AsRef<CoreDocument> + From<FooDocument> + From<BarDocument> + From<IotaDocument> + Send + Sync,
{
  let presentation: Presentation = Presentation::from_json(PRESENTATION_JSON).unwrap();

  resolver.attach_handler(IotaDID::METHOD.to_owned(), resolve_iota);
  resolver.attach_handler("foo".to_owned(), resolve_foo);
  resolver.attach_handler("bar".to_owned(), resolve_bar);

  // resolve the DID documents of the presentation's holder and credential issuers.
  let holder_doc = resolver.resolve_presentation_holder(&presentation).await.unwrap();
  let issuer_docs = resolver.resolve_presentation_issuers(&presentation).await.unwrap();

  // check that verification works regardless of whether we first resolve and then pass holder/issuers to the method or
  // if resolution of missing documents is done internally.
  for pass_holder_as_arg in [true, false] {
    for pass_issuers_as_arg in [true, false] {
      let holder: Option<&DOC> = pass_holder_as_arg.then_some(&holder_doc);
      let issuers: Option<&[DOC]> = pass_issuers_as_arg.then_some(&issuer_docs);
      assert!(resolver
        .verify_presentation(
          &presentation,
          &PresentationValidationOptions::new()
            .presentation_verifier_options(
              VerifierOptions::new().challenge(presentation.proof.clone().unwrap().challenge.unwrap())
            )
            .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
          FailFast::FirstError,
          holder,
          issuers
        )
        .await
        .is_ok());
    }
  }
}

#[tokio::test]
async fn correct_presentation_validation() {
  let core_resolver: Resolver<CoreDocument> = Resolver::new();
  let custom_resolver: Resolver<DocumentTypes> = Resolver::new();
  check_verify_presentation(core_resolver).await;
  check_verify_presentation(custom_resolver).await;
}
