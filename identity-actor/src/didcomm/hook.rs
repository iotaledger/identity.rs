// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::marker::PhantomData;

use futures::Future;

use crate::actor::HandlerBuilder;
use crate::traits::AnyFuture;
use crate::traits::RequestHandler;
use crate::Actor;
use crate::ActorRequest;
use crate::Endpoint;
use crate::HandlerObject;
use crate::RemoteSendError;
use crate::RequestContext;
use crate::Result as ActorResult;

use super::termination::DidCommTermination;
use crate::SyncMode;

impl<'builder, MOD, OBJ> HandlerBuilder<'builder, MOD, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
  MOD: SyncMode,
{
  pub fn add_hook<REQ, FUT>(
    self,
    endpoint: &'static str,
    handler: fn(OBJ, Actor, RequestContext<REQ>) -> FUT,
  ) -> ActorResult<Self>
  where
    REQ: ActorRequest<MOD> + Sync,
    REQ::Response: Send,
    FUT: Future<Output = Result<REQ, DidCommTermination>> + Send + 'static,
    MOD: Send + Sync + 'static,
  {
    let handler = Hook::new(handler);
    self.handler_map.insert(
      Endpoint::new(endpoint)?,
      HandlerObject::new(self.object_id, Box::new(handler)),
    );
    Ok(self)
  }
}

/// A function that hooks and thus extends existing handler logic.
/// Can modify incoming requests or abort handling.
#[derive(Clone)]
pub struct Hook<MOD, OBJ, REQ, FUT>
where
  OBJ: 'static,
  REQ: ActorRequest<MOD>,
  FUT: Future<Output = Result<REQ, DidCommTermination>>,
  MOD: SyncMode + 'static,
{
  func: fn(OBJ, Actor, RequestContext<REQ>) -> FUT,
  // Need to use the types that appear in the closure's arguments here,
  // as it is otherwise considered unused.
  // Since this type does not actually own any of these types, we use a reference.
  // See also the drop check section in the PhantomData doc.
  _marker_obj: PhantomData<&'static OBJ>,
  _marker_req: PhantomData<&'static REQ>,
  _marker_mod: PhantomData<&'static MOD>,
}

impl<MOD, OBJ, REQ, FUT> Hook<MOD, OBJ, REQ, FUT>
where
  OBJ: 'static,
  REQ: ActorRequest<MOD>,
  FUT: Future<Output = Result<REQ, DidCommTermination>>,
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

impl<MOD, OBJ, REQ, FUT> RequestHandler for Hook<MOD, OBJ, REQ, FUT>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: ActorRequest<MOD> + Sync,
  REQ::Response: Send,
  FUT: Future<Output = Result<REQ, DidCommTermination>> + Send,
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
      RemoteSendError::HookInvocationError(format!(
        "{}: could not downcast request to: {}",
        context.endpoint,
        std::any::type_name::<REQ>()
      ))
    })?;

    let request: RequestContext<REQ> = context.convert(*input);

    let boxed_object: Box<OBJ> = object.downcast().map_err(|_| {
      RemoteSendError::HookInvocationError(format!(
        "{}: could not downcast state object to: {}",
        request.endpoint,
        std::any::type_name::<OBJ>()
      ))
    })?;

    let future = async move {
      let response: Result<REQ, DidCommTermination> = (self.func)(*boxed_object, actor, request).await;
      let type_erased: Box<dyn Any + Send> = Box::new(response);
      type_erased
    };

    Ok(Box::pin(future))
  }

  fn serialize_response(&self, _input: Box<dyn Any>) -> Result<Vec<u8>, RemoteSendError> {
    unreachable!("serialize_response is never called on hooks");
  }

  fn deserialize_request(&self, _input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError> {
    unreachable!("deserialize_request is never called on hooks");
  }

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError> {
    crate::traits::request_handler_clone_object::<OBJ>(object)
  }
}
