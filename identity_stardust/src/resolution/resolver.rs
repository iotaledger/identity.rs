// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

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

use super::resolve::ResolveValidator;

pub struct Resolver {
  method_map: HashMap<String, Box<dyn ResolveValidator>>,
}

impl Resolver {
  //TODO: Improve error handling.
  pub async fn resolve<DOC, D>(&self, did: &D) -> Result<DOC>
  where
    DOC: Document<D = D> + 'static,
    D: DID,
  {
    let delegate = self
      .method_map
      .get(did.method())
      .ok_or(Error::ResolutionProblem("did method not supported".into()))?;

    let validator_doc = delegate.resolve_validator(did.as_str()).await?;

    validator_doc
      .into_any()
      .downcast::<DOC>()
      .map(|boxed| *boxed)
      .map_err(|_| Error::ResolutionProblem("failed to convert the resolved document to the desired type".into()))
  }

  //TODO: Improve error handling.
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
  /// Errors if the issuer URL cannot be parsed to a DID with a method resolver attached, or resolution itself fails.
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
      .ok_or(Error::ResolutionProblem("did method not supported".into()))?;

    delegate
      .resolve_validator(did.as_str())
      .await
      .map(|value| value.into_validator_document())
  }
}
