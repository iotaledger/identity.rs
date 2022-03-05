// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::result::Result as StdResult;

use crate::didcomm::message::DidCommPlaintextMessage;
use crate::p2p::event_loop::InboundRequest;
use crate::p2p::event_loop::ThreadRequest;
use crate::p2p::messages::ResponseMessage;
use crate::p2p::net_commander::NetCommander;
use crate::traits::RequestHandler;
use crate::Actor;
use crate::RemoteSendError;
use crate::RequestContext;

use libp2p::request_response::InboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;

#[async_trait::async_trait]
pub trait InvocationStrategy {
  #[allow(clippy::too_many_arguments)]
  async fn invoke_handler(
    handler: &dyn RequestHandler,
    actor: Actor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  );

  async fn handler_deserialization_failure(
    actor: &mut Actor,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
    error: RemoteSendError,
  ) -> StdResult<(), InboundFailure>;

  async fn endpoint_not_found(actor: &mut Actor, request: InboundRequest);
}

async fn send_response<T: serde::Serialize>(
  commander: &mut NetCommander,
  response: StdResult<T, RemoteSendError>,
  channel: ResponseChannel<ResponseMessage>,
  request_id: RequestId,
) -> StdResult<(), InboundFailure> {
  let response: Vec<u8> = serde_json::to_vec(&response).unwrap();
  commander.send_response(response, channel, request_id).await
}

pub struct SynchronousInvocationStrategy;

#[async_trait::async_trait]
impl InvocationStrategy for SynchronousInvocationStrategy {
  async fn endpoint_not_found(actor: &mut Actor, request: InboundRequest) {
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

  async fn handler_deserialization_failure(
    actor: &mut Actor,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
    error: RemoteSendError,
  ) -> StdResult<(), InboundFailure> {
    send_response(
      &mut actor.commander,
      StdResult::<Vec<u8>, RemoteSendError>::Err(error),
      channel,
      request_id,
    )
    .await
  }

  async fn invoke_handler(
    handler: &dyn RequestHandler,
    actor: Actor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  ) {
    let mut commander = actor.commander.clone();
    let endpoint = context.endpoint.clone();

    match handler.invoke(actor, context, object, input) {
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

pub struct AsynchronousInvocationStrategy;

#[async_trait::async_trait]
impl InvocationStrategy for AsynchronousInvocationStrategy {
  async fn endpoint_not_found(actor: &mut Actor, request: InboundRequest) {
    let result: StdResult<(), RemoteSendError> =
      match serde_json::from_slice::<DidCommPlaintextMessage<serde_json::Value>>(&request.input) {
        Err(error) => Err(RemoteSendError::DeserializationFailure {
          location: "[generic DCPM deserialization]".to_owned(),
          message: error.to_string(),
        }),
        Ok(plaintext_msg) => {
          let thread_id = plaintext_msg.thread_id();

          match actor.state.threads_sender.remove(thread_id) {
            Some(sender) => {
              let thread_request = ThreadRequest {
                peer_id: request.peer_id,
                endpoint: request.endpoint,
                input: request.input,
              };

              if sender.1.send(thread_request).is_err() {
                log::warn!("unable to send request for thread id `{thread_id}`");
              }

              Ok(())
            }
            None => {
              log::info!(
                "no handler or thread found for the received message `{}`",
                request.endpoint
              );
              // The assumption is that DID authentication is done before this point, so this is not
              // considered an information leak, e.g. to enumerate thread ids.
              Err(RemoteSendError::UnexpectedRequest(format!(
                "thread id `{}` not found",
                thread_id
              )))
            }
          }
        }
      };

    let send_result = send_response(
      &mut actor.commander,
      result,
      request.response_channel,
      request.request_id,
    )
    .await;

    if let Err(err) = send_result {
      log::error!("could not acknowledge request due to: {err:?}",);
    }
  }

  async fn handler_deserialization_failure(
    actor: &mut Actor,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
    error: RemoteSendError,
  ) -> StdResult<(), InboundFailure> {
    send_response(
      &mut actor.commander,
      StdResult::<(), RemoteSendError>::Err(error),
      channel,
      request_id,
    )
    .await
  }

  async fn invoke_handler(
    handler: &dyn RequestHandler,
    mut actor: Actor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  ) {
    let send_result = send_response(&mut actor.commander, Ok(()), channel, request_id).await;

    // TODO: If error, should we abort handling this request?
    if let Err(err) = send_result {
      log::error!(
        "could not acknowledge request on endpoint `{}` due to: {err:?}",
        context.endpoint
      );
    }

    match handler.invoke(actor, context, object, input) {
      Ok(invocation) => {
        // Invocation result is () in async handlers.
        let _ = invocation.await;
      }
      Err(err) => {
        log::error!("{}", err);
      }
    }
  }
}
