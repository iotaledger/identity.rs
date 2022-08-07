// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use identity_credential::{
  credential::Credential,
  presentation::Presentation,
  validator::{CredentialValidator, FailFast, PresentationValidationOptions, PresentationValidator},
};
use identity_did::{
  did::{CoreDID, DID},
  document::Document,
};
use serde::Serialize;

use crate::{Error, Result};
use identity_credential::validator::ValidatorDocument;

use super::AbstractResolverDelegate;
 

/// Convenience type for resolving did documents from different did methods.   
///  
/// Also provides functions for resolving DID Documents associated with
/// verifiable [`Credentials`][Credential] and [`Presentations`][Presentation].
///
/// # Configuration
/// The resolver will only be able to resolve did documents corresponding to a certain method after it has been
/// configured to do so. This setup is achieved by implementing the [`MethodBoundedResolver` trait](super::MethodBoundResolver) for your client
/// and then attaching it with [`Self::attach_method_handler`](`Resolver::attach_method_handler`).
pub struct Resolver {
  method_map: HashMap<String, Arc<dyn AbstractResolutionHandler>>,
}

impl Resolver {
  /// Constructs a new [`Resolver`].
  pub fn new() -> Self {
    Self {
      method_map: HashMap::new(),
    }
  }

  /// Attach a [`ValidatorDocumentResolver`] to this resolver. If the resolver has previously been configured to handle the same DID method
  /// the new handler will replace the previous which will then returned from this method, otherwise `None` is returned.
  #[must_use]
  pub fn attach_method_handler(
    &mut self,
    handler: Arc<dyn AbstractResolutionHandler>,
  ) -> Option<Arc<dyn AbstractResolutionHandler>> {
    self.method_map.insert(handler.method(), handler)
  }

  /// Fetches the DID Document of the given DID.
  ///
  ///
  /// # Errors
  /// Errors if the resolver has not been configured to handle the method corresponding to the given did, resolution fails, or the
  /// resolved document is of another type than the specified [`Document`] implementor.  
  //TODO: Improve error handling.
  pub async fn resolve<DOC, D>(&self, did: &D) -> Result<DOC>
  where
    DOC: Document<D = D> + 'static,
    D: DID,
  {
    let delegate = self
      .method_map
      .get(did.method())
      .ok_or_else(|| Error::ResolutionProblem("did method not supported".into()))?;

    let validator_doc = delegate.resolve_validator(did.as_str()).await?;

    validator_doc
      .into_any()
      .downcast::<DOC>()
      .map(|boxed| *boxed)
      .map_err(|_| Error::ResolutionProblem("failed to convert the resolved document to the desired type".into()))
  }

  /// Fetches all DID Documents of [`Credential`] issuers contained in a [`Presentation`].
  /// Issuer documents are returned in arbitrary order.
  ///
  /// # Errors
  ///
  /// Errors if any issuer URL cannot be parsed to a DID whose associated method is supported by this Resolver, or resolution fails.
  // TODO: Improve error handling.
  pub async fn resolve_presentation_issuers<U, V>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<Vec<Box<dyn ValidatorDocument>>> {
    // Extract unique issuers.
    //TODO: Improve error handling.
    let issuers: HashSet<CoreDID> = presentation
      .verifiable_credential
      .iter()
      .map(|credential| {
        CredentialValidator::extract_issuer::<CoreDID, V>(credential)
          .map_err(|_| Error::ResolutionProblem("Failed to parse the issuer's did from a credential".into()))
      })
      .collect::<Result<_>>()?;

    if let Some(unsupported_method) = issuers
      .iter()
      .find(|issuer| !self.method_map.contains_key(issuer.method()))
      .map(|issuer| issuer.method())
    {
      // The presentation contains did's whose methods are not attached to this Resolver.
      // TODO: Find a much better error!
      return Err(Error::ResolutionProblem(format!(
        "the presentation contains a credential issued with the following unsupported did method: {}",
        unsupported_method
      )));
    }
    // Resolve issuers concurrently.
    futures::future::try_join_all(
      issuers
        .iter()
        .map(|issuer| self.resolve_validator_document(issuer))
        .collect::<Vec<_>>(),
    )
    .await
  }

  /// Fetches the DID Document of the holder of a [`Presentation`].
  ///
  /// # Errors
  ///
  /// Errors if the holder URL is missing, cannot be parsed to a valid DID whose method is supported by the resolver, or DID resolution fails.
  //TODO: Improve error handling
  pub async fn resolve_presentation_holder<U, V>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<Box<dyn ValidatorDocument>> {
    let holder: CoreDID = PresentationValidator::extract_holder(presentation)
      .map_err(|error| Error::ResolutionProblem(error.to_string()))?;
    self.resolve_validator_document(&holder).await
  }

  /// Fetches the DID Document of the issuer of a [`Credential`].
  ///
  /// # Errors
  ///
  /// Errors if the issuer URL cannot be parsed to a DID with a method supported by the resolver, or resolution fails.
  // TODO: Improve errors!
  pub async fn resolve_credential_issuer<U: Serialize>(
    &self,
    credential: &Credential<U>,
  ) -> Result<Box<dyn ValidatorDocument>> {
    let issuer_did: CoreDID = CredentialValidator::extract_issuer(credential)
      .map_err(|_| Error::ResolutionProblem("failed to parse the issuer's did".into()))?;
    self.resolve_validator_document(&issuer_did).await
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
  /// See also [`Resolver::resolve_presentation_issuers`] and [`Resolver::resolve_presentation_holder`].
  ///
  /// # Errors
  /// Errors from resolving the holder and issuer DID Documents, if not provided, will be returned immediately.
  /// Otherwise, errors from validating the presentation and its credentials will be returned
  /// according to the `fail_fast` parameter.
  pub async fn verify_presentation<U: Serialize, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    fail_fast: FailFast,
    holder: Option<&dyn ValidatorDocument>,
    issuers: Option<&[&dyn ValidatorDocument]>,
  ) -> Result<()> {
    match (holder, issuers) {
      (Some(holder), Some(issuers)) => {
        PresentationValidator::validate(presentation, &holder, issuers, options, fail_fast)
      }
      (Some(holder), None) => {
        let issuers: Vec<Box<dyn ValidatorDocument>> = self.resolve_presentation_issuers(presentation).await?;
        PresentationValidator::validate(presentation, &holder, issuers.as_slice(), options, fail_fast)
      }
      (None, Some(issuers)) => {
        let holder = self.resolve_presentation_holder(presentation).await?;
        PresentationValidator::validate(presentation, &holder, issuers, options, fail_fast)
      }
      (None, None) => {
        let (holder, issuers): (Box<dyn ValidatorDocument>, Vec<Box<dyn ValidatorDocument>>) =
          futures::future::try_join(
            self.resolve_presentation_holder(presentation),
            self.resolve_presentation_issuers(presentation),
          )
          .await?;

        PresentationValidator::validate(presentation, &holder, &issuers, options, fail_fast)
      }
    }
    .map_err(Into::into)
  }

  async fn resolve_validator_document(&self, did: &CoreDID) -> Result<Box<dyn ValidatorDocument>> {
    let delegate = self
      .method_map
      .get(did.method())
      .ok_or_else(|| Error::ResolutionProblem("did method not supported".into()))?;

    delegate.resolve_validator(did.as_str()).await
  }
}

impl Default for Resolver {
  fn default() -> Self {
    Self::new()
  }
}
