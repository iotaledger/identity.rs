// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use identity_credential::{
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

pub struct TBA;
pub struct Resolver {
  delegates: HashMap<String, TBA>,
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
