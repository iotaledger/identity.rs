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
use crate::ErrorCause;

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
    let method = did.method();
    let delegate = self
      .command_map
      .get(method)
      .ok_or_else(|| ErrorCause::UnsupportedMethodError {
        method: method.to_owned(),
      })
      .map_err(Error::new)?;

    delegate.apply(did.as_str()).await
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
          .map_err(|error| ErrorCause::DIDParsingError { source: error.into() })
          .map_err(Error::new)
          .map_err(|error| {
            Error::update_resolution_action(
              error,
              crate::error::ResolutionAction::PresentationIssuersResolution(idx),
            )
          })
          .map(|did| (did, idx))
      })
      .collect::<Result<_>>()?;

    // Resolve issuers concurrently.
    futures::future::try_join_all(
      issuers
        .iter()
        .map(|(issuer, cred_idx)| {
          self.resolve(issuer).map_err(|error| {
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
    let holder: CoreDID = PresentationValidator::extract_holder(presentation)
      .map_err(|error| ErrorCause::DIDParsingError { source: error.into() })
      .map_err(Error::new)
      .map_err(|error| {
        Error::update_resolution_action(error, crate::error::ResolutionAction::PresentationHolderResolution)
      })?;
    self.resolve(&holder).await.map_err(|error| {
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
    let issuer_did: CoreDID = CredentialValidator::extract_issuer(credential)
      .map_err(|error| ErrorCause::DIDParsingError { source: error.into() })
      .map_err(Error::new)?;
    self.resolve(&issuer_did).await
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
  pub async fn verify_presentation<U, V>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    fail_fast: FailFast,
    holder: Option<&DOC>,
    issuers: Option<&[DOC]>,
  ) -> Result<()>
  where
    U: Serialize,
    V: Serialize,
  {
    match (holder, issuers) {
      (Some(holder), Some(issuers)) => {
        PresentationValidator::validate(presentation, holder, issuers, options, fail_fast)
          .map_err(|error| ErrorCause::PresentationValidationError { source: error })
          .map_err(Error::new)
      }
      (Some(holder), None) => {
        let issuers: Vec<DOC> = self.resolve_presentation_issuers(presentation).await?;
        PresentationValidator::validate(presentation, holder, issuers.as_slice(), options, fail_fast)
          .map_err(|error| ErrorCause::PresentationValidationError { source: error })
          .map_err(Error::new)
      }
      (None, Some(issuers)) => {
        let holder = self.resolve_presentation_holder(presentation).await?;
        PresentationValidator::validate(presentation, &holder, issuers, options, fail_fast)
          .map_err(|error| ErrorCause::PresentationValidationError { source: error })
          .map_err(Error::new)
      }
      (None, None) => {
        let (holder, issuers): (DOC, Vec<DOC>) = futures::future::try_join(
          self.resolve_presentation_holder(presentation),
          self.resolve_presentation_issuers(presentation),
        )
        .await?;

        PresentationValidator::validate(presentation, &holder, &issuers, options, fail_fast)
          .map_err(|error| ErrorCause::PresentationValidationError { source: error })
          .map_err(Error::new)
      }
    }
    .map_err(Into::into)
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
    Fut: Future<Output = std::result::Result<DOCUMENT, E>> + Send,
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

#[cfg(feature = "iota")]
mod iota_handler {
  use super::Resolver;
  use identity_credential::validator::ValidatorDocument;
  use identity_stardust::StardustClientExt;
  use identity_stardust::StardustDID;
  use identity_stardust::StardustDocument;
  use identity_stardust::StardustIdentityClientExt;
  use std::sync::Arc;

  impl<DOC> Resolver<DOC>
  where
    DOC: From<StardustDocument> + ValidatorDocument + Send + Sync + 'static,
  {
    /// Convenience method for attaching a new handler responsible for resolving IOTA DIDs.
    ///
    /// See also [`attach_handler`](Self::attach_handler).
    pub fn attach_iota_handler<CLI>(&mut self, client: CLI)
    where
      CLI: StardustClientExt + Send + Sync + 'static,
    {
      let arc_client: Arc<CLI> = Arc::new(client);

      let handler = move |did: StardustDID| {
        let future_client = arc_client.clone();
        async move { future_client.resolve_did(&did).await }
      };

      self.attach_handler(StardustDID::METHOD.to_owned(), handler);
    }
  }
}
