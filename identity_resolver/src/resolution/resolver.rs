// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::future::Future;
use std::collections::HashMap;
use std::marker::PhantomData;

use futures::TryFutureExt;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_credential::validator::AbstractThreadSafeValidatorDocument;
use identity_credential::validator::CredentialValidator;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_credential::validator::PresentationValidator;
use identity_credential::validator::ValidatorDocument;
use identity_did::did::CoreDID;
use identity_did::did::DID;

use serde::Serialize;

use crate::Error;

use crate::Result;

use super::commands::Command;
use super::commands::SendSyncCommand;
use super::commands::SingleThreadedCommand;

/// Convenience type for resolving DID documents from different DID methods.   
///  
/// Also provides methods for resolving DID Documents associated with
/// verifiable [`Credentials`][Credential] and [`Presentations`][Presentation].
///
/// # Configuration
/// The resolver will only be able to resolve DID documents for methods it has been configured for. This is done by
/// attaching method specific handlers with [`Self::attach_handler`](Self::attach_handler()).
pub struct Resolver<DOC = AbstractThreadSafeValidatorDocument, CMD = SendSyncCommand<DOC>>
where
  CMD: for<'r> Command<'r, Result<DOC>>,
  DOC: ValidatorDocument,
{
  command_map: HashMap<String, CMD>,
  _required: PhantomData<DOC>,
}

impl<M, DOC> Resolver<DOC, M>
where
  M: for<'r> Command<'r, Result<DOC>>,
  DOC: ValidatorDocument,
{
  /// Constructs a new [`Resolver`].
  ///
  /// # Example
  /// Construct a `Resolver` that resolves DID documents of type
  /// [`CoreDocument`](::identity_did::document::CoreDocument).
  ///  ```
  /// # use identity_resolver::Resolver;
  /// # use identity_did::document::CoreDocument;
  ///
  /// let mut resolver = Resolver::<CoreDocument>::new();
  /// // Now attach some handlers whose output can be converted to a `CoreDocument`.
  /// ```
  /// 
  /// # Example
  /// Construct a `Resolver` that is agnostic about DID Document types.
  /// ```
  /// # use identity_resolver::Resolver;
  /// let mut resolver: Resolver = Resolver::new();
  /// // Now attach some handlers whose output type implements the `Document` trait.
  /// ```
  pub fn new() -> Self {
    Self {
      command_map: HashMap::new(),
      _required: PhantomData::<DOC>,
    }
  }

  /// Fetches the DID Document of the given DID.
  ///
  /// # Errors
  /// Errors if the resolver has not been configured to handle the method corresponding to the given DID or the
  /// resolution process itself fails.
  ///
  /// # When the return type is abstract
  /// If the resolver has been constructed without specifying the return type (in [`Self::new`](Self::new()))
  /// one can call [`into_any`](AbstractThreadSafeValidatorDocument::into_any()) on the resolved output to obtain a
  /// [`Box<dyn Any>`] which one may then attempt to downcast to a concrete type. The downcast will succeed if the
  /// specified type matches the return type of the attached handler responsible for resolving the given DID.
  ///
  /// ## Example
  /// ```
  /// # use identity_resolver::Resolver;
  /// # use identity_did::did::CoreDID;
  /// # use identity_did::did::DID;
  /// # use identity_did::document::CoreDocument;
  ///
  /// async fn resolve_and_cast(
  ///   did: CoreDID,
  /// ) -> std::result::Result<Option<CoreDocument>, Box<dyn std::error::Error>> {
  ///   let resolver: Resolver = configure_resolver(Resolver::new());
  ///   let abstract_doc = resolver.resolve(&did).await?;
  ///   let document: Option<CoreDocument> = abstract_doc
  ///     .into_any()
  ///     .downcast::<CoreDocument>()
  ///     .ok()
  ///     .map(|boxed| *boxed);
  ///   if did.method() == "foo" {
  ///     assert!(document.is_some());
  ///   }
  ///   return Ok(document);
  /// }
  ///
  /// fn configure_resolver(mut resolver: Resolver) -> Resolver {
  ///   resolver.attach_handler("foo".to_owned(), resolve_foo);
  ///   // TODO: attach handlers for other DID methods we are interested in.
  ///   resolver
  /// }
  ///
  /// async fn resolve_foo(did: CoreDID) -> std::result::Result<CoreDocument, std::io::Error> {
  ///   todo!()
  /// }
  /// ```
  pub async fn resolve<D: DID>(&self, did: &D) -> Result<DOC> {
    self.delegate_resolution(did.method(), did.as_str()).await
  }

  /// Fetches all DID Documents of [`Credential`] issuers contained in a [`Presentation`].
  /// Issuer documents are returned in arbitrary order.
  ///
  /// # Errors
  ///
  /// Errors if any issuer URL cannot be parsed to a DID whose associated method is supported by this Resolver, or
  /// resolution fails.
  pub async fn resolve_presentation_issuers<U, V>(&self, presentation: &Presentation<U, V>) -> Result<Vec<DOC>> {
    // Extract unique issuers, but keep the position of one credential issued by said DID for each unique issuer.
    // The credential positions help us provide better errors.
    let issuers: HashMap<CoreDID, usize> = presentation
      .verifiable_credential
      .iter()
      .enumerate()
      .map(|(idx, credential)| {
        CredentialValidator::extract_issuer::<CoreDID, V>(credential)
          .map_err(|error| Error::DIDParsingError {
            source: error.into(),
            context: crate::error::ResolutionAction::PresentationIssuersResolution(idx),
          })
          .map(|did| (did, idx))
      })
      .collect::<Result<_>>()?;

    // Resolve issuers concurrently.
    futures::future::try_join_all(
      issuers
        .iter()
        .map(|(issuer, cred_idx)| {
          self
            .delegate_resolution(issuer.method(), issuer.as_str())
            .map_err(|error| {
              Error::update_resolution_action(
                error,
                crate::error::ResolutionAction::PresentationIssuersResolution(*cred_idx),
              )
            })
        })
        .collect::<Vec<_>>(),
    )
    .await
  }

  /// Fetches the DID Document of the holder of a [`Presentation`].
  ///
  /// # Errors
  ///
  /// Errors if the holder URL is missing, cannot be parsed to a valid DID whose method is supported by the resolver, or
  /// DID resolution fails.
  pub async fn resolve_presentation_holder<U, V>(&self, presentation: &Presentation<U, V>) -> Result<DOC> {
    let holder: CoreDID =
      PresentationValidator::extract_holder(presentation).map_err(|error| Error::DIDParsingError {
        source: error.into(),
        context: crate::error::ResolutionAction::PresentationHolderResolution,
      })?;
    self
      .delegate_resolution(holder.method(), holder.as_str())
      .await
      .map_err(|error| {
        Error::update_resolution_action(error, crate::error::ResolutionAction::PresentationHolderResolution)
      })
  }

  /// Fetches the DID Document of the issuer of a [`Credential`].
  ///
  /// # Errors
  ///
  /// Errors if the issuer URL cannot be parsed to a DID whose associated method is supported by the resolver, or
  /// resolution fails.
  pub async fn resolve_credential_issuer<U: Serialize>(&self, credential: &Credential<U>) -> Result<DOC> {
    let issuer_did: CoreDID =
      CredentialValidator::extract_issuer(credential).map_err(|error| Error::DIDParsingError {
        source: error.into(),
        context: crate::error::ResolutionAction::CredentialIssuerResolution,
      })?;
    self
      .delegate_resolution(issuer_did.method(), issuer_did.as_str())
      .await
      .map_err(|error| {
        Error::update_resolution_action(error, crate::error::ResolutionAction::CredentialIssuerResolution)
      })
  }

  /// Verifies a [`Presentation`].
  ///
  /// # Important
  /// See [`PresentationValidator::validate`](PresentationValidator::validate()) for information about which properties
  /// get validated and what is expected of the optional arguments `holder` and `issuer`.
  ///
  /// # Resolution
  /// The DID Documents for the `holder` and `issuers` are optionally resolved if not given.
  /// If you already have up-to-date versions of these DID Documents, you may want
  /// to use [`PresentationValidator::validate`].
  /// See also [`Resolver::resolve_presentation_issuers`] and [`Resolver::resolve_presentation_holder`]. Note that
  /// DID Documents of a certain method can only be resolved if the resolver has been configured handle this method.
  /// See [`Self::attach_handler`].
  ///
  /// # Errors
  /// Errors from resolving the holder and issuer DID Documents, if not provided, will be returned immediately.
  /// Otherwise, errors from validating the presentation and its credentials will be returned
  /// according to the `fail_fast` parameter.
  pub async fn verify_presentation<U, V, HDOC, IDOC>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    fail_fast: FailFast,
    holder: Option<&HDOC>,
    issuers: Option<&[IDOC]>,
  ) -> Result<()>
  where
    U: Serialize,
    V: Serialize,
    HDOC: ValidatorDocument + ?Sized,
    IDOC: ValidatorDocument,
  {
    match (holder, issuers) {
      (Some(holder), Some(issuers)) => {
        PresentationValidator::validate(presentation, holder, issuers, options, fail_fast)
          .map_err(|error| Error::PresentationValidationError { source: error })
      }
      (Some(holder), None) => {
        let issuers: Vec<DOC> = self.resolve_presentation_issuers(presentation).await?;
        PresentationValidator::validate(presentation, holder, issuers.as_slice(), options, fail_fast)
          .map_err(|error| Error::PresentationValidationError { source: error })
      }
      (None, Some(issuers)) => {
        let holder = self.resolve_presentation_holder(presentation).await?;
        PresentationValidator::validate(presentation, &holder, issuers, options, fail_fast)
          .map_err(|error| Error::PresentationValidationError { source: error })
      }
      (None, None) => {
        let (holder, issuers): (DOC, Vec<DOC>) = futures::future::try_join(
          self.resolve_presentation_holder(presentation),
          self.resolve_presentation_issuers(presentation),
        )
        .await?;

        PresentationValidator::validate(presentation, &holder, &issuers, options, fail_fast)
          .map_err(|error| Error::PresentationValidationError { source: error })
      }
    }
    .map_err(Into::into)
  }

  /// Delegates Resolution to the relevant attached handler.
  ///
  /// The first input parameters `method` and `did` must be &str representations of the DID method name and the DID
  /// respectively.  
  async fn delegate_resolution(&self, method: &str, did: &str) -> Result<DOC> {
    let delegate = self
      .command_map
      .get(method)
      .ok_or_else(|| Error::UnsupportedMethodError {
        method: method.to_owned(),
        context: crate::error::ResolutionAction::Unknown,
      })?;

    delegate.apply(did).await
  }
}

impl<DOC: ValidatorDocument + Send + Sync + 'static> Resolver<DOC> {
  /// Attach a new handler responsible for resolving DIDs of the given DID method.
  ///
  /// The `handler` is expected to be a closure taking an owned DID and asynchronously returning a DID Document
  /// which can be converted to the type this [`Resolver`] is parametrized over. The `handler` is required to be
  /// [`Clone`], [`Send`], [`Sync`] and `'static` hence all captured variables must satisfy these bounds. In this regard
  /// the `move` keyword and (possibly) wrapping values in an [`Arc`](std::sync::Arc) may come in handy (see the example
  /// below).
  ///
  /// NOTE: If there already exists a handler for this method then it will be replaced with the new handler.
  /// In the case where one would like to have a "backup handler" for the same DID method, one can achieve this with
  /// composition.
  ///
  /// # Example
  /// ```
  /// # use identity_resolver::Resolver;
  /// # use identity_did::did::CoreDID;
  /// # use identity_did::document::CoreDocument;
  ///
  ///    // A client that can resolve DIDs of our invented "foo" method.
  ///    struct Client;
  ///
  ///    impl Client {
  ///      // Resolves some of the DIDs we are interested in.
  ///      async fn resolve(&self, _did: &CoreDID) -> std::result::Result<CoreDocument, std::io::Error> {
  ///        todo!()
  ///      }
  ///    }
  ///
  ///    // This way we can essentially produce (cheap) clones of our client.
  ///    let client = std::sync::Arc::new(Client {});
  ///
  ///    // Get a clone we can move into a handler.
  ///    let client_clone = client.clone();
  ///
  ///    // Construct a resolver that resolves documents of type `CoreDocument`.
  ///    let mut resolver = Resolver::<CoreDocument>::new();
  ///
  ///    // Now we want to attach a handler that uses the client to resolve DIDs whose method is "foo".
  ///    resolver.attach_handler("foo".to_owned(), move |did: CoreDID| {
  ///      // We want to resolve the did asynchronously, but since we do not know when it will be awaited we
  ///      // let the future take ownership of the client by moving a clone into the asynchronous block.
  ///      let future_client = client_clone.clone();
  ///      async move { future_client.resolve(&did).await }
  ///    });
  /// ```
  pub fn attach_handler<D, F, Fut, DOCUMENT, E, DIDERR>(&mut self, method: String, handler: F)
  where
    D: DID + Send + for<'r> TryFrom<&'r str, Error = DIDERR> + Sync + 'static,
    DOCUMENT: 'static + Into<DOC>,
    F: Fn(D) -> Fut + 'static + Clone + Send + Sync,
    Fut: Future<Output = std::result::Result<DOCUMENT, E>> + Send + Sync,
    E: std::error::Error + Send + Sync + 'static,
    DIDERR: std::error::Error + Send + Sync + 'static,
  {
    let command = SendSyncCommand::new(handler);
    self.command_map.insert(method, command);
  }
}

impl<DOC: ValidatorDocument + 'static> Resolver<DOC, SingleThreadedCommand<DOC>> {
  /// Attach a new handler responsible for resolving DIDs of the given DID method.
  ///
  /// The `handler` is expected to be a closure taking an owned DID and asynchronously returning a DID Document
  /// which can be converted to the type this [`Resolver`] is parametrized over. The `handler` is required to be
  /// [`Clone`] and `'static`  hence all captured variables must satisfy these bounds. In this regard the
  /// `move` keyword and (possibly) wrapping values in an [`std::rc::Rc`] may come in handy (see the example below).
  ///
  /// NOTE: If there already exists a handler for this method then it will be replaced with the new handler.
  /// In the case where one would like to have a "backup handler" for the same DID method, one can achieve this with
  /// composition.
  ///
  /// # Example
  /// ```
  /// # use identity_resolver::SingleThreadedResolver;
  /// # use identity_did::did::CoreDID;
  /// # use identity_did::document::CoreDocument;
  ///
  ///    // A client that can resolve DIDs of our invented "foo" method.
  ///    struct Client;
  ///
  ///    impl Client {
  ///      // Resolves some of the DIDs we are interested in.
  ///      async fn resolve(&self, _did: &CoreDID) -> std::result::Result<CoreDocument, std::io::Error> {
  ///        todo!()
  ///      }
  ///    }
  ///
  ///    // This way we can essentially produce (cheap) clones of our client.
  ///    let client = std::rc::Rc::new(Client {});
  ///
  ///    // Get a clone we can move into a handler.
  ///    let client_clone = client.clone();
  ///
  ///    // Construct a resolver that resolves documents of type `CoreDocument`.
  ///    let mut resolver = SingleThreadedResolver::<CoreDocument>::new();
  ///
  ///    // Now we want to attach a handler that uses the client to resolve DIDs whose method is "foo".
  ///    resolver.attach_handler("foo".to_owned(), move |did: CoreDID| {
  ///      // We want to resolve the did asynchronously, but since we do not know when it will be awaited we
  ///      // let the future take ownership of the client by moving a clone into the asynchronous block.
  ///      let future_client = client_clone.clone();
  ///      async move { future_client.resolve(&did).await }
  ///    });
  /// ```
  pub fn attach_handler<D, F, Fut, DOCUMENT, E, DIDERR>(&mut self, method: String, handler: F)
  where
    D: DID + for<'r> TryFrom<&'r str, Error = DIDERR> + 'static,
    DOCUMENT: 'static + Into<DOC>,
    F: Fn(D) -> Fut + 'static + Clone,
    Fut: Future<Output = std::result::Result<DOCUMENT, E>>,
    E: std::error::Error + Send + Sync + 'static,
    DIDERR: std::error::Error + Send + Sync + 'static,
  {
    let command = SingleThreadedCommand::new(handler);
    self.command_map.insert(method, command);
  }
}

#[cfg(test)]
mod tests {
  use std::error::Error;
  use std::str::FromStr;

  use crate::ResolutionAction;

  use super::Credential;
  use super::Error as ResolverError;
  use super::Presentation;
  use super::Resolver;
  use super::ValidatorDocument;
  use super::DID;
  use identity_credential::credential::CredentialBuilder;
  use identity_credential::credential::Subject;
  use identity_credential::presentation::PresentationBuilder;
  use identity_credential::validator::FailFast;
  use identity_credential::validator::PresentationValidationOptions;
  use identity_did::did::BaseDIDUrl;
  use identity_did::did::CoreDID;
  use identity_did::did::DIDError;
  use identity_did::document::CoreDocument;
  use identity_did::document::DocumentBuilder;

  fn credential<D: DID>(did: D) -> Credential {
    CredentialBuilder::default()
      .issuer(did.to_url())
      .subject(Subject::with_id(did.to_url().into()))
      .build()
      .unwrap()
  }

  fn presentation(credentials: impl Iterator<Item = Credential>, holder: CoreDID) -> Presentation {
    let mut builder = PresentationBuilder::default().holder(holder.to_url().into());
    for credential in credentials {
      builder = builder.credential(credential);
    }
    builder.build().unwrap()
  }

  async fn mock_handler(did: CoreDID) -> std::result::Result<CoreDocument, std::io::Error> {
    Ok(core_document(did))
  }

  fn core_document(did: CoreDID) -> CoreDocument {
    DocumentBuilder::default().id(did).build().unwrap()
  }

  #[allow(dead_code)]
  fn is_send_sync<T: Send + Sync>(_t: T) {}

  #[allow(dead_code)]
  fn resolver_methods_give_send_sync_futures<DOC>()
  where
    DOC: ValidatorDocument + Send + Sync + 'static,
  {
    let did: CoreDID = "did:key:4353526346363sdtsdfgdfg".parse().unwrap();
    let credential: Credential = credential(did.clone());
    let presentation: Presentation = presentation(std::iter::once(credential.clone()), did.clone());

    let resolver = Resolver::<DOC>::new();
    is_send_sync(resolver.resolve(&did));

    is_send_sync(resolver.resolve_credential_issuer(&credential));

    is_send_sync(resolver.resolve_presentation_holder(&presentation));

    is_send_sync(resolver.resolve_presentation_issuers(&presentation));

    is_send_sync(resolver.verify_presentation(
      &presentation,
      &PresentationValidationOptions::default(),
      FailFast::FirstError,
      Option::<&DOC>::None,
      Option::<&[DOC]>::None,
    ));
  }

  /// Checks that all methods on the resolver involving resolution fail under the assumption that
  /// the resolver is set up in such a way that the `resolve` method must fail for this did, but succeed with
  /// `good_did`. The error_matcher is a closure that asserts that the error is of the expected value.  
  async fn check_failure_for_all_methods<F, D>(resolver: Resolver, bad_did: CoreDID, good_did: D, error_matcher: F)
  where
    F: Fn(ResolverError, fn(ResolutionAction) -> bool) -> (),
    D: DID,
  {
    // resolving bad_did fails
    let err: ResolverError = resolver.resolve(&bad_did).await.unwrap_err();
    error_matcher(err, |_| true);

    // resolving the issuer of the bad credential fails
    let cred: Credential = credential(bad_did.clone());

    let err: ResolverError = resolver.resolve_credential_issuer(&cred).await.unwrap_err();
    error_matcher(err, |value| value == ResolutionAction::CredentialIssuerResolution);

    // set up a presentation of the form: holder: bad_did , verifiableCredential: [good_credential, bad_credential,
    // good_credential] , other stuff irrelevant for this test.
    let other_credential: Credential = credential(good_did.clone());
    let presentation: Presentation = presentation(
      [other_credential.clone(), cred, other_credential.clone()].into_iter(),
      bad_did.clone(),
    );

    // resolving the holder of the presentation fails
    let err: ResolverError = resolver.resolve_presentation_holder(&presentation).await.unwrap_err();
    error_matcher(err, |value| value == ResolutionAction::PresentationHolderResolution);

    //resolving the presentation issuers will fail because of the bad credential at position 1.
    let err: ResolverError = resolver.resolve_presentation_issuers(&presentation).await.unwrap_err();
    error_matcher(err, |value| value == ResolutionAction::PresentationIssuersResolution(1));

    // check that our expectations are also matched when calling `verify_presentation`.

    // this can be passed as an argument to `verify_presentation` to avoid resolution of the holder or issuers.
    let mock_document: CoreDocument = core_document(bad_did.clone());
    let err: ResolverError = resolver
      .verify_presentation(
        &presentation,
        &PresentationValidationOptions::default(),
        FailFast::FirstError,
        Some(&mock_document),
        Option::<&[CoreDocument]>::None,
      )
      .await
      .unwrap_err();

    error_matcher(err, |value| value == ResolutionAction::PresentationIssuersResolution(1));

    let err: ResolverError = resolver
      .verify_presentation(
        &presentation,
        &PresentationValidationOptions::default(),
        FailFast::FirstError,
        Option::<&CoreDocument>::None,
        Some(std::slice::from_ref(&mock_document)),
      )
      .await
      .unwrap_err();

    error_matcher(err, |value| value == ResolutionAction::PresentationHolderResolution);

    // finally when both holder and issuer needs to be resolved we check that resolution fails
    let err: ResolverError = resolver
      .verify_presentation(
        &presentation,
        &PresentationValidationOptions::default(),
        FailFast::FirstError,
        Option::<&CoreDocument>::None,
        Option::<&[CoreDocument]>::None,
      )
      .await
      .unwrap_err();

    error_matcher(err, |value| {
      matches!(
        value,
        ResolutionAction::PresentationIssuersResolution(..) | ResolutionAction::PresentationHolderResolution
      )
    });
  }
  // ===========================================================================
  // Missing handler for DID method failure tests
  // ===========================================================================
  #[tokio::test]
  async fn missing_handler_errors() {
    let method_name: String = "foo".to_owned();
    let bad_did: CoreDID = CoreDID::parse(&format!("did:{method_name}:1234")).unwrap();
    let other_method: String = "bar".to_owned();
    let good_did: CoreDID = CoreDID::parse(format!("did:{other_method}:1234")).unwrap();
    // configure `resolver` to resolve the "bar" method
    let mut resolver: Resolver = Resolver::new();
    resolver.attach_handler(other_method, mock_handler);

    // to avoid boiler plate
    let check_match = move |err, comp: fn(ResolutionAction) -> bool| match err {
      ResolverError::UnsupportedMethodError { method, context } => {
        assert_eq!(method_name, method);
        assert!(comp(context));
      }
      _ => unreachable!(),
    };

    check_failure_for_all_methods(resolver, bad_did, good_did, check_match).await;
  }

  // ===========================================================================
  // DID Parsing failure tests
  // ===========================================================================

  // Implement the DID trait for a new type
  #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
  struct FooDID(CoreDID);
  impl FooDID {
    const METHOD_ID_LENGTH: usize = 5;
  }
  impl FooDID {
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

    // register a handler that wants `did` to be of type `FooDID`.
    async fn handler(did: FooDID) -> std::result::Result<CoreDocument, std::io::Error> {
      mock_handler(did.as_ref().clone()).await
    }

    resolver.attach_handler("foo".to_owned(), handler);

    let bad_did: CoreDID = CoreDID::parse("did:foo:1234").unwrap();
    // ensure that the DID we created does not satisfy the requirements of the "foo" method
    assert!(bad_did.method_id().len() < FooDID::METHOD_ID_LENGTH);

    let good_did: FooDID = FooDID::try_from("did:foo:12345").unwrap();

    let error_matcher = |err: ResolverError, comp: fn(ResolutionAction) -> bool| {
      assert!(matches!(
        err
          .source()
          .unwrap()
          .downcast_ref::<DIDError>()
          .expect(&format!("{:?}", &err)),
        &DIDError::InvalidMethodName
      ));

      match err {
        ResolverError::DIDParsingError { context, .. } => {
          assert!(comp(context));
        }
        _ => unreachable!(),
      }
    };

    check_failure_for_all_methods(resolver, bad_did, good_did, error_matcher).await;
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
    resolver.attach_handler("foo".to_owned(), failing_handler);

    let bad_did: CoreDID = CoreDID::parse(&format!("did:foo:1234")).unwrap();
    let good_did: CoreDID = CoreDID::parse(format!("did:bar:1234")).unwrap();
    resolver.attach_handler(good_did.method().to_owned(), mock_handler);

    // to avoid boiler plate
    let error_matcher = |err: ResolverError, comp: fn(ResolutionAction) -> bool| {
      assert!(err.source().unwrap().downcast_ref::<ResolutionError>().is_some());
      match err {
        ResolverError::HandlerError { context, .. } => {
          assert!(comp(context));
        }
        _ => unreachable!(),
      }
    };

    check_failure_for_all_methods(resolver, bad_did, good_did, error_matcher).await;
  }
}
