// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::fmt::Debug;
use std::pin::Pin;

use futures::Future;
use serde::de::DeserializeOwned;

use crate::actor::errors::ErrorLocation;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;
use crate::actor::SyncActorRequest;

use super::Actor;

/// A boxed future whose output is an `Any` trait object.
pub type AnyFuture<'me> = Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send + 'me>>;

pub trait RequestHandlerCore {
  /// Attempts to deserialize bytes into some input type for use in invocations of this handler.
  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError>;

  /// Helper function to clone the type-erased shared state object.
  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError>;
}

/// An abstraction for an asynchronous function.
pub trait SyncRequestHandler: RequestHandlerCore + Send + Sync {
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

  /// Serializes the returned result from an invocation of this handler.
  fn serialize_response(&self, input: Box<dyn Any>) -> Result<Vec<u8>, RemoteSendError>;
}

// Default implementations of some (A)SyncRequestHandler methods. These cannot be implemented on
// the trait itself, because the trait cannot be made generic without losing its type-erasing nature.

#[inline(always)]
pub fn request_handler_serialize_response<REQ: SyncActorRequest>(
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

  let response: Vec<u8> = serde_json::to_vec(&input).map_err(|error| RemoteSendError::SerializationFailure {
    location: ErrorLocation::Remote,
    context: format!(
      "serializing the handler's response into `{}`",
      std::any::type_name::<REQ::Response>()
    ),
    error_message: error.to_string(),
  })?;

  Ok(response)
}

#[inline(always)]
pub fn request_handler_deserialize_request<REQ>(input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError>
where
  REQ: Debug + DeserializeOwned + Send + 'static,
{
  log::debug!("Attempt deserialization into {:?}", std::any::type_name::<REQ>());

  let request: REQ = serde_json::from_slice(&input).map_err(|error| RemoteSendError::DeserializationFailure {
    location: ErrorLocation::Remote,
    context: format!(
      "deserializing the received bytes into the handler's expected type `{}`",
      std::any::type_name::<REQ>()
    ),
    error_message: error.to_string(),
  })?;

  Ok(Box::new(request))
}

#[inline(always)]
pub fn request_handler_clone_object<OBJ: Clone + Send + Sync + 'static>(
  object: &Box<dyn Any + Send + Sync>,
) -> Result<Box<dyn Any + Send + Sync>, RemoteSendError> {
  // Double indirection is unfortunately required - the downcast fails otherwise.
  let object: &OBJ = object.downcast_ref::<OBJ>().ok_or_else(|| {
    crate::actor::RemoteSendError::HandlerInvocationError(format!(
      "unable to downcast to type {} in order to clone the object",
      std::any::type_name::<OBJ>()
    ))
  })?;

  Ok(Box::new(object.clone()))
}
