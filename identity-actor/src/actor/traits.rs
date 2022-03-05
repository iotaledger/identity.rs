// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::pin::Pin;

use futures::Future;

use crate::Actor;
use crate::ActorRequest;
use crate::RemoteSendError;
use crate::RequestContext;

use super::actor_request::private::SyncMode;

/// A future whose output is an `Any` trait object.
pub type AnyFuture<'me> = Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send + 'me>>;

/// An abstraction for an asynchronous function.
pub trait RequestHandler: Send + Sync {
  /// Invokes the handler with the given `actor` and `context`, as well as the shared
  /// state `object` and the `input` received from a peer. Returns the result as a
  /// type-erased `Any` object.
  fn invoke(
    &self,
    actor: Actor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
  ) -> Result<AnyFuture<'_>, RemoteSendError>;

  /// Serializes the returned result from [`Self::invoke`].
  fn serialize_response(&self, input: Box<dyn Any>) -> Result<Vec<u8>, RemoteSendError>;

  /// Attempts to deserialize bytes into some input type compatible with [`Self::invoke`].
  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError>;

  /// Helper function to clone the type-erased shared state object.
  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError>;
}

// Default implementations of some RequestHandler methods. These cannot be implemented on
// the trait itself, because the trait cannot be made generic without losing its type-erasing nature.

#[inline(always)]
pub fn request_handler_serialize_response<MOD: SyncMode, REQ: ActorRequest<MOD>>(
  input: Box<dyn Any>,
) -> Result<Vec<u8>, RemoteSendError> {
  log::debug!(
    "Attempt serialization into {:?}",
    std::any::type_name::<REQ::Response>()
  );

  // Note: The error here is impossible unless there's an actor logic bug.
  let input = input.downcast::<REQ::Response>().map_err(|_| {
    RemoteSendError::HandlerInvocationError(format!(
      "could not downcast response to: {}",
      std::any::type_name::<REQ::Response>()
    ))
  })?;

  let response: Vec<u8> = serde_json::to_vec(&input).map_err(|_| RemoteSendError::SerializationFailure {
    location: "[request handler serialization]".to_owned(),
    message: format!(
      "failed to serialize response into {}",
      std::any::type_name::<REQ::Response>()
    ),
  })?;

  Ok(response)
}

#[inline(always)]
pub fn request_handler_deserialize_request<MOD: SyncMode, REQ: ActorRequest<MOD>>(
  input: Vec<u8>,
) -> Result<Box<dyn Any + Send>, RemoteSendError> {
  log::debug!("Attempt deserialization into {:?}", std::any::type_name::<REQ>());

  let request: REQ = serde_json::from_slice(&input).map_err(|_| RemoteSendError::DeserializationFailure {
    location: "[request handler deserialization]".to_owned(),
    message: format!("failed to deserialize request into {}", std::any::type_name::<REQ>()),
  })?;

  Ok(Box::new(request))
}

#[inline(always)]
pub fn request_handler_clone_object<OBJ: Clone + Send + Sync + 'static>(
  object: &Box<dyn Any + Send + Sync>,
) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError> {
  // Double indirection is unfortunately required - the downcast fails otherwise.
  let object: &OBJ = object.downcast_ref::<OBJ>().ok_or_else(|| {
    crate::RemoteSendError::HandlerInvocationError(format!(
      "unable to downcast to type {} in order to clone the object",
      std::any::type_name::<OBJ>()
    ))
  })?;

  Ok(Box::new(object.clone()))
}
