// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::ErrorCause;
use crate::Resolver;
use identity_core::convert::FromJson;
use identity_credential::presentation::Presentation;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_did::CoreDID;
use identity_document::document::CoreDocument;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;

use super::valid_presentation_data::HOLDER_FOO_DOC_JSON;
use super::valid_presentation_data::ISSUER_BAR_DOC_JSON;
use super::valid_presentation_data::ISSUER_IOTA_DOC_JSON;
use super::valid_presentation_data::PRESENTATION_JSON;

type DynamicError = Box<dyn std::error::Error + Send + Sync + 'static>;
async fn misconfigured_iota_resolver(_did: IotaDID) -> Result<CoreDocument, DynamicError> {
  Ok(CoreDocument::from_json(HOLDER_FOO_DOC_JSON).unwrap())
}

async fn misconfigured_bar_resolver(_did: CoreDID) -> Result<IotaDocument, DynamicError> {
  Ok(IotaDocument::from_json(ISSUER_IOTA_DOC_JSON).unwrap())
}

async fn misconfigured_foo_resolver(_did: CoreDID) -> Result<CoreDocument, DynamicError> {
  Ok(CoreDocument::from_json(ISSUER_BAR_DOC_JSON).unwrap())
}

enum DocumentTypes {
  Iota(IotaDocument),
  CoreDocument(CoreDocument),
}

impl AsRef<CoreDocument> for DocumentTypes {
  fn as_ref(&self) -> &CoreDocument {
    match self {
      Self::Iota(doc) => doc.as_ref(),
      Self::CoreDocument(doc) => doc.as_ref(),
    }
  }
}
impl From<IotaDocument> for DocumentTypes {
  fn from(value: IotaDocument) -> Self {
    Self::Iota(value)
  }
}
impl From<CoreDocument> for DocumentTypes {
  fn from(value: CoreDocument) -> Self {
    Self::CoreDocument(value)
  }
}

/// checks that `Resolver::verify_presentation` fails when the resolver is misconfigured.
async fn check_verify_presentation<DOC>(mut resolver: Resolver<DOC>)
where
  DOC: AsRef<CoreDocument> + From<CoreDocument> + From<IotaDocument> + Send + Sync,
{
  let correct_iota_issuer: IotaDocument = IotaDocument::from_json(ISSUER_IOTA_DOC_JSON).unwrap();
  let correct_bar_issuer: CoreDocument = CoreDocument::from_json(ISSUER_BAR_DOC_JSON).unwrap();
  let correct_issuers: [DOC; 2] = [correct_bar_issuer.into(), correct_iota_issuer.into()];
  let correct_holder: DOC = CoreDocument::from_json(HOLDER_FOO_DOC_JSON).unwrap().into();

  resolver.attach_handler("iota".to_owned(), misconfigured_iota_resolver);
  resolver.attach_handler("bar".to_owned(), misconfigured_bar_resolver);
  resolver.attach_handler("foo".to_owned(), misconfigured_foo_resolver);

  let presentation: Presentation = Presentation::from_json(PRESENTATION_JSON).unwrap();

  let resolved_holder: DOC = resolver.resolve_presentation_holder(&presentation).await.unwrap();
  let resolved_issuers: Vec<DOC> = resolver.resolve_presentation_issuers(&presentation).await.unwrap();

  // Make sure that verification passes when all correct arguments are passed
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

  // Fails when the holder argument is correct, but the issuers get resolved with a misconfigured handler
  for use_resolved_issuers in [true, false] {
    let issuers: Option<&[DOC]> = (use_resolved_issuers).then_some(&resolved_issuers);
    assert!(matches!(
      resolver
        .verify_presentation(
          &presentation,
          &validation_options,
          fail_fast,
          Some(&correct_holder),
          issuers
        )
        .await
        .unwrap_err()
        .into_error_cause(),
      ErrorCause::PresentationValidationError { .. }
    ));
  }

  // Fails when the issuer argument is correct , but the holder gets resolved with a misconfigured handler
  for use_resolved_holder in [true, false] {
    let holder: Option<&DOC> = (use_resolved_holder).then_some(&resolved_holder);
    assert!(matches!(
      resolver
        .verify_presentation(
          &presentation,
          &validation_options,
          fail_fast,
          holder,
          Some(&correct_issuers)
        )
        .await
        .unwrap_err()
        .into_error_cause(),
      ErrorCause::PresentationValidationError { .. }
    ));
  }

  // Fails when no arguments are given when using a misconfigured resolver
  assert!(matches!(
    resolver
      .verify_presentation(&presentation, &validation_options, fail_fast, None, None)
      .await
      .unwrap_err()
      .into_error_cause(),
    ErrorCause::PresentationValidationError { .. }
  ));
}

#[tokio::test]
async fn misconfigured_resolvers_verify_incorrectly() {
  let resolver_core: Resolver<CoreDocument> = Resolver::new();
  let resolver: Resolver<DocumentTypes> = Resolver::new();
  check_verify_presentation(resolver_core).await;
  check_verify_presentation(resolver).await;
}
