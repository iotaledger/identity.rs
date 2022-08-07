// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::future::Future;
use std::pin::Pin;

use crate::{Error, Result};
use async_trait::async_trait;
use identity_credential::validator::ValidatorDocument;
use identity_did::{did::DID, document::Document};
use iota_client::block::output::Output;
#[async_trait]
/// A trait for resolving DID documents adhering to a given DID method.
pub trait ResolutionHandler<D>: Clone
where
  D: DID + for<'a> TryFrom<&'a str> + Send + 'static,
{
  type Output: Document<D = D> + 'static + Send + Default;

  /// Fetch the associated DID Document from the given DID.
  async fn resolve(&self, did: &D) -> Result<Self::Output>;

  /// The supported did method.
  fn method() -> String;

  fn abstract_delegate(self) -> AbstractResolverDelegate
  where
    Self: Sized,
  {
    let method = Self::method();
    AbstractResolverDelegate {
      method,
      handler: Box::new(|input: &str| {
        Box::pin({
          let did: D = D::try_from(input)
            .map_err(|_| Error::ResolutionProblem("failed to parse did".into()))
            .unwrap();
          Self::Output::default()
        })
      }),
    }
  }

  fn into_delegate(self) -> ResolverDelegate<Self::Output>
  where
    Self: Sized + 'static,
  {
    let method = Self::method();
    ResolverDelegate {
      method,
      handler: Box::new(move |input: &str| {
        let self_clone = self.clone();
        Box::pin(async move {
          let did: D = D::try_from(input)
            .map_err(|_| Error::ResolutionProblem("failed to parse did".into()))
            .unwrap();
          self_clone.resolve(&did).await.unwrap()
        })
      }),
    }
  }
}

pub struct ResolverDelegate<DOC: Document + Send + 'static> {
  pub(super) method: String,
  pub(super) handler: Box<dyn for<'r> Fn(&'r str) -> Pin<Box<dyn Future<Output = DOC> + 'r>>>,
}

pub struct AbstractResolverDelegate {
  pub(super) method: String,
  pub(super) handler: Box<dyn Fn(&str) -> Pin<Box<dyn ValidatorDocument>>>,
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
