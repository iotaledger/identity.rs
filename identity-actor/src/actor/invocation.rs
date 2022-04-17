// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::result::Result as StdResult;

use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::ResponseMessage;
use crate::traits::RequestHandler;
use crate::Actor;
use crate::ActorStateExtension;
use crate::RemoteSendError;
use crate::RequestContext;

use libp2p::request_response::InboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;

// An abstraction over the strategy with which to invoke a handler, which is implemented
// synchronously and asynchronously.
// #[async_trait::async_trait]
// pub trait InvocationStrategy: Send + Sync + 'static {
//   /// Invokes the `handler` and communicates with the remote through `channel`.
//   #[allow(clippy::too_many_arguments)]
//   async fn invoke_handler(
//     handler: &dyn RequestHandler,
//     actor: Actor,
//     context: RequestContext<()>,
//     object: Box<dyn Any + Send + Sync>,
//     input: Box<dyn Any + Send>,
//     channel: ResponseChannel<ResponseMessage>,
//     request_id: RequestId,
//   );

//   /// Called when the actor is unable to deserialize the request to the expected input for the handler.
//   async fn handler_deserialization_failure(
//     actor: &mut Actor,
//     channel: ResponseChannel<ResponseMessage>,
//     request_id: RequestId,
//     error: RemoteSendError,
//   ) -> crate::Result<StdResult<(), InboundFailure>>;

//   /// Called when no handler was found for the requested endpoint.
//   async fn endpoint_not_found(actor: &mut Actor, request: InboundRequest);
// }

pub(crate) async fn send_response<T: serde::Serialize>(
  commander: &mut NetCommander,
  response: StdResult<T, RemoteSendError>,
  channel: ResponseChannel<ResponseMessage>,
  request_id: RequestId,
) -> crate::Result<StdResult<(), InboundFailure>> {
  let response: Vec<u8> = serde_json::to_vec(&response).unwrap();
  commander.send_response(response, channel, request_id).await
}

pub struct SynchronousInvocationStrategy;

impl SynchronousInvocationStrategy {
  #[inline(always)]
  pub async fn endpoint_not_found<EXT>(actor: &mut Actor<EXT>, request: InboundRequest)
  where
    EXT: ActorStateExtension,
  {
    let response: StdResult<Vec<u8>, RemoteSendError> =
      Err(RemoteSendError::UnexpectedRequest(request.endpoint.to_string()));

    let send_result = send_response(
      &mut actor.commander,
      response,
      request.response_channel,
      request.request_id,
    )
    .await;

    if let Err(err) = send_result {
      log::error!("could not return error to: {err:?}",);
    }
  }

  #[inline(always)]
  pub async fn handler_deserialization_failure<EXT>(
    actor: &mut Actor<EXT>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
    error: RemoteSendError,
  ) -> crate::Result<StdResult<(), InboundFailure>>
  where
    EXT: ActorStateExtension,
  {
    send_response(
      &mut actor.commander,
      StdResult::<Vec<u8>, RemoteSendError>::Err(error),
      channel,
      request_id,
    )
    .await
  }

  #[inline(always)]
  pub async fn invoke_handler<EXT>(
    handler: &dyn RequestHandler,
    actor: Actor<EXT>,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  ) where
    EXT: ActorStateExtension,
  {
    let mut commander = actor.commander.clone();
    let endpoint = context.endpoint.clone();

    match handler.invoke(Box::new(actor), context, object, input) {
      Ok(invocation) => {
        let result = invocation.await;
        let ser_res = handler.serialize_response(result);

        match ser_res {
          Ok(response) => {
            if let Err(error) = send_response(&mut commander, Ok(response), channel, request_id).await {
              log::error!("unable to respond to synchronous request on endpoint `{endpoint}` due to: {error}");
            }
          }
          Err(err) => {
            if let Err(error) = send_response(
              &mut commander,
              StdResult::<(), RemoteSendError>::Err(err),
              channel,
              request_id,
            )
            .await
            {
              log::error!("unable to respond to synchronous request on endpoint `{endpoint}` due to: {error}");
            }
          }
        }
      }
      Err(err) => {
        log::error!("{}", err);

        if let Err(error) = send_response(
          &mut commander,
          StdResult::<(), RemoteSendError>::Err(err),
          channel,
          request_id,
        )
        .await
        {
          log::error!("unable to respond to synchronous request on endpoint `{endpoint}` due to: {error}");
        }
      }
    }
  }
}
