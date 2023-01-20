// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::str::FromStr;

use identity_credential::credential::Credential;
use identity_credential::credential::CredentialBuilder;
use identity_credential::credential::Subject;
use identity_credential::presentation::Presentation;
use identity_credential::presentation::PresentationBuilder;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_credential::validator::ValidatorDocument;
use identity_did::BaseDIDUrl;
use identity_did::CoreDID;
use identity_did::Error as DIDError;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_document::document::DocumentBuilder;

use crate::Error as ResolverError;
use crate::ErrorCause;
use crate::ResolutionAction;
use crate::Resolver;

/// A very simple handler
async fn mock_handler(did: CoreDID) -> std::result::Result<CoreDocument, std::io::Error> {
  Ok(core_document(did))
}

/// Create a [`CoreDocument`]
fn core_document(did: CoreDID) -> CoreDocument {
  DocumentBuilder::default().id(did).build().unwrap()
}

// make a simple self issued credential with the given did
fn make_credential(did: impl DID) -> Credential {
  CredentialBuilder::default()
    .issuer(did.to_url())
    .subject(Subject::with_id(did.to_url().into()))
    .build()
    .unwrap()
}

// make a simple presentation consisting of the given credentials
fn make_presentation(credentials: impl Iterator<Item = Credential>, holder: CoreDID) -> Presentation {
  let mut builder = PresentationBuilder::default().holder(holder.to_url().into());
  for credential in credentials {
    builder = builder.credential(credential);
  }
  builder.build().unwrap()
}

/// Checks that all methods on the resolver involving resolution fail under the assumption that
/// the resolver is set up in such a way that the `resolve` method must fail for this DID (because of a specific
/// cause), but succeed with `good_did`. The `assertions` argument is a function or closure that asserts that the
/// [`ErrorCause`] is of the expected value.
async fn check_failure_for_all_methods<F, D, DOC>(resolver: Resolver<DOC>, bad_did: CoreDID, good_did: D, assertions: F)
where
  F: Fn(ErrorCause),
  D: DID,
  DOC: AsRef<CoreDocument> + Send + Sync + 'static + From<CoreDocument>,
{
  // resolving bad_did fails
  let err: ResolverError = resolver.resolve(&bad_did).await.unwrap_err();
  assertions(err.into_error_cause());

  // resolving the issuer of the bad credential fails
  let cred: Credential = make_credential(bad_did.clone());

  let err: ResolverError = resolver.resolve_credential_issuer(&cred).await.unwrap_err();
  assertions(err.into_error_cause());

  // set up a presentation of the form: holder: bad_did , verifiableCredential: [good_credential, bad_credential,
  // good_credential] , other stuff irrelevant for this test.
  let other_credential: Credential = make_credential(good_did.clone());
  let presentation: Presentation = make_presentation(
    [other_credential.clone(), cred, other_credential.clone()].into_iter(),
    bad_did.clone(),
  );

  // resolving the holder of the presentation fails
  let err: ResolverError = resolver.resolve_presentation_holder(&presentation).await.unwrap_err();
  assert!(err.action().unwrap() == ResolutionAction::PresentationHolderResolution);
  assertions(err.into_error_cause());

  //resolving the presentation issuers will fail because of the bad credential at position 1.
  let err: ResolverError = resolver.resolve_presentation_issuers(&presentation).await.unwrap_err();
  assert!(err.action().unwrap() == ResolutionAction::PresentationIssuersResolution(1));
  assertions(err.into_error_cause());
  // check that our expectations are also matched when calling `verify_presentation`.

  // this can be passed as an argument to `verify_presentation` to avoid resolution of the holder or issuers.
  let mock_document: DOC = core_document(bad_did.clone()).into();
  let err: ResolverError = resolver
    .verify_presentation(
      &presentation,
      &PresentationValidationOptions::default(),
      FailFast::FirstError,
      Some(&mock_document),
      None,
    )
    .await
    .unwrap_err();

  assert!(err.action().unwrap() == ResolutionAction::PresentationIssuersResolution(1));
  assertions(err.into_error_cause());

  let err: ResolverError = resolver
    .verify_presentation(
      &presentation,
      &PresentationValidationOptions::default(),
      FailFast::FirstError,
      None,
      Some(std::slice::from_ref(&mock_document)),
    )
    .await
    .unwrap_err();

  assert!(err.action().unwrap() == ResolutionAction::PresentationHolderResolution);
  assertions(err.into_error_cause());

  // finally when both holder and issuer needs to be resolved we check that resolution fails
  let err: ResolverError = resolver
    .verify_presentation(
      &presentation,
      &PresentationValidationOptions::default(),
      FailFast::FirstError,
      None,
      None,
    )
    .await
    .unwrap_err();

  assert!(matches!(
    err.action().unwrap(),
    ResolutionAction::PresentationIssuersResolution(..) | ResolutionAction::PresentationHolderResolution
  ));

  assertions(err.into_error_cause());
}

// ===========================================================================
// Missing handler for DID method failure tests
// ===========================================================================
#[tokio::test]
async fn missing_handler_errors() {
  let method_name: String = "foo".to_owned();
  let bad_did: CoreDID = CoreDID::parse(format!("did:{method_name}:1234")).unwrap();
  let other_method: String = "bar".to_owned();
  let good_did: CoreDID = CoreDID::parse(format!("did:{other_method}:1234")).unwrap();
  // configure `resolver` to resolve the "bar" method
  let mut resolver: Resolver = Resolver::new();
  let mut resolver_core: Resolver<CoreDocument> = Resolver::new();
  resolver.attach_handler(other_method.clone(), mock_handler);
  resolver_core.attach_handler(other_method, mock_handler);

  // to avoid boiler plate
  let check_match = move |err| match err {
    ErrorCause::UnsupportedMethodError { method } => {
      assert_eq!(method_name, method);
    }
    _ => unreachable!(),
  };

  check_failure_for_all_methods(resolver, bad_did.clone(), good_did.clone(), check_match.clone()).await;
  check_failure_for_all_methods(resolver_core, bad_did, good_did, check_match).await;
}

// ===========================================================================
// DID Parsing failure tests
// ===========================================================================

// Implement the DID trait for a new type
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct FooDID(CoreDID);

impl FooDID {
  const METHOD_ID_LENGTH: usize = 5;

  fn try_from_core(did: CoreDID) -> std::result::Result<Self, DIDError> {
    Some(did)
      .filter(|did| did.method() == "foo" && did.method_id().len() == FooDID::METHOD_ID_LENGTH)
      .map(Self)
      .ok_or(DIDError::InvalidMethodName)
  }
}

impl AsRef<CoreDID> for FooDID {
  fn as_ref(&self) -> &CoreDID {
    &self.0
  }
}

impl From<FooDID> for String {
  fn from(did: FooDID) -> Self {
    String::from(did.0)
  }
}

impl FromStr for FooDID {
  type Err = DIDError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    CoreDID::from_str(s).and_then(Self::try_from_core)
  }
}

impl TryFrom<BaseDIDUrl> for FooDID {
  type Error = DIDError;
  fn try_from(value: BaseDIDUrl) -> Result<Self, Self::Error> {
    CoreDID::try_from(value).and_then(Self::try_from_core)
  }
}

impl<'a> TryFrom<&'a str> for FooDID {
  type Error = DIDError;
  fn try_from(value: &'a str) -> Result<Self, Self::Error> {
    CoreDID::try_from(value).and_then(Self::try_from_core)
  }
}

#[tokio::test]
async fn resolve_unparsable() {
  let mut resolver: Resolver = Resolver::new();
  let mut resolver_core: Resolver<CoreDocument> = Resolver::new();

  // register a handler that wants `did` to be of type `FooDID`.
  async fn handler(did: FooDID) -> std::result::Result<CoreDocument, std::io::Error> {
    mock_handler(did.as_ref().clone()).await
  }

  resolver.attach_handler("foo".to_owned(), handler);
  resolver_core.attach_handler("foo".to_owned(), handler);

  let bad_did: CoreDID = CoreDID::parse("did:foo:1234").unwrap();
  // ensure that the DID we created does not satisfy the requirements of the "foo" method
  assert!(bad_did.method_id().len() < FooDID::METHOD_ID_LENGTH);

  let good_did: FooDID = FooDID::try_from("did:foo:12345").unwrap();

  let error_matcher = |err: ErrorCause| {
    assert!(matches!(
      err
        .source()
        .unwrap()
        .downcast_ref::<DIDError>()
        .unwrap_or_else(|| panic!("{:?}", &err)),
      &DIDError::InvalidMethodName
    ));

    match err {
      ErrorCause::DIDParsingError { .. } => {}
      _ => unreachable!(),
    }
  };

  check_failure_for_all_methods(resolver, bad_did.clone(), good_did.clone(), error_matcher).await;
  check_failure_for_all_methods(resolver_core, bad_did, good_did, error_matcher).await;
}

// ===========================================================================
// Failing handler tests
// ===========================================================================

#[tokio::test]
async fn handler_failure() {
  #[derive(Debug, thiserror::Error)]
  #[error("resolution failed")]
  struct ResolutionError;
  async fn failing_handler(_did: CoreDID) -> std::result::Result<CoreDocument, ResolutionError> {
    Err(ResolutionError)
  }

  let mut resolver: Resolver = Resolver::new();
  let mut resolver_core: Resolver<CoreDocument> = Resolver::new();
  resolver.attach_handler("foo".to_owned(), failing_handler);
  resolver_core.attach_handler("foo".to_owned(), failing_handler);

  let bad_did: CoreDID = CoreDID::parse("did:foo:1234").unwrap();
  let good_did: CoreDID = CoreDID::parse("did:bar:1234").unwrap();
  resolver.attach_handler(good_did.method().to_owned(), mock_handler);
  resolver_core.attach_handler(good_did.method().to_owned(), mock_handler);

  // to avoid boiler plate
  let error_matcher = |err: ErrorCause| {
    assert!(err.source().unwrap().downcast_ref::<ResolutionError>().is_some());
    match err {
      ErrorCause::HandlerError { .. } => {}
      _ => unreachable!(),
    }
  };

  check_failure_for_all_methods(resolver, bad_did.clone(), good_did.clone(), error_matcher).await;
  check_failure_for_all_methods(resolver_core, bad_did, good_did, error_matcher).await;
}
