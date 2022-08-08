// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::future::Future;
use std::{pin::Pin, sync::Arc};

use crate::{Error, Result};
use async_trait::async_trait;
use identity_credential::validator::ValidatorDocument;
use identity_did::{did::DID, document::Document};
use iota_client::block::output::Output;
#[async_trait]
/// A parameterized trait for handling resolution of DID Documents using a specified DID method.
pub trait ResolutionHandler<D>
where
  D: DID + for<'a> TryFrom<&'a str> + Send + 'static,
{
  type Output: Document<D = D> + 'static + Send + Default;

  /// Fetch the associated DID Document from the given DID.
  async fn resolve(&self, did: &D) -> Result<Self::Output>;

  /// The supported did method.
  fn method() -> String;

  /// This method enables registering the [`ResolutionHandler`] with a dynamic [`Resolver`] (see [`Resolver::<ValidatorDocument>::attach_handler](super::Resolver::<ValidatorDocument>::attach_handler)).
  ///
  /// The method can be called, but it is not possible to name the output type by design, hence one cannot overwrite this default method.
  fn into_delegate(self: Arc<Self>) -> DynamicResolverDelegate
  where
    Self: Sized + 'static,
  {
    let method = Self::method();
    DynamicResolverDelegate {
      method,
      handler: Box::new(move |input: &str| {
        let self_clone = self.clone();
        Box::pin(async move {
          let did: D = D::try_from(input)
            .map_err(|_| Error::ResolutionProblem("failed to parse did".into()))
            .unwrap();
          let resolved_doc = self_clone.resolve(&did).await.unwrap();
          Box::new(resolved_doc) as Box<dyn ValidatorDocument>
        })
      }),
    }
  }

  /// This method enables registering the [`ResolutionHandler`] with a typed [`Resolver`].
  ///
  /// The method cannot be overwritten and is only intended to be called internally (by [`Resolver::attach_handler`](super::Resolver::attach_handler)).
  fn into_typed_delegate<T>(self: Arc<Self>) -> TypedResolverDelegate<T>
  where
    Self: Sized + 'static,
    T: From<Self::Output> + Send + Document,
  {
    let method = Self::method();
    TypedResolverDelegate {
      method,
      handler: Box::new(move |input: &str| {
        let self_clone = self.clone();
        Box::pin(async move {
          let did: D = D::try_from(input)
            .map_err(|_| Error::ResolutionProblem("failed to parse did".into()))
            .unwrap();
          let resolved = self_clone.resolve(&did).await.unwrap();
          T::from(resolved)
        })
      }),
    }
  }
}

/// Intermediary type used internally by [`Resolver`::attach_handler](super::Resolver::attach_handler).
///
/// This type cannot be named.
pub struct TypedResolverDelegate<DOC: Document + Send + 'static> {
  pub(super) method: String,
  pub(super) handler: Box<dyn for<'r> Fn(&'r str) -> Pin<Box<dyn Future<Output = DOC> + 'r>>>,
}

/// Type passed to ...
pub struct DynamicResolverDelegate {
  pub(super) method: String,
  pub(super) handler: Box<dyn for<'r> Fn(&'r str) -> Pin<Box<dyn Future<Output = Box<dyn ValidatorDocument>> + 'r>>>,
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
