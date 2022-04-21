// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::future::Future;
use std::marker::PhantomData;

use crate::actor::actor_request::AsyncActorRequest;
use crate::actor::traits::RequestHandlerCore;
use crate::actor::AnyFuture;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;

use super::traits::AsyncRequestHandler;
use super::DidCommActor;

/// An abstraction over an asynchronous function that processes a [`AsyncActorRequest`].
#[derive(Clone)]
pub struct AsyncHandler<OBJ, REQ, FUT>
where
  OBJ: 'static,
  REQ: AsyncActorRequest,
  FUT: Future<Output = ()>,
{
  pub(crate) func: fn(OBJ, DidCommActor, RequestContext<REQ>) -> FUT,
  // Need to use the types that appear in the closure's arguments here,
  // as it is otherwise considered unused.
  // Since this type does not actually own any of these types, we use a reference.
  // See also the drop check section in the PhantomData doc.
  _marker_obj: std::marker::PhantomData<&'static OBJ>,
  _marker_req: std::marker::PhantomData<&'static REQ>,
}

impl<OBJ, REQ, FUT> AsyncHandler<OBJ, REQ, FUT>
where
  OBJ: 'static,
  REQ: AsyncActorRequest,
  FUT: Future<Output = ()>,
{
  pub fn new(func: fn(OBJ, DidCommActor, RequestContext<REQ>) -> FUT) -> Self {
    Self {
      func,
      _marker_obj: PhantomData,
      _marker_req: PhantomData,
    }
  }
}

impl<OBJ, REQ, FUT> AsyncRequestHandler for AsyncHandler<OBJ, REQ, FUT>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: AsyncActorRequest + Sync,
  FUT: Future<Output = ()> + Send,
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
      (self.func)(*boxed_object, actor, request).await;
      // This doesn't allocate because () is zero-sized.
      Box::new(()) as Box<dyn Any + Send>
    };

    Ok(Box::pin(future))
  }
}

impl<OBJ, REQ, FUT> RequestHandlerCore for AsyncHandler<OBJ, REQ, FUT>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: AsyncActorRequest + Sync,
  FUT: Future<Output = ()> + Send,
{
  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError> {
    crate::actor::request_handler_deserialize_request::<REQ>(input)
  }

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError> {
    crate::actor::request_handler_clone_object::<OBJ>(object)
  }
}
