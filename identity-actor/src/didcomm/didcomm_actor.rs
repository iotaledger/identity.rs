// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::any::Any;
use std::result::Result as StdResult;

use crate::didcomm::message::DidCommPlaintextMessage;
use crate::didcomm::termination::DidCommTermination;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::RequestMessage;
use crate::p2p::ResponseMessage;
use crate::p2p::ThreadRequest;
use crate::send_response;
use crate::Actor;
use crate::ActorConfig;
use crate::ActorRequest;
use crate::ActorStateExtension;
use crate::Asynchronous;
use crate::Endpoint;
use crate::Error;
use crate::GenericActor;
use crate::HandlerMap;
use crate::ObjectMap;
use crate::RemoteSendError;
use crate::RequestContext;
use crate::RequestHandler;
use crate::RequestMode;
use crate::Result;
use crate::Synchronous;

use dashmap::DashMap;
use futures::channel::oneshot;
use identity_iota_core::document::IotaDocument;
use libp2p::request_response::InboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;
use libp2p::Multiaddr;
use libp2p::PeerId;
use serde::de::DeserializeOwned;

use crate::ErrorLocation;

use super::thread_id::ThreadId;

pub struct ActorIdentity {
  // TODO: This type is meant for illustrating the state extension mechanism
  // and will be used in a future update.
  #[allow(dead_code)]
  pub(crate) document: IotaDocument,
}

pub struct DidCommStateExtension {
  pub(crate) threads_receiver: DashMap<ThreadId, oneshot::Receiver<ThreadRequest>>,
  pub(crate) threads_sender: DashMap<ThreadId, oneshot::Sender<ThreadRequest>>,
  // TODO: See above.
  #[allow(dead_code)]
  pub(crate) identity: ActorIdentity,
}

impl DidCommStateExtension {
  pub fn new(identity: ActorIdentity) -> Self {
    Self {
      threads_receiver: DashMap::new(),
      threads_sender: DashMap::new(),
      identity,
    }
  }
}

impl crate::traits::state_extension_seal::Sealed for DidCommStateExtension {}
impl ActorStateExtension for DidCommStateExtension {}

#[derive(Clone)]
pub struct DidCommActor(Actor<DidCommStateExtension>);

impl GenericActor for DidCommActor {
  type Extension = DidCommStateExtension;

  fn from_actor_builder(
    handlers: HandlerMap,
    objects: ObjectMap,
    config: ActorConfig,
    peer_id: PeerId,
    commander: NetCommander,
    extension: Self::Extension,
  ) -> crate::Result<Self> {
    Actor::<Self::Extension>::from_builder(handlers, objects, config, peer_id, commander, extension).map(Self)
  }

  fn handle_request(self, request: InboundRequest) {
    if request.request_mode == RequestMode::Asynchronous {
      self.handle_async_request(request)
    } else {
      self.0.handle_sync_request(request)
    }
  }
}

impl DidCommActor {
  // TODO: Is there an automated way to copy docs from the delegated fns?

  /// See [`Actor::start_listening`].
  pub async fn start_listening(&mut self, address: Multiaddr) -> crate::Result<Multiaddr> {
    self.0.start_listening(address).await
  }

  /// See [`Actor::peer_id`].
  pub fn peer_id(&self) -> PeerId {
    self.0.peer_id()
  }

  /// See [`Actor::addresses`].
  pub async fn addresses(&mut self) -> crate::Result<Vec<Multiaddr>> {
    self.0.addresses().await
  }

  /// See [`Actor::add_address`].
  pub async fn add_address(&mut self, peer_id: PeerId, address: Multiaddr) -> crate::Result<()> {
    self.0.add_address(peer_id, address).await
  }

  /// See [`Actor::add_addresses`].
  pub async fn add_addresses(&mut self, peer_id: PeerId, addresses: Vec<Multiaddr>) -> crate::Result<()> {
    self.0.add_addresses(peer_id, addresses).await
  }

  /// See [`Actor::shutdown`].
  pub async fn shutdown(self) -> Result<()> {
    self.0.shutdown().await
  }

  /// See [`Actor::send_request`].
  pub async fn send_request<REQ: ActorRequest<Synchronous>>(
    &mut self,
    peer: PeerId,
    request: REQ,
  ) -> Result<REQ::Response> {
    self.0.send_request(peer, request).await
  }

  /// Sends an asynchronous message to a peer. To receive a potential response, use [`DidCommActor::await_message`],
  /// with the same `thread_id`.
  pub async fn send_message<REQ: ActorRequest<Asynchronous>>(
    &mut self,
    peer: PeerId,
    thread_id: &ThreadId,
    message: REQ,
  ) -> Result<()> {
    let endpoint: &'static str = REQ::endpoint();
    let request_mode: RequestMode = message.request_mode();

    let dcpm = DidCommPlaintextMessage::new(thread_id.to_owned(), endpoint.to_owned(), message);

    let dcpm = self.call_send_message_hook(peer, dcpm).await?;

    self.create_thread_channels(thread_id);

    let dcpm_vec = serde_json::to_vec(&dcpm).map_err(|err| Error::SerializationFailure {
      location: ErrorLocation::Local,
      context: "send message".to_owned(),
      error_message: err.to_string(),
    })?;

    let message = RequestMessage::new(endpoint, request_mode, dcpm_vec)?;

    log::debug!("Sending `{}` message", endpoint);

    let response = self.0.commander.send_request(peer, message).await?;

    serde_json::from_slice::<StdResult<(), RemoteSendError>>(&response.0).map_err(|err| {
      Error::DeserializationFailure {
        location: ErrorLocation::Local,
        context: "send message".to_owned(),
        error_message: err.to_string(),
      }
    })??;

    Ok(())
  }

  /// Wait for a message on a given `thread_id`. This can only be called successfully if
  /// [`DidCommActor::send_message`] was called on the same `thread_id` previously.
  /// This will return a timeout error if no message is received within the duration passed
  /// to [`DidCommActorBuilder::timeout`](crate::didcomm::didcomm_actor_builder::DidCommActorBuilder::timeout).
  pub async fn await_message<T: DeserializeOwned + Send + 'static>(
    &mut self,
    thread_id: &ThreadId,
  ) -> Result<DidCommPlaintextMessage<T>> {
    if let Some(receiver) = self.0.state.extension.threads_receiver.remove(thread_id) {
      // Receival + Deserialization
      let inbound_request = tokio::time::timeout(self.0.state.config.timeout, receiver.1)
        .await
        .map_err(|_| Error::AwaitTimeout(receiver.0.clone()))?
        .map_err(|_| Error::ThreadNotFound(receiver.0))?;

      let message: DidCommPlaintextMessage<T> =
        serde_json::from_slice(inbound_request.input.as_ref()).map_err(|err| Error::DeserializationFailure {
          location: ErrorLocation::Local,
          context: "await message".to_owned(),
          error_message: err.to_string(),
        })?;

      log::debug!("awaited message {}", inbound_request.endpoint);

      // Hooking
      let mut hook_endpoint: Endpoint = inbound_request.endpoint;
      hook_endpoint.is_hook = true;

      if self.0.handlers().contains_key(&hook_endpoint) {
        log::debug!("Calling hook: {}", hook_endpoint);

        let hook_result: StdResult<StdResult<DidCommPlaintextMessage<T>, DidCommTermination>, RemoteSendError> =
          self.call_hook(hook_endpoint, inbound_request.peer_id, message).await;

        match hook_result {
          Ok(Ok(request)) => Ok(request),
          Ok(Err(_)) => {
            unimplemented!("didcomm termination");
          }
          Err(err) => Err(err.into()),
        }
      } else {
        Ok(message)
      }
    } else {
      log::warn!("attempted to wait for a message on thread {thread_id:?}, which does not exist");
      Err(Error::ThreadNotFound(thread_id.to_owned()))
    }
  }

  /// Creates the channels used to await a message on a thread.
  fn create_thread_channels(&mut self, thread_id: &ThreadId) {
    let (sender, receiver) = oneshot::channel();

    // The logic is that for every received message on a thread,
    // there must be a preceding send_message on that same thread.
    // Note that on the receiving actor, the very first message of a protocol
    // is not awaited through await_message, so it does not need to follow that logic.
    self
      .0
      .state
      .extension
      .threads_sender
      .insert(thread_id.to_owned(), sender);
    self
      .0
      .state
      .extension
      .threads_receiver
      .insert(thread_id.to_owned(), receiver);
  }

  #[inline(always)]
  pub(crate) fn handle_async_request(mut self, request: InboundRequest) {
    cfg_if::cfg_if! {
      if #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))] {
        let spawn = tokio::spawn;
      } else {
        let spawn = wasm_bindgen_futures::spawn_local;
      }
    }

    let _ = spawn(async move {
      if self.0.state.handlers.contains_key(&request.endpoint) {
        let mut actor = self.clone();

        match self.0.get_handler(&request.endpoint).and_then(|handler_ref| {
          let input = handler_ref.0.handler.deserialize_request(request.input)?;
          Ok((handler_ref.0, handler_ref.1, input))
        }) {
          Ok((handler_ref, object, input)) => {
            let handler: &dyn RequestHandler = handler_ref.handler.as_ref();

            let request_context: RequestContext<()> = RequestContext::new((), request.peer_id, request.endpoint);

            AsynchronousInvocationStrategy::invoke_handler(
              handler,
              actor,
              request_context,
              object,
              input,
              request.response_channel,
              request.request_id,
            )
            .await;
          }
          Err(error) => {
            log::debug!("handler error: {error:?}");

            let result = AsynchronousInvocationStrategy::handler_deserialization_failure(
              &mut actor,
              request.response_channel,
              request.request_id,
              error,
            )
            .await;

            match result {
              Ok(Err(err)) => {
                log::error!(
                  "could not send error for request on endpoint `{}` due to: {err:?}",
                  request.endpoint
                );
              }
              Err(err) => {
                log::error!(
                  "could not send error for request on endpoint `{}` due to: {err:?}",
                  request.endpoint
                );
              }
              Ok(_) => (),
            }
          }
        }
      } else {
        AsynchronousInvocationStrategy::endpoint_not_found(&mut self, request).await;
      }
    });
  }

  #[inline(always)]
  pub(crate) async fn call_send_message_hook<REQ: ActorRequest<Asynchronous>>(
    &self,
    peer: PeerId,
    input: REQ,
  ) -> Result<REQ> {
    let mut endpoint = Endpoint::new(REQ::endpoint())?;
    endpoint.is_hook = true;

    if self.0.handlers().contains_key(&endpoint) {
      log::debug!("Calling send hook: {}", endpoint);

      let hook_result: StdResult<StdResult<REQ, DidCommTermination>, RemoteSendError> =
        self.call_hook(endpoint, peer, input).await;

      match hook_result {
        Ok(Ok(request)) => Ok(request),
        Ok(Err(_)) => {
          unimplemented!("didcomm termination");
        }
        Err(err) => Err(err.into()),
      }
    } else {
      Ok(input)
    }
  }

  /// Call the hook identified by the given `endpoint` with some `input`.
  pub(crate) async fn call_hook<I, O>(
    &self,
    endpoint: Endpoint,
    peer: PeerId,
    input: I,
  ) -> StdResult<O, RemoteSendError>
  where
    I: Send + 'static,
    O: 'static,
  {
    match self.0.get_handler(&endpoint) {
      Ok(handler_object) => {
        let handler: &dyn RequestHandler = handler_object.0.handler.as_ref();
        let state: Box<dyn Any + Send + Sync> = handler_object.1;
        let type_erased_input: Box<dyn Any + Send> = Box::new(input);
        let request_context = RequestContext::new((), peer, endpoint);

        let result = handler
          .invoke(Box::new(self.clone()), request_context, state, type_erased_input)?
          .await;

        match result.downcast::<O>() {
          Ok(result) => Ok(*result),
          Err(_) => {
            let err = RemoteSendError::HookInvocationError(format!(
              "hook did not return the expected type: {:?}",
              std::any::type_name::<O>(),
            ));

            Err(err)
          }
        }
      }
      Err(error) => Err(error),
    }
  }
}

pub struct AsynchronousInvocationStrategy;

impl AsynchronousInvocationStrategy {
  #[inline(always)]
  async fn endpoint_not_found(actor: &mut DidCommActor, request: InboundRequest) {
    let result: StdResult<(), RemoteSendError> =
      match serde_json::from_slice::<DidCommPlaintextMessage<serde_json::Value>>(&request.input) {
        Err(error) => Err(RemoteSendError::DeserializationFailure {
          location: ErrorLocation::Remote,
          context: "DIDComm plaintext message deserialization".to_owned(),
          error_message: error.to_string(),
        }),
        Ok(plaintext_msg) => {
          let thread_id = plaintext_msg.thread_id();

          match actor.0.state.extension.threads_sender.remove(thread_id) {
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
      &mut actor.0.commander,
      result,
      request.response_channel,
      request.request_id,
    )
    .await;

    if let Err(err) = send_result {
      log::error!("could not acknowledge request due to: {err:?}",);
    }
  }

  #[inline(always)]
  async fn handler_deserialization_failure(
    actor: &mut DidCommActor,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
    error: RemoteSendError,
  ) -> crate::Result<StdResult<(), InboundFailure>> {
    send_response(
      &mut actor.0.commander,
      StdResult::<(), RemoteSendError>::Err(error),
      channel,
      request_id,
    )
    .await
  }

  #[inline(always)]
  async fn invoke_handler(
    handler: &dyn RequestHandler,
    mut actor: DidCommActor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    input: Box<dyn Any + Send>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  ) {
    let send_result = send_response(&mut actor.0.commander, Ok(()), channel, request_id).await;

    if let Err(err) = send_result {
      log::error!(
        "could not acknowledge request on endpoint `{}` due to: {err:?}",
        context.endpoint
      );

      // Peer seems to be unresponsive, so we do not continue handling this request.
      return;
    }

    match handler.invoke(Box::new(actor), context, object, input) {
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
