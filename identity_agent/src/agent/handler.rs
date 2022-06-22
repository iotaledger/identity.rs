// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use crate::agent::ErrorLocation;
use crate::agent::HandlerRequest;
use crate::agent::RemoteSendError;
use crate::agent::RequestContext;

/// A boxed future that is `Send`.
pub(crate) type BoxFuture<'me, T> = Pin<Box<dyn Future<Output = T> + Send + 'me>>;

/// Handlers are objects that encapsulate state and behavior.
///
/// Handlers handle one or more requests by implementing this trait one or more times
/// for different `HandlerRequest` types.
///
/// The requests for a handler are handled synchronously, meaning that the calling agent waits for
/// the handler to return its result before continuing.
#[async_trait::async_trait]
pub trait Handler<REQ: HandlerRequest>: Debug + 'static {
  /// Called when the agent receives a request of type `REQ`.
  /// The result will be returned to the calling agent.
  async fn handle(&self, request: RequestContext<REQ>) -> REQ::Response;
}

/// A trait that wraps a synchronous handler implementation and erases its type.
/// This allows holding handlers with different concrete types in the same collection.
pub(crate) trait AbstractHandler: Debug + Send + Sync + 'static {
  fn handle(&self, request: RequestContext<Vec<u8>>) -> BoxFuture<'_, Result<Vec<u8>, RemoteSendError>>;
}

/// A wrapper around synchronous handler implementations that is used for
/// type erasure together with [`AbstractHandler`].
#[derive(Debug)]
pub(crate) struct HandlerWrapper<HND, REQ>
where
  REQ: HandlerRequest + Send + Sync,
  HND: Handler<REQ> + Send + Sync,
{
  handler: HND,
  _phantom_req: PhantomData<REQ>,
}

impl<HND, REQ> HandlerWrapper<HND, REQ>
where
  REQ: HandlerRequest + Send + Sync,
  HND: Handler<REQ> + Send + Sync,
{
  pub(crate) fn new(handler: HND) -> Self {
    Self {
      handler,
      _phantom_req: PhantomData,
    }
  }
}

impl<HND, REQ> AbstractHandler for HandlerWrapper<HND, REQ>
where
  REQ: HandlerRequest + Send + Sync,
  REQ::Response: Send,
  HND: Handler<REQ> + Send + Sync,
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
      let result: REQ::Response = self.handler.handle(req).await;
      serialize_response::<REQ>(&result)
    };

    Box::pin(future)
  }
}

#[inline(always)]
fn serialize_response<REQ: HandlerRequest>(input: &REQ::Response) -> Result<Vec<u8>, RemoteSendError> {
  log::debug!(
    "attempt response serialization into {:?}",
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
