// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::any::TypeId;
use std::pin::Pin;

use futures::Future;

use crate::Actor;
use crate::ActorRequest;
use crate::RemoteSendError;
use crate::RequestContext;

/// A future whose output is an `Any` trait object.
pub type AnyFuture<'me> = Pin<Box<dyn Future<Output = Box<dyn Any>> + Send + 'me>>;

pub trait RequestHandler: Send + Sync {
  fn invoke(
    &self,
    actor: Actor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    request: Box<dyn Any + Send>,
  ) -> Result<AnyFuture<'_>, RemoteSendError>;

  fn object_type_id(&self) -> TypeId;

  fn serialize_response(&self, input: Box<dyn Any>) -> Result<Vec<u8>, RemoteSendError>;

  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError>;

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Box<dyn Any + Send + Sync>;
}

// Default implementations of some RequestHandler methods. These cannot be implemented on
// the trait itself, because the trait cannot be made generic without losing its type-erasing nature.

#[inline(always)]
pub fn request_handler_serialize_response<REQ: ActorRequest>(input: Box<dyn Any>) -> Result<Vec<u8>, RemoteSendError> {
  log::debug!(
    "Attempt serialization into {:?}",
    std::any::type_name::<REQ::Response>()
  );

  let input = input.downcast::<REQ::Response>().expect("TODO");

  let response: Vec<u8> = serde_json::to_vec(&input)?;
  Ok(response)
}

#[inline(always)]
pub fn request_handler_deserialize_request<REQ: ActorRequest>(
  input: Vec<u8>,
) -> Result<Box<dyn Any + Send>, RemoteSendError> {
  log::debug!("Attempt deserialization into {:?}", std::any::type_name::<REQ>());
  let request: REQ = serde_json::from_slice(&input)?;
  Ok(Box::new(request))
}

#[inline(always)]
pub fn request_handler_object_type_id<OBJ: 'static>() -> TypeId {
  TypeId::of::<OBJ>()
}

#[inline(always)]
pub fn request_handler_clone_object<OBJ: Clone + Send + Sync + 'static>(
  object: &Box<dyn Any + Send + Sync>,
) -> Box<dyn Any + Send + Sync> {
  // TODO: Unwrap?
  // Double indirection is unfortunately required - the downcast fails otherwise.
  Box::new(object.downcast_ref::<OBJ>().unwrap().clone())
}
