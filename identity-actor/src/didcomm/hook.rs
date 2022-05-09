// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::marker::PhantomData;

use futures::Future;

use crate::actor::traits::RequestHandlerCore;
use crate::actor::AnyFuture;
use crate::actor::AsyncActorRequest;
use crate::actor::Endpoint;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;

use super::didcomm_actor::DidCommActor;
use super::termination::DidCommTermination;
use super::traits::AsyncRequestHandler;
use super::AsyncHandlerObject;
use super::DidCommHandlerBuilder;

impl<'builder, OBJ> DidCommHandlerBuilder<'builder, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
{
  pub fn add_hook<REQ, FUT>(self, handler: fn(OBJ, DidCommActor, RequestContext<REQ>) -> FUT) -> Self
  where
    REQ: AsyncActorRequest + Sync,
    FUT: Future<Output = Result<REQ, DidCommTermination>> + Send + 'static,
  {
    let handler: Hook<_, _, _> = Hook::new(handler);
    let endpoint: Endpoint = REQ::endpoint().into_hook();

    println!("adding hook {endpoint}");

    self
      .async_handlers
      .insert(endpoint, AsyncHandlerObject::new(self.object_id, Box::new(handler)));
    self
  }
}

/// A function that hooks and thus extends existing handler logic.
/// Can modify incoming requests or abort handling.
#[derive(Clone)]
pub struct Hook<OBJ, REQ, FUT>
where
  OBJ: 'static,
  REQ: AsyncActorRequest,
  FUT: Future<Output = Result<REQ, DidCommTermination>>,
{
  func: fn(OBJ, DidCommActor, RequestContext<REQ>) -> FUT,
  // Need to use the types that appear in the closure's arguments here,
  // as it is otherwise considered unused.
  // Since this type does not actually own any of these types, we use a reference.
  // See also the drop check section in the PhantomData doc.
  _marker_obj: PhantomData<&'static OBJ>,
  _marker_req: PhantomData<&'static REQ>,
}

impl<OBJ, REQ, FUT> Hook<OBJ, REQ, FUT>
where
  OBJ: 'static,
  REQ: AsyncActorRequest,
  FUT: Future<Output = Result<REQ, DidCommTermination>>,
{
  pub fn new(func: fn(OBJ, DidCommActor, RequestContext<REQ>) -> FUT) -> Self {
    Self {
      func,
      _marker_obj: PhantomData,
      _marker_req: PhantomData,
    }
  }
}

impl<OBJ, REQ, FUT> AsyncRequestHandler for Hook<OBJ, REQ, FUT>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: AsyncActorRequest + Sync,
  FUT: Future<Output = Result<REQ, DidCommTermination>> + Send,
{
  fn invoke(
    &self,
    actor: DidCommActor,
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
}

impl<OBJ, REQ, FUT> RequestHandlerCore for Hook<OBJ, REQ, FUT>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: AsyncActorRequest + Sync,
  FUT: Future<Output = Result<REQ, DidCommTermination>> + Send,
{
  fn deserialize_request(&self, _input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError> {
    unreachable!("deserialize_request is never called on hooks");
  }

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError> {
    crate::actor::request_handler_clone_object::<OBJ>(object)
  }
}
