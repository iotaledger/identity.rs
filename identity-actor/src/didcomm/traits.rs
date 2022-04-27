// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use super::DidCommActor;
use crate::actor::traits::RequestHandlerCore;
use crate::actor::AnyFuture;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;

pub trait AsyncRequestHandler: RequestHandlerCore + Send + Sync {
  /// Invokes the handler with the given `actor` and `context`, as well as the shared
  /// state `object` and the `input` received from a peer. Returns the result as a
  /// type-erased `Any` object.
  fn invoke(
    &self,
    actor: DidCommActor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
  ) -> Result<AnyFuture<'_>, RemoteSendError>;
}
