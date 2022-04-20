// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use super::DidCommActor;
use crate::actor::traits::RequestHandlerCore;
use crate::actor::ActorRequest;
use crate::actor::AnyFuture;
use crate::actor::Handler;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;
use crate::actor::SyncMode;
use std::future::Future;

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

impl<MOD, OBJ, REQ, FUT> AsyncRequestHandler for Handler<MOD, DidCommActor, OBJ, REQ, FUT>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: ActorRequest<MOD> + Sync,
  REQ::Response: Send,
  FUT: Future<Output = REQ::Response> + Send,
  MOD: SyncMode + Send + Sync + 'static,
{
  fn invoke(
    &self,
    actor: DidCommActor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
  ) -> Result<AnyFuture<'_>, RemoteSendError> {
    let input: Box<REQ> = input.downcast().map_err(|_| {
      RemoteSendError::HandlerInvocationError(format!(
        "{}: could not downcast request to: {}",
        context.endpoint,
        std::any::type_name::<REQ>()
      ))
    })?;

    let request: RequestContext<REQ> = context.convert(*input);

    let boxed_object: Box<OBJ> = object.downcast().map_err(|_| {
      RemoteSendError::HandlerInvocationError(format!(
        "{}: could not downcast state object to: {}",
        request.endpoint,
        std::any::type_name::<OBJ>()
      ))
    })?;

    let future = async move {
      let response: REQ::Response = (self.func)(*boxed_object, actor, request).await;
      let type_erased: Box<dyn Any + Send> = Box::new(response);
      type_erased
    };
    Ok(Box::pin(future))
  }
}
