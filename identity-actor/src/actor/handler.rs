// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::future::Future;
use std::marker::PhantomData;

use crate::actor::ActorRequest;
use crate::actor::AnyFuture;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;
use crate::actor::RequestHandler;
use crate::actor::SyncMode;

/// An abstraction over an asynchronous function that processes some [`ActorRequest`].
#[derive(Clone)]
pub struct Handler<MOD, ACT, OBJ, REQ, FUT>
where
  ACT: Send + Sync + 'static,
  OBJ: 'static,
  REQ: ActorRequest<MOD>,
  FUT: Future<Output = REQ::Response>,
  MOD: SyncMode + 'static,
{
  func: fn(OBJ, ACT, RequestContext<REQ>) -> FUT,
  // Need to use the types that appear in the closure's arguments here,
  // as it is otherwise considered unused.
  // Since this type does not actually own any of these types, we use a reference.
  // See also the drop check section in the PhantomData doc.
  _marker_obj: std::marker::PhantomData<&'static OBJ>,
  _marker_req: std::marker::PhantomData<&'static REQ>,
  _marker_mod: std::marker::PhantomData<&'static MOD>,
  _marker_act: std::marker::PhantomData<&'static ACT>,
}

impl<MOD, ACT, OBJ, REQ, FUT> Handler<MOD, ACT, OBJ, REQ, FUT>
where
  ACT: Send + Sync + 'static,
  OBJ: 'static,
  REQ: ActorRequest<MOD>,
  FUT: Future<Output = REQ::Response>,
  MOD: SyncMode + 'static,
{
  pub fn new(func: fn(OBJ, ACT, RequestContext<REQ>) -> FUT) -> Self {
    Self {
      func,
      _marker_obj: PhantomData,
      _marker_req: PhantomData,
      _marker_mod: PhantomData,
      _marker_act: PhantomData,
    }
  }
}

impl<MOD, ACT, OBJ, REQ, FUT> RequestHandler for Handler<MOD, ACT, OBJ, REQ, FUT>
where
  ACT: Send + Sync + 'static,
  OBJ: Clone + Send + Sync + 'static,
  REQ: ActorRequest<MOD> + Sync,
  REQ::Response: Send,
  FUT: Future<Output = REQ::Response> + Send,
  MOD: SyncMode + Send + Sync + 'static,
{
  fn invoke(
    &self,
    actor: Box<dyn Any + Send + Sync>,
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

    let actor: ACT = *actor.downcast().expect("TODO");

    let future = async move {
      let response: REQ::Response = (self.func)(*boxed_object, actor, request).await;
      let type_erased: Box<dyn Any + Send> = Box::new(response);
      type_erased
    };
    Ok(Box::pin(future))
  }

  fn serialize_response(&self, input: Box<dyn Any>) -> Result<Vec<u8>, RemoteSendError> {
    crate::actor::request_handler_serialize_response::<MOD, REQ>(input)
  }

  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError> {
    crate::actor::request_handler_deserialize_request::<MOD, REQ>(input)
  }

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError> {
    crate::actor::request_handler_clone_object::<OBJ>(object)
  }
}
