// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
  any::{Any, TypeId},
  marker::PhantomData,
  pin::Pin,
};

use futures::Future;

use crate::traits::{ActorRequest, RequestHandler};

#[derive(Clone)]
pub struct AsyncFn<H, R, F, C>
where
  H: 'static,
  R: ActorRequest,
  F: Future<Output = R::Response>,
  C: Fn(H, R) -> F,
{
  func: C,
  // Need to use the types that appear in the closure's arguments here,
  // as it is otherwise considered unused.
  // Since this type does not actually own any of these types, we use a reference.
  // See alos the drop check section in the PhantomData doc.
  _marker_obj: PhantomData<&'static H>,
  _marker_req: PhantomData<&'static R>,
}

impl<H, R, F, C> AsyncFn<H, R, F, C>
where
  H: 'static,
  R: ActorRequest,
  F: Future<Output = R::Response>,
  C: Fn(H, R) -> F,
{
  pub fn new(func: C) -> Self {
    Self {
      func,
      _marker_obj: PhantomData,
      _marker_req: PhantomData,
    }
  }
}

impl<OBJ, REQ, FUT, CLO> RequestHandler for AsyncFn<OBJ, REQ, FUT, CLO>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: ActorRequest + Send + Sync,
  FUT: Future<Output = REQ::Response> + Send,
  CLO: Send + Sync + Fn(OBJ, REQ) -> FUT,
{
  fn invoke<'this>(
    &'this self,
    object: Box<dyn Any + Send + Sync>,
    input: Vec<u8>,
  ) -> Pin<Box<dyn Future<Output = Vec<u8>> + Send + 'this>> {
    log::debug!("Attempt deserialization into {:?}", std::any::type_name::<REQ>());
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
