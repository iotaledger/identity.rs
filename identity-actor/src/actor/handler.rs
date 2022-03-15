// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::future::Future;
use std::marker::PhantomData;

use crate::traits::AnyFuture;
use crate::traits::RequestHandler;
use crate::Actor;
use crate::ActorRequest;
use crate::RemoteSendError;
use crate::RequestContext;
use crate::SyncMode;

/// An abstraction over an asynchronous function that processes some [`ActorRequest`].
#[derive(Clone)]
pub struct Handler<MOD, OBJ, REQ, FUT>
where
  OBJ: 'static,
  REQ: ActorRequest<MOD>,
  FUT: Future<Output = REQ::Response>,
  MOD: SyncMode + 'static,
{
  func: fn(OBJ, Actor, RequestContext<REQ>) -> FUT,
  // Need to use the types that appear in the closure's arguments here,
  // as it is otherwise considered unused.
  // Since this type does not actually own any of these types, we use a reference.
  // See also the drop check section in the PhantomData doc.
  _marker_obj: std::marker::PhantomData<&'static OBJ>,
  _marker_req: std::marker::PhantomData<&'static REQ>,
  _marker_mod: std::marker::PhantomData<&'static MOD>,
}

impl<MOD, OBJ, REQ, FUT> Handler<MOD, OBJ, REQ, FUT>
where
  OBJ: 'static,
  REQ: ActorRequest<MOD>,
  FUT: Future<Output = REQ::Response>,
  MOD: SyncMode + 'static,
{
  pub fn new(func: fn(OBJ, Actor, RequestContext<REQ>) -> FUT) -> Self {
    Self {
      func,
      _marker_obj: PhantomData,
      _marker_req: PhantomData,
      _marker_mod: PhantomData,
    }
  }
}

impl<MOD, OBJ, REQ, FUT> RequestHandler for Handler<MOD, OBJ, REQ, FUT>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: ActorRequest<MOD> + Sync,
  REQ::Response: Send,
  FUT: Future<Output = REQ::Response> + Send,
  MOD: SyncMode + Send + Sync + 'static,
{
  fn invoke(
    &self,
    actor: Actor,
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

  fn serialize_response(&self, input: Box<dyn Any>) -> Result<Vec<u8>, RemoteSendError> {
    crate::traits::request_handler_serialize_response::<MOD, REQ>(input)
  }

  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError> {
    crate::traits::request_handler_deserialize_request::<MOD, REQ>(input)
  }

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError> {
    crate::traits::request_handler_clone_object::<OBJ>(object)
  }
}
