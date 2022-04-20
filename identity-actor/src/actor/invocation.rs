// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use crate::actor::RemoteSendError;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::actor::SyncRequestHandler;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::NetCommanderMut;
use crate::p2p::ResponseMessage;

use libp2p::request_response::InboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;

use super::actor::RawActor;
use super::Actor;
use super::ActorStateRef;

pub(crate) async fn send_response<T: serde::Serialize>(
  commander: &mut NetCommander,
  response: Result<T, RemoteSendError>,
  channel: ResponseChannel<ResponseMessage>,
  request_id: RequestId,
) -> ActorResult<Result<(), InboundFailure>> {
  let response: Vec<u8> = serde_json::to_vec(&response).unwrap();
  commander.send_response(response, channel, request_id).await
}

pub struct SynchronousInvocationStrategy;

impl SynchronousInvocationStrategy {
  #[inline(always)]
  pub async fn endpoint_not_found<CMD, STA>(actor: &mut RawActor<CMD, STA>, request: InboundRequest)
  where
    CMD: NetCommanderMut,
    STA: ActorStateRef,
  {
    let response: Result<Vec<u8>, RemoteSendError> =
      Err(RemoteSendError::UnexpectedRequest(request.endpoint.to_string()));

    let send_result = send_response(
      actor.commander(),
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
  pub async fn handler_deserialization_failure<CMD, STA>(
    actor: &mut RawActor<CMD, STA>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
    error: RemoteSendError,
  ) -> ActorResult<Result<(), InboundFailure>>
  where
    CMD: NetCommanderMut,
    STA: ActorStateRef,
  {
    send_response(
      actor.commander(),
      Result::<Vec<u8>, RemoteSendError>::Err(error),
      channel,
      request_id,
    )
    .await
  }

  #[inline(always)]
  pub async fn invoke_handler(
    handler: &dyn SyncRequestHandler,
    mut actor: Actor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  ) {
    let mut commander = actor.commander().clone();
    let endpoint = context.endpoint.clone();

    match handler.invoke(actor.clone(), context, object, input) {
      Ok(invocation) => {
        let result = invocation.await;
        let ser_res = handler.serialize_response(result);

        match ser_res {
          Ok(response) => {
            if let Err(error) = send_response(commander.as_mut(), Ok(response), channel, request_id).await {
              log::error!("unable to respond to synchronous request on endpoint `{endpoint}` due to: {error}");
            }
          }
          Err(err) => {
            if let Err(error) = send_response(
              commander.as_mut(),
              Result::<(), RemoteSendError>::Err(err),
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
          commander.as_mut(),
          Result::<(), RemoteSendError>::Err(err),
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
