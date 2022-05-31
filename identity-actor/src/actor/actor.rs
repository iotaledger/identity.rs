// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use super::ActorRequest;
use super::RemoteSendError;
use super::RequestContext;
use crate::actor::ErrorLocation;

/// A boxed future that is `Send`.
pub(crate) type BoxFuture<'me, T> = Pin<Box<dyn Future<Output = T> + Send + 'me>>;

/// Actors are objects that encapsulate state and behavior.
///
/// Actors handle one or more requests by implementing this trait one or more times
/// for different `ActorRequest` types.
///
/// The requests for an actor are handled synchronously, meaning that the caller waits for
/// the actor to return its result before continuing.
#[async_trait::async_trait]
pub trait Actor<REQ: ActorRequest>: Debug + 'static {
  /// Called when the system receives a request of type `REQ`.
  /// The result will be returned to the calling peer.
  async fn handle(&self, request: RequestContext<REQ>) -> REQ::Response;
}

/// A trait that wraps a synchronous actor implementation and erases its type.
/// This allows holding actors with different concrete types in the same collection.
pub(crate) trait AbstractActor: Debug + Send + Sync + 'static {
  fn handle(&self, request: RequestContext<Vec<u8>>) -> BoxFuture<'_, Result<Vec<u8>, RemoteSendError>>;
}

/// A wrapper around synchronous actor implementations that is used for
/// type erasure together with [`AbstractSyncActor`].
#[derive(Debug)]
pub(crate) struct ActorWrapper<ACT, REQ>
where
  REQ: ActorRequest + Send + Sync,
  ACT: Actor<REQ> + Send + Sync,
{
  actor: ACT,
  _phantom_req: PhantomData<REQ>,
}

impl<ACT, REQ> ActorWrapper<ACT, REQ>
where
  REQ: ActorRequest + Send + Sync,
  ACT: Actor<REQ> + Send + Sync,
{
  pub(crate) fn new(actor: ACT) -> Self {
    Self {
      actor,
      _phantom_req: PhantomData,
    }
  }
}

impl<ACT, REQ> AbstractActor for ActorWrapper<ACT, REQ>
where
  REQ: ActorRequest + Send + Sync,
  REQ::Response: Send,
  ACT: Actor<REQ> + Send + Sync,
{
  fn handle(&self, request: RequestContext<Vec<u8>>) -> BoxFuture<'_, Result<Vec<u8>, RemoteSendError>> {
    let future = async move {
      let req: REQ =
        serde_json::from_slice(&request.input).map_err(|error| RemoteSendError::DeserializationFailure {
          location: ErrorLocation::Remote,
          context: format!(
            "deserializing the received bytes into the handler's expected type `{}`",
            std::any::type_name::<REQ>()
          ),
          error_message: error.to_string(),
        })?;

      let req: RequestContext<REQ> = request.convert(req);
      let result: REQ::Response = self.actor.handle(req).await;
      serialize_response::<REQ>(&result)
    };

    Box::pin(future)
  }
}

#[inline(always)]
fn serialize_response<REQ: ActorRequest>(input: &REQ::Response) -> Result<Vec<u8>, RemoteSendError> {
  log::debug!(
    "Attempt serialization into {:?}",
    std::any::type_name::<REQ::Response>()
  );

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
