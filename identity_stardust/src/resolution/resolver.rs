// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use identity_credential::{
  credential::Credential,
  presentation::Presentation,
  validator::{FailFast, PresentationValidationOptions, PresentationValidator},
};
use identity_did::{
  did::DID,
  document::{CoreDocument, Document},
};
use serde::Serialize;

use crate::Result;
use identity_credential::validator::ValidatorDocument;

use super::Resolve;

pub struct TBA;
pub struct Resolver {
  delegates: HashMap<String, Box<dyn ValidatorDocument>>,
}

impl Resolver {
  pub fn resolve<Doc, D>(did: D) -> Result<Doc>
  where
    Doc: Document<D = D>,
  {
    todo!()
  }

  pub async fn resolve_presentation_issuers<U, V>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<Vec<Box<dyn ValidatorDocument>>> {
    todo!()
  }

  pub async fn resolve_presentation_issuers_core<U, V>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<Vec<CoreDocument>> {
    todo!()
  }

  pub async fn resolve_presentation_holder<U, V>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<Box<dyn ValidatorDocument>> {
    todo!()
  }

  pub async fn resolve_presentation_holder_core<U, V>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<CoreDocument> {
    todo!()
  }

  /// Fetches the DID Document of the issuer on a [`Credential`].
  ///
  /// # Errors
  ///
  /// Errors if the issuer URL is not a valid [`IotaDID`] or DID resolution fails.
  pub async fn resolve_credential_issuer<U: Serialize>(
    &self,
    credential: &Credential<U>,
  ) -> Result<Box<dyn ValidatorDocument>> {
    todo!()
  }

  pub async fn resolve_credential_issuer_core<U: Serialize>(&self, credential: &Credential<U>) -> Result<CoreDocument> {
    todo!()
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
}
