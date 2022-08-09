// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Result;
use async_trait::async_trait;

use identity_did::{did::DID, document::Document};

#[async_trait]
/// A parameterized trait for handling resolution of DID Documents using a specified DID method.
pub trait ResolutionHandler<D>
where
  D: DID + for<'a> TryFrom<&'a str> + Send + 'static,
{
  type Resolved: Document + 'static + Send;

  /// Fetch the associated DID Document from the given DID.
  async fn resolve(&self, did: &D) -> Result<Self::Resolved>;

  /// The supported did method.
  /// The returned string is expected to match the `did-method-name` when parsing DIDs of the method this handler requires.
  fn method() -> String;
}

/*
#[async_trait]
impl<T> AbstractResolutionHandler for T
where
  T: ResolutionHandler + Send + Sync,
  T::DOC: Send + Sync + 'static,
  T::D: Send + Sync + 'static,
{
  async fn resolve_validator(&self, did: &str) -> Result<Box<dyn ValidatorDocument>> {
    // TODO: Consider improving error handling.
    let parsed_did: <T as ResolutionHandler>::D = did.parse().map_err(|_| {
      Error::DIDSyntaxError(identity_did::did::DIDError::Other(
        "failed to convert DID during resolution",
      ))
    })?;

    let doc = self.resolve(&parsed_did).await?;

    Ok(Box::new(doc))
  }

  fn method(&self) -> String {
    <T as ResolutionHandler>::method()
  }
}
*/
