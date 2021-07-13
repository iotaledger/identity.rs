use std::{
  any::{Any, TypeId},
  pin::Pin,
};

use futures::Future;

use crate::traits::{ActorRequest, RequestHandler};

#[derive(Clone)]
pub struct AsyncFn<H, R, F>
where
  H: 'static,
  R: ActorRequest,
  F: Future<Output = R::Response>,
{
  func: fn(H, R) -> F,
}

impl<H, R, F> AsyncFn<H, R, F>
where
  H: 'static,
  R: ActorRequest,
  F: Future<Output = R::Response>,
{
  pub fn new(func: fn(H, R) -> F) -> Self {
    Self { func }
  }
}

impl<OBJ, REQ, FUT> RequestHandler for AsyncFn<OBJ, REQ, FUT>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: ActorRequest + Send,
  FUT: Future<Output = REQ::Response> + Send,
{
  fn invoke<'this>(
    &'this self,
    object: Box<dyn Any + Send + Sync>,
    input: Vec<u8>,
  ) -> Pin<Box<dyn Future<Output = Vec<u8>> + Send + 'this>> {
    let request: REQ = serde_json::from_slice(&input).unwrap();
    let boxed_object: Box<OBJ> = object.downcast().unwrap();
    let future = async move {
      let response: REQ::Response = (self.func)(*boxed_object, request).await;
      serde_json::to_vec(&response).unwrap()
    };
    Box::pin(future)
  }

  fn object_type_id(&self) -> TypeId {
    TypeId::of::<OBJ>()
  }

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Box<dyn Any + Send + Sync> {
    // Double indirection is unfortunately required - the downcast fails otherwise.
    Box::new(object.downcast_ref::<OBJ>().unwrap().clone())
  }
}
