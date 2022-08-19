// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::ResolutionHandler;
use crate::Error;
use crate::Result;
use core::future::Future;
use identity_credential::validator::BorrowValidator;
use identity_did::did::DID;
use std::pin::Pin;
use std::sync::Arc;

#[cfg(not(feature = "internals"))]
pub(super) type AsyncFnPtr<S, T> = Box<dyn for<'r> Fn(&'r S) -> Pin<Box<dyn Future<Output = T> + 'r>>>;
#[cfg(feature = "internals")]
/// Indicates an asynchronous function pointer returning a Future pinned to the heap.
pub type AsyncFnPtr<S, T> = Box<dyn for<'r> Fn(&'r S) -> Pin<Box<dyn Future<Output = T> + 'r>>>;

/// Intermediary type used to register a [`ResolutionHandler`](super::ResolutionHandler) with a
/// [`Resolver`](super::Resolver).
///
/// Consists of the DID Method encoded as a string and a collectable asynchronous function pointer that the [`Resolver`]
/// will delegate resolution to when encountering did's of the corresponding method.
pub(super) struct ResolverDelegate<DOC: BorrowValidator> {
  pub(super) method: String,
  pub(super) handler: AsyncFnPtr<str, Result<Option<DOC>>>,
}

impl<DOC: BorrowValidator + 'static> ResolverDelegate<DOC> {
  /// Constructor
  ///
  /// Converts a [`ResolutionHandler`] into a collectable asynchronous function pointer. The `output` transformer is
  /// used to transform the [resolved document](ResolutionHandler::Resolved) to any desired document type.
  // TODO: Improve error handling.
  pub(super) fn new<D, R>(handler: Arc<R>) -> Self
  where
    D: DID + Send + for<'r> TryFrom<&'r str> + 'static,
    R: ResolutionHandler<D> + 'static,
    <R as ResolutionHandler<D>>::Resolved: Into<DOC>,
  {
    let method = R::method();
    ResolverDelegate {
      method,
      handler: Box::new(move |input: &str| {
        let value_clone = handler.clone();
        Box::pin(async move {
          let did: D =
            D::try_from(input).map_err(|_| Error::ResolutionProblem(format!("failed to parse did: {}", input)))?;

          let resolved = value_clone.resolve(&did).await?;
          Ok(resolved.map(Into::into))
        })
      }),
    }
  }
}
