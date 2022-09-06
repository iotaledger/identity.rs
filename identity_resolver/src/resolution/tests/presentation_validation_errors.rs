// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Error;
use crate::ErrorCause;
use crate::Resolver;
use identity_credential::validator::ValidatorDocument;
use identity_core::convert::FromJson;
use identity_credential::presentation::Presentation;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_did::did::CoreDID;
use identity_did::document::CoreDocument;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;

const SUBJECT_FOO_DOC: &str =
  include_str!("../../../../identity_credential/tests/fixtures/signed_presentation/subject_foo_doc.json");
const ISSUER_IOTA_DOC: &str =
  include_str!("../../../../identity_credential/tests/fixtures/signed_presentation/issuer_iota_doc.json");
const ISSUER_BAR_DOC: &str =
  include_str!("../../../../identity_credential/tests/fixtures/signed_presentation/issuer_bar_doc.json");
const PRESENTATION_JSON: &str =
  include_str!("../../../../identity_credential/tests/fixtures/signed_presentation/presentation.json");

type DynamicError = Box<dyn std::error::Error + Send + Sync + 'static>;
async fn misconfigured_iota_resolver(_did: StardustDID) -> Result<CoreDocument, DynamicError> {
  Ok(CoreDocument::from_json(SUBJECT_FOO_DOC).unwrap())
}

async fn misconfigured_bar_resolver(_did: CoreDID) -> Result<StardustDocument, DynamicError> {
  Ok(StardustDocument::from_json(ISSUER_IOTA_DOC).unwrap())
}

async fn misconfigured_foo_resolver(_did: CoreDID) -> Result<CoreDocument, DynamicError> {
  Ok(CoreDocument::from_json(ISSUER_BAR_DOC).unwrap())
}

/// checks that `Resolver::verify_presentation` fails when the resolver is misconfigured. 
async fn check_verify_presentation<DOC>(mut resolver: Resolver<DOC>)
where
  DOC: ValidatorDocument + From<CoreDocument> + From<StardustDocument> + Send + Sync,
{
  let correct_iota_issuer: StardustDocument = StardustDocument::from_json(ISSUER_IOTA_DOC).unwrap();
  let correct_bar_issuer: CoreDocument = CoreDocument::from_json(ISSUER_BAR_DOC).unwrap();
  let correct_issuers: [DOC; 2] = [correct_bar_issuer.into(), correct_iota_issuer.into()];
  let correct_holder: DOC = CoreDocument::from_json(SUBJECT_FOO_DOC).unwrap().into();

  resolver.attach_handler("iota".to_owned(), misconfigured_iota_resolver);
  resolver.attach_handler("bar".to_owned(), misconfigured_bar_resolver);
  resolver.attach_handler("foo".to_owned(), misconfigured_foo_resolver);

  let presentation: Presentation = Presentation::from_json(PRESENTATION_JSON).unwrap();

  let resolved_holder: DOC = resolver.resolve_presentation_holder(&presentation).await.unwrap();
  let resolved_issuers: Vec<DOC> = resolver.resolve_presentation_issuers(&presentation).await.unwrap();

  // make sure that verification passes when all correct arguments are passed
  let validation_options: PresentationValidationOptions = PresentationValidationOptions::default();
  let fail_fast: FailFast = FailFast::FirstError;
  assert!(resolver
    .verify_presentation(
      &presentation,
      &validation_options,
      fail_fast,
      Some(&correct_holder),
      Some(&correct_issuers)
    )
    .await
    .is_ok());

  // check when the holder argument is correct
  for use_resolved_issuers in [true, false] {
    let issuers: Option<&[DOC]> = (use_resolved_issuers).then_some(&resolved_issuers);
    assert!(matches!(resolver
      .verify_presentation(
        &presentation,
        &validation_options,
        fail_fast,
        Some(&correct_holder),
        issuers
      )
      .await
      .unwrap_err()
      .into_error_cause(), ErrorCause::PresentationValidationError { .. })
    );
  }
}
