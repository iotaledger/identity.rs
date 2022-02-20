// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::any::TypeId;
use std::borrow::Cow;
use std::fmt::Debug;
use std::pin::Pin;

use futures::Future;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Actor;
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

  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError>;

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Box<dyn Any + Send + Sync>;
}

// Default implementations of some RequestHandler methods. These cannot be implemented on
// the trait itself, because the trait cannot be made generic without losing its type-erasing nature.

#[inline(always)]
pub fn request_handler_deserialize_request<REQ: ActorRequest + Send + Sync>(
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
  // Double indirection is unfortunately required - the downcast fails otherwise.
  Box::new(object.downcast_ref::<OBJ>().unwrap().clone())
}

pub trait ActorRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  type Response: Debug + Serialize + DeserializeOwned + 'static;

  fn request_name<'cow>(&self) -> Cow<'cow, str>;
}
