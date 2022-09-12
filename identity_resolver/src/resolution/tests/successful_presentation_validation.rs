// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_credential::presentation::Presentation;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_credential::validator::SubjectHolderRelationship;
use identity_credential::validator::ValidatorDocument;
use identity_did::did::CoreDID;
use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::document::Document;
use identity_did::verifiable::VerifierOptions;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use serde::de::DeserializeOwned;

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
  DOC: Document + Send + Sync + 'static + DeserializeOwned,
  <DOC as Document>::D: PartialEq<D>,
{
  let doc: DOC = DOC::from_json(json).unwrap();
  (doc.id() == &did).then_some(doc).ok_or(ResolutionError)
}

async fn resolve_foo(did: CoreDID) -> Result<CoreDocument, ResolutionError> {
  resolve(did, HOLDER_FOO_DOC_JSON).await
}

async fn resolve_iota(did: IotaDID) -> Result<IotaDocument, ResolutionError> {
  resolve(did, ISSUER_IOTA_DOC_JSON).await
}

async fn resolve_bar(did: CoreDID) -> Result<CoreDocument, ResolutionError> {
  resolve(did, ISSUER_BAR_DOC_JSON).await
}

async fn check_verify_presentation<DOC>(mut resolver: Resolver<DOC>)
where
  DOC: ValidatorDocument + From<CoreDocument> + From<IotaDocument> + Send + Sync,
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
  let dynamic_resolver: Resolver = Resolver::new();
  check_verify_presentation(core_resolver).await;
  check_verify_presentation(dynamic_resolver).await;
}
