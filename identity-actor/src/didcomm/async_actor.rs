// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;
use serde::Serialize;

use crate::actor::AsyncActorRequest;
use crate::actor::BoxFuture;
use crate::actor::Endpoint;
use crate::actor::ErrorLocation;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;
use crate::didcomm::DidCommSystem;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::ResponseMessage;

#[async_trait::async_trait]
pub trait AsyncActor<REQ: AsyncActorRequest>: 'static {
  async fn handle(&self, actor: DidCommSystem, request: RequestContext<REQ>);
}

pub trait AbstractAsyncActor: Send + Sync + 'static {
  fn handle(&self, actor: DidCommSystem, request: InboundRequest) -> BoxFuture<'_, ()>;
}

pub struct AsyncActorWrapper<ACT, REQ>
where
  REQ: AsyncActorRequest + Send + Sync,
  ACT: AsyncActor<REQ> + Send + Sync,
{
  actor: ACT,
  _phantom_req: PhantomData<REQ>,
}

impl<ACT, REQ> AsyncActorWrapper<ACT, REQ>
where
  REQ: AsyncActorRequest + Send + Sync,
  ACT: AsyncActor<REQ> + Send + Sync,
{
  pub fn new(actor: ACT) -> Self {
    Self {
      actor,
      _phantom_req: PhantomData,
    }
  }
}

impl<ACT, REQ> AbstractAsyncActor for AsyncActorWrapper<ACT, REQ>
where
  REQ: AsyncActorRequest + Send + Sync,
  ACT: AsyncActor<REQ> + Send + Sync,
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
