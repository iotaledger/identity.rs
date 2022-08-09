// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::ResolutionHandler;
use crate::{Error, Result};
use core::future::Future;
use identity_credential::validator::ValidatorDocument;
use identity_did::did::DID;
use std::{pin::Pin, sync::Arc};

pub(super) type AsyncFnPtr<T> = Box<dyn for<'r> Fn(&'r str) -> Pin<Box<dyn Future<Output =T> + 'r>>>;

/// Intermediary type used to register a [`ResolutionHandler`](super::ResolutionHandler) with a [`Resolver`](super::Resolver).
///
/// Consists of the DID Method encoded as a string and a collectable asynchronous function pointer that the [`Resolver`] will
/// delegate resolution to when encountering did's of the corresponding method.
pub(super) struct ResolverDelegate<DOC: ValidatorDocument> {
  pub(super) method: String,
  pub(super) handler: AsyncFnPtr<Result<DOC>>,
}

impl<DOC: ValidatorDocument + 'static> ResolverDelegate<DOC> {
  /// Constructor
  ///
  /// Converts a [`ResolutionHandler`] into a collectable asynchronous function pointer. The `output` transformer is used to
  /// transform the [resolved document](ResolutionHandler::Resolved) to any desired document type.
  ///  
  /// The trait bounds on F cover both the desired cases: <T as From>::from and |doc| {Box::new(doc) as Box<dyn ValidatorDocument>}
  // TODO: Improve error handling.
  pub(super) fn new<D, R, F>(handler: Arc<R>, output_transformer: F) -> Self
  where
    D: DID + Send + for<'r> TryFrom<&'r str> + 'static,
    R: ResolutionHandler<D> + 'static,
    F: Fn(<R as ResolutionHandler<D>>::Resolved) -> DOC + Copy + 'static,
  {
    let method = R::method();
    ResolverDelegate {
      method,
      handler: Box::new(move |input: &str| {
        let value_clone = handler.clone();
        Box::pin(async move {
          let did: D = D::try_from(input).map_err(|_| Error::ResolutionProblem("failed to parse did".into()))?;

          let resolved = value_clone.resolve(&did).await?;
          Ok(output_transformer(resolved))
        })
      }),
    }
  }
}
