// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use crate::{Error, Result};
use async_trait::async_trait;
use identity_credential::validator::ValidatorDocument;
use identity_did::{
  did::{CoreDID, DIDError, DID},
  document::{CoreDocument, Document},
};
#[async_trait]
pub trait Resolve {
  type D: for<'a> TryFrom<&'a str> + DID;
  type DOC: Document<D = Self::D>;

  /// Fetch the associated DID Document from the given DID.
  async fn resolve(&self, did: &Self::D) -> Result<Self::DOC>;
}

pub trait ValidatorDocumentExt: ValidatorDocument + 'static {
  /// Helper method to upcast to an [`Any`] trait object.
  /// The intended use case is to enable downcasting to a concrete [`Document`].
  fn into_any(self: Box<Self>) -> Box<dyn Any>;

  /// Helper method to upcast to a [`ValidatorDocument`] trait object.  
  fn into_validator_document(self: Box<Self>) -> Box<dyn ValidatorDocument>;
}

impl<T> ValidatorDocumentExt for T
where
  T: ValidatorDocument + 'static,
{
  fn into_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn into_validator_document(self: Box<Self>) -> Box<dyn ValidatorDocument> {
    self
  }
}
#[async_trait]
pub trait ResolveValidator: private::Sealed {
  async fn resolve_validator(&self, did: &str) -> Result<Box<dyn ValidatorDocumentExt>>;
}

mod private {
  use super::ResolveValidator;

  pub trait Sealed {}
  impl<T> Sealed for T where T: ResolveValidator {}
}

#[async_trait]
impl<T> ResolveValidator for T
where
  T: Resolve + Send + Sync,
  T::DOC: Send + Sync + 'static,
  T::D: Send + Sync + 'static,
{
  async fn resolve_validator(&self, did: &str) -> Result<Box<dyn ValidatorDocumentExt>> {
    // TODO: Consider improving error handling.
    let parsed_did: <T as Resolve>::D = did.try_into().map_err(|_| {
      Error::DIDSyntaxError(identity_did::did::DIDError::Other(
        "failed to convert DID during resolution",
      ))
    })?;

    let doc = self.resolve(&parsed_did).await?;

    Ok(Box::new(doc))
  }
}
