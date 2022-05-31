// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::future::Future;
use std::pin::Pin;

use super::DidCommSystem;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;

/// A boxed future whose output is an `Any` trait object.
pub type AnyFuture<'me> = Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send + 'me>>;

pub trait RequestHandlerCore {
  /// Helper function to clone the type-erased shared state object.
  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError>;
}

pub trait AsyncRequestHandler: RequestHandlerCore + Send + Sync {
  /// Invokes the handler with the given `actor` and `context`, as well as the shared
  /// state `object` and the `input` received from a peer. Returns the result as a
  /// type-erased `Any` object.
  fn invoke(
    &self,
    actor: DidCommSystem,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
  ) -> Result<AnyFuture<'_>, RemoteSendError>;
}
