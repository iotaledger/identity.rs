// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;
use std::marker::PhantomData;

use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;
use serde::Serialize;

use crate::actor::BoxFuture;
use crate::actor::Endpoint;
use crate::actor::ErrorLocation;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;
use crate::didcomm::DidCommRequest;
use crate::didcomm::DidCommSystem;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::ResponseMessage;

/// Actors are objects that encapsulate state and behavior.
///
/// A DidCommActor handles one or more requests by implementing this trait one or more times
/// for different `DidCommRequest` types.
///
/// The requests for a DidCommActor are handled asynchronously, meaning that the calling agent does
/// not wait for the actor to complete its invocation. If that is desired, the [`Actor`](crate::actor::Actor) trait
/// should be implemented instead.
#[async_trait::async_trait]
pub trait DidCommActor<REQ: DidCommRequest>: Debug + 'static {
  /// Called when the system receives a request of type `REQ`.
  async fn handle(&self, actor: DidCommSystem, request: RequestContext<REQ>);
}

/// A trait that wraps a DidCommActor implementation and erases its type.
/// This allows holding actors with different concrete types in the same collection.
pub(crate) trait AbstractDidCommActor: Debug + Send + Sync + 'static {
  fn handle(&self, actor: DidCommSystem, request: InboundRequest) -> BoxFuture<'_, ()>;
}

/// A wrapper around asynchronous actor implementations that is used for
/// type erasure together with [`AbstractAsyncActor`].
#[derive(Debug)]
pub(crate) struct DidCommActorWrapper<ACT, REQ>
where
  REQ: DidCommRequest + Send + Sync,
  ACT: DidCommActor<REQ> + Send + Sync,
{
  actor: ACT,
  _phantom_req: PhantomData<REQ>,
}

impl<ACT, REQ> DidCommActorWrapper<ACT, REQ>
where
  REQ: DidCommRequest + Send + Sync,
  ACT: DidCommActor<REQ> + Send + Sync,
{
  pub(crate) fn new(actor: ACT) -> Self {
    Self {
      actor,
      _phantom_req: PhantomData,
    }
  }
}

impl<ACT, REQ> AbstractDidCommActor for DidCommActorWrapper<ACT, REQ>
where
  REQ: DidCommRequest + Send + Sync,
  ACT: DidCommActor<REQ> + Send + Sync,
{
  fn handle(&self, mut system: DidCommSystem, request: InboundRequest) -> BoxFuture<'_, ()> {
    let future: _ = async move {
      let req: REQ = match serde_json::from_slice::<'_, REQ>(&request.input).map_err(|error| {
        RemoteSendError::DeserializationFailure {
          location: ErrorLocation::Remote,
          context: format!(
            "deserializing the received bytes into the actor's expected type `{}`",
            std::any::type_name::<REQ>()
          ),
          error_message: error.to_string(),
        }
      }) {
        Ok(req) => {
          // Acknowledge request was received and understood.
          send_didcomm_response(
            system.commander_mut(),
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
            system.commander_mut(),
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

      self.actor.handle(system, context).await;
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
  match crate::actor::send_response(commander, response, channel, request_id).await {
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
