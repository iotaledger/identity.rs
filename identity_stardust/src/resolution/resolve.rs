// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Error, Result};
use async_trait::async_trait;
use identity_credential::validator::ValidatorDocument;
use identity_did::{
  did::{CoreDID, DIDError, DID},
  document::{CoreDocument, Document},
};
#[async_trait]
pub trait Resolve {
  type D: for<'a> TryFrom<CoreDID> + DID;
  type DOC: Document<D = Self::D>;

  /// Fetch the associated DID Document from the given DID.
  async fn resolve(&self, did: &Self::D) -> Result<Self::DOC>;
}

#[async_trait]
pub(super) trait ResolveDynamic: private::Sealed {
  async fn resolve_dynamic(&self, did: CoreDID) -> Result<Box<dyn ValidatorDocument>>;
}

// TODO: Is Sealed necessary here, it is only available to the super module and not the public API ...
mod private {
  use super::Resolve;
  pub trait Sealed {}
  impl<T> Sealed for T where T: Resolve {}
}

#[async_trait]
impl<T> ResolveDynamic for T
where
  T: Resolve + Send + Sync,
  T::DOC: Send + Sync + 'static,
  T::D: Send + Sync + 'static,
{
  async fn resolve_dynamic(&self, did: CoreDID) -> Result<Box<dyn ValidatorDocument>> {
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
