// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;
use std::marker::PhantomData;

use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;
use serde::Serialize;

use crate::agent::BoxFuture;
use crate::agent::Endpoint;
use crate::agent::ErrorLocation;
use crate::agent::RemoteSendError;
use crate::agent::RequestContext;
use crate::didcomm::DidCommAgent;
use crate::didcomm::DidCommRequest;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::ResponseMessage;

/// Handlers are objects that encapsulate state and behavior.
///
/// A DidCommHandler handles one or more requests by implementing this trait one or more times
/// for different `DidCommRequest` types.
///
/// The requests for a DidCommHandler are handled asynchronously, meaning that the calling agent does
/// not wait for the handler to complete its invocation. If that is desired, the [`Handler`](crate::agent::Handler)
/// trait should be implemented instead.
#[async_trait::async_trait]
pub trait DidCommHandler<REQ: DidCommRequest>: Debug + 'static {
  /// Called when the agent receives a request of type `REQ`.
  async fn handle(&self, handler: DidCommAgent, request: RequestContext<REQ>);
}

/// A trait that wraps a DidCommHandler implementation and erases its type.
/// This allows holding handlers with different concrete types in the same collection.
pub(crate) trait AbstractDidCommHandler: Debug + Send + Sync + 'static {
  fn handle(&self, handler: DidCommAgent, request: InboundRequest) -> BoxFuture<'_, ()>;
}

/// A wrapper around asynchronous handler implementations that is used for
/// type erasure together with [`AbstractAsyncHandler`].
#[derive(Debug)]
pub(crate) struct DidCommHandlerWrapper<ACT, REQ>
where
  REQ: DidCommRequest + Send + Sync,
  ACT: DidCommHandler<REQ> + Send + Sync,
{
  handler: ACT,
  _phantom_req: PhantomData<REQ>,
}

impl<ACT, REQ> DidCommHandlerWrapper<ACT, REQ>
where
  REQ: DidCommRequest + Send + Sync,
  ACT: DidCommHandler<REQ> + Send + Sync,
{
  pub(crate) fn new(handler: ACT) -> Self {
    Self {
      handler,
      _phantom_req: PhantomData,
    }
  }
}

impl<ACT, REQ> AbstractDidCommHandler for DidCommHandlerWrapper<ACT, REQ>
where
  REQ: DidCommRequest + Send + Sync,
  ACT: DidCommHandler<REQ> + Send + Sync,
{
  fn handle(&self, mut agent: DidCommAgent, request: InboundRequest) -> BoxFuture<'_, ()> {
    let future: _ = async move {
      let req: REQ = match serde_json::from_slice::<'_, REQ>(&request.input).map_err(|error| {
        RemoteSendError::DeserializationFailure {
          location: ErrorLocation::Remote,
          context: format!(
            "deserializing the received bytes into the handler's expected type `{}`",
            std::any::type_name::<REQ>()
          ),
          error_message: error.to_string(),
        }
      }) {
        Ok(req) => {
          // Acknowledge request was received and understood.
          send_didcomm_response(
            agent.commander_mut(),
            Ok(()),
            &request.endpoint,
            request.response_channel,
            request.request_id,
          )
          .await;

          req
        }
        Err(err) => {
          send_didcomm_response(
            agent.commander_mut(),
            Result::<(), RemoteSendError>::Err(err),
            &request.endpoint,
            request.response_channel,
            request.request_id,
          )
          .await;

          // Abort because there is no request to handle and/or the peer is unresponsive.
          return;
        }
      };

      let context: RequestContext<REQ> = RequestContext::new(req, request.peer_id, request.endpoint);

      self.handler.handle(agent, context).await;
    };

    Box::pin(future)
  }
}

async fn send_didcomm_response<T: Serialize>(
  commander: &mut NetCommander,
  response: Result<T, RemoteSendError>,
  endpoint: &Endpoint,
  channel: ResponseChannel<ResponseMessage>,
  request_id: RequestId,
) {
  match crate::agent::send_response(commander, response, channel, request_id).await {
    Ok(Err(err)) => {
      log::error!(
        "could not send error for request on endpoint `{}` due to: {err:?}",
        endpoint
      );
    }
    Err(err) => {
      log::error!(
        "could not send error for request on endpoint `{}` due to: {err:?}",
        endpoint
      );
    }
    Ok(_) => (),
  }
}
