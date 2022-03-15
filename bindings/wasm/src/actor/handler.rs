use std::{any::Any, marker::PhantomData};

use futures::Future;
use identity::actor::{
  primitives::AnyFuture, primitives::RequestHandler, primitives::SyncMode, Actor, ActorRequest, RemoteSendError,
  RequestContext,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[repr(transparent)]
pub struct Json(serde_json::Value);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[repr(transparent)]
pub struct ActorRequestJson(serde_json::Value);

impl<MOD: SyncMode> ActorRequest<MOD> for ActorRequestJson {
  type Response = ();

  fn endpoint<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    unreachable!("`ActorRequestJson` exists only for serialization, it is never used as an `ActorRequest` instance.")
  }
}

/// An abstraction over an asynchronous function that processes some [`ActorRequest`].
#[derive(Clone)]
pub struct WasmHandler<MOD: SyncMode, OBJ, FUT, FUN>
where
  OBJ: 'static,
  FUT: Future<Output = Json>,
  FUN: Fn(OBJ, Actor, RequestContext<Json>) -> FUT,
  MOD: 'static,
{
  func: FUN,
  _marker_obj: std::marker::PhantomData<&'static OBJ>,
  _marker_mod: std::marker::PhantomData<&'static MOD>,
}

impl<MOD: SyncMode, OBJ, FUT, FUN> WasmHandler<MOD, OBJ, FUT, FUN>
where
  OBJ: 'static,
  FUT: Future<Output = Json>,

  FUN: Fn(OBJ, Actor, RequestContext<Json>) -> FUT,
  MOD: 'static,
{
  pub fn _new(func: FUN) -> Self {
    Self {
      func,
      _marker_obj: PhantomData,
      _marker_mod: PhantomData,
    }
  }
}

impl<MOD: SyncMode, OBJ, FUT, FUN> RequestHandler for WasmHandler<MOD, OBJ, FUT, FUN>
where
  OBJ: Clone + Send + Sync + 'static,
  FUT: Future<Output = Json> + Send,
  FUN: Send + Sync + Fn(OBJ, Actor, RequestContext<Json>) -> FUT,
  MOD: Send + Sync + 'static,
{
  fn invoke(
    &self,
    actor: Actor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
  ) -> Result<AnyFuture<'_>, RemoteSendError> {
    let input: Box<Json> = input.downcast().map_err(|_| {
      RemoteSendError::HandlerInvocationError(format!(
        "{}: could not downcast request to: {}",
        context.endpoint,
        std::any::type_name::<Json>()
      ))
    })?;

    let request: RequestContext<Json> = context.convert(*input);

    let boxed_object: Box<OBJ> = object.downcast().map_err(|_| {
      RemoteSendError::HandlerInvocationError(format!(
        "{}: could not downcast state object to: {}",
        request.endpoint,
        std::any::type_name::<OBJ>()
      ))
    })?;
    let future = async move {
      let response: Json = (self.func)(*boxed_object, actor, request).await;
      let type_erased: Box<dyn Any + Send> = Box::new(response);
      type_erased
    };
    Ok(Box::pin(future))
  }

  fn serialize_response(&self, input: Box<dyn Any>) -> Result<Vec<u8>, RemoteSendError> {
    identity::actor::primitives::request_handler_serialize_response::<MOD, ActorRequestJson>(input)
  }

  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError> {
    identity::actor::primitives::request_handler_deserialize_request::<MOD, ActorRequestJson>(input)
  }

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError> {
    identity::actor::primitives::request_handler_clone_object::<OBJ>(object)
  }
}
