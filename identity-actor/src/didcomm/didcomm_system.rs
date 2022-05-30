// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use futures::channel::oneshot;
use identity_iota_core::document::IotaDocument;
use libp2p::Multiaddr;
use libp2p::PeerId;
use serde::de::DeserializeOwned;

use crate::actor::AsyncActorRequest;
use crate::actor::Endpoint;
use crate::actor::Error;
use crate::actor::ErrorLocation;
use crate::actor::ObjectId;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;
use crate::actor::RequestMode;
use crate::actor::Result as ActorResult;
use crate::actor::SyncActorRequest;
use crate::actor::System;
use crate::didcomm::message::DidCommPlaintextMessage;
use crate::didcomm::termination::DidCommTermination;
use crate::didcomm::traits::AsyncRequestHandler;
use crate::didcomm::AbstractAsyncActor;
use crate::didcomm::ThreadId;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::RequestMessage;
use crate::p2p::ThreadRequest;

/// The identity of a
pub struct ActorIdentity {
  // TODO: This type is meant to be used in a future update.
  #[allow(dead_code)]
  pub(crate) document: IotaDocument,
}

pub struct DidCommSystemState {
  pub(crate) async_handlers: AsyncHandlerMap,
  pub(crate) actors: AsyncActorMap,
  pub(crate) threads_receiver: DashMap<ThreadId, oneshot::Receiver<ThreadRequest>>,
  pub(crate) threads_sender: DashMap<ThreadId, oneshot::Sender<ThreadRequest>>,
  // TODO: See above.
  #[allow(dead_code)]
  pub(crate) identity: ActorIdentity,
}

impl DidCommSystemState {
  pub(crate) fn new(async_handlers: AsyncHandlerMap, actors: AsyncActorMap, identity: ActorIdentity) -> Self {
    Self {
      async_handlers,
      actors,
      threads_receiver: DashMap::new(),
      threads_sender: DashMap::new(),
      identity,
    }
  }
}

#[derive(Clone)]
pub struct DidCommSystem {
  pub(crate) actor: System,
  pub(crate) state: Arc<DidCommSystemState>,
}

impl DidCommSystem {
  pub(crate) fn commander_mut(&mut self) -> &mut NetCommander {
    self.actor.commander_mut()
  }

  /// Let this actor handle the given `request`, by invoking a handler function.
  /// This consumes the actor because it passes the actor to the handler.
  /// The actor will thus typically be cloned before calling this method.
  pub(crate) fn handle_request(self, request: InboundRequest) {
    match request.request_mode {
      RequestMode::Asynchronous => self.handle_async_request(request),
      RequestMode::Synchronous => self.actor.handle_sync_request(request),
    }
  }

  /// See [`Actor::start_listening`].
  pub async fn start_listening(&mut self, address: Multiaddr) -> ActorResult<Multiaddr> {
    self.actor.start_listening(address).await
  }

  /// See [`Actor::peer_id`].
  pub fn peer_id(&self) -> PeerId {
    self.actor.peer_id()
  }

  /// See [`Actor::addresses`].
  pub async fn addresses(&mut self) -> ActorResult<Vec<Multiaddr>> {
    self.actor.addresses().await
  }

  /// See [`Actor::add_address`].
  pub async fn add_address(&mut self, peer_id: PeerId, address: Multiaddr) -> ActorResult<()> {
    self.actor.add_address(peer_id, address).await
  }

  /// See [`Actor::add_addresses`].
  pub async fn add_addresses(&mut self, peer_id: PeerId, addresses: Vec<Multiaddr>) -> ActorResult<()> {
    self.actor.add_addresses(peer_id, addresses).await
  }

  /// See [`Actor::shutdown`].
  pub async fn shutdown(self) -> ActorResult<()> {
    self.actor.shutdown().await
  }

  pub(crate) fn get_async_handler(&self, endpoint: &Endpoint) -> Result<AsyncHandlerObjectTuple<'_>, RemoteSendError> {
    match self.state.async_handlers.get(endpoint) {
      Some(handler_object) => {
        let object_id = handler_object.object_id;

        if let Some(object) = self.actor.state().objects.get(&object_id) {
          let object_clone = handler_object.handler.clone_object(object)?;
          Ok((handler_object, object_clone))
        } else {
          Err(RemoteSendError::HandlerInvocationError(format!(
            "no state set for {}",
            endpoint
          )))
        }
      }
      None => Err(RemoteSendError::UnexpectedRequest(endpoint.to_string())),
    }
  }

  /// See [`Actor::send_request`].
  pub async fn send_request<REQ: SyncActorRequest>(
    &mut self,
    peer: PeerId,
    request: REQ,
  ) -> ActorResult<REQ::Response> {
    self.actor.send_request(peer, request).await
  }

  /// Sends an asynchronous message to a peer. To receive a potential response, use [`DidCommActor::await_message`],
  /// with the same `thread_id`.
  pub async fn send_message<REQ: AsyncActorRequest>(
    &mut self,
    peer: PeerId,
    thread_id: &ThreadId,
    message: REQ,
  ) -> ActorResult<()> {
    let endpoint: Endpoint = REQ::endpoint();
    let request_mode: RequestMode = REQ::request_mode();

    let dcpm = DidCommPlaintextMessage::new(thread_id.to_owned(), endpoint.to_string(), message);

    let dcpm = self.call_send_message_hook(peer, dcpm).await?;

    self.create_thread_channels(thread_id);

    let dcpm_vec = serde_json::to_vec(&dcpm).map_err(|err| Error::SerializationFailure {
      location: ErrorLocation::Local,
      context: "send message".to_owned(),
      error_message: err.to_string(),
    })?;

    log::debug!("Sending `{}` message", endpoint);
    let message: RequestMessage = RequestMessage::new(endpoint, request_mode, dcpm_vec);

    let response = self.commander_mut().send_request(peer, message).await?;

    serde_json::from_slice::<Result<(), RemoteSendError>>(&response.0).map_err(|err| {
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
  /// to [`DidCommActorBuilder::timeout`](crate::didcomm::DidCommActorBuilder::timeout).
  pub async fn await_message<T: DeserializeOwned + Send + 'static>(
    &mut self,
    thread_id: &ThreadId,
  ) -> ActorResult<DidCommPlaintextMessage<T>> {
    if let Some(receiver) = self.state.threads_receiver.remove(thread_id) {
      // Receival + Deserialization
      let inbound_request = tokio::time::timeout(self.actor.state().config.timeout, receiver.1)
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
      let hook_endpoint: Endpoint = inbound_request.endpoint.into_hook();

      if self.state.async_handlers.contains_key(&hook_endpoint) {
        log::debug!("Calling hook: {}", hook_endpoint);

        let hook_result: Result<Result<DidCommPlaintextMessage<T>, DidCommTermination>, RemoteSendError> =
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
    log::debug!("creating thread channels for {thread_id}");
    let (sender, receiver) = oneshot::channel();

    // The logic is that for every received message on a thread,
    // there must be a preceding send_message on that same thread.
    // Note that on the receiving actor, the very first message of a protocol
    // is not awaited through await_message, so it does not need to follow that logic.
    self.state.threads_sender.insert(thread_id.to_owned(), sender);
    self.state.threads_receiver.insert(thread_id.to_owned(), receiver);
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
      match self.state.actors.get(&request.endpoint) {
        Some(actor) => {
          let handler: &dyn AbstractAsyncActor = actor.as_ref();

          handler.handle(self.clone(), request).await;
        }
        None => {
          AsynchronousInvocationStrategy::endpoint_not_found(&mut self, request).await;
        }
      }
    });
  }

  #[inline(always)]
  pub(crate) async fn call_send_message_hook<REQ: AsyncActorRequest>(
    &self,
    peer: PeerId,
    input: REQ,
  ) -> ActorResult<REQ> {
    let endpoint: Endpoint = REQ::endpoint().into_hook();

    if self.state.async_handlers.contains_key(&endpoint) {
      log::debug!("Calling send hook: {}", endpoint);

      let hook_result: Result<Result<REQ, DidCommTermination>, RemoteSendError> =
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
  pub(crate) async fn call_hook<I, O>(&self, endpoint: Endpoint, peer: PeerId, input: I) -> Result<O, RemoteSendError>
  where
    I: Send + 'static,
    O: 'static,
  {
    match self.get_async_handler(&endpoint) {
      Ok(handler_object) => {
        let handler: &dyn AsyncRequestHandler = handler_object.0.handler.as_ref();
        let state: Box<dyn Any + Send + Sync> = handler_object.1;
        let type_erased_input: Box<dyn Any + Send> = Box::new(input);
        let request_context = RequestContext::new((), peer, endpoint);

        let result = handler
          .invoke(self.clone(), request_context, state, type_erased_input)?
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
  async fn endpoint_not_found(actor: &mut DidCommSystem, request: InboundRequest) {
    let result: Result<(), RemoteSendError> =
      match serde_json::from_slice::<DidCommPlaintextMessage<serde_json::Value>>(&request.input) {
        Err(error) => Err(RemoteSendError::DeserializationFailure {
          location: ErrorLocation::Remote,
          context: "DIDComm plaintext message deserialization".to_owned(),
          error_message: error.to_string(),
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

    let send_result = crate::actor::send_response(
      actor.commander_mut(),
      result,
      request.response_channel,
      request.request_id,
    )
    .await;

    if let Err(err) = send_result {
      log::error!("could not acknowledge request due to: {err:?}",);
    }
  }
}

/// An [`AsyncRequestHandler`] and the id of its associated shared state object.
pub(crate) struct AsyncHandlerObject {
  pub(crate) handler: Box<dyn AsyncRequestHandler>,
  pub(crate) object_id: ObjectId,
}

impl AsyncHandlerObject {
  pub(crate) fn new(object_id: ObjectId, handler: Box<dyn AsyncRequestHandler>) -> Self {
    Self { object_id, handler }
  }
}

/// A map from an endpoint to the identifier of the shared state object
/// and the method that handles that particular request.
pub(crate) type AsyncHandlerMap = HashMap<Endpoint, AsyncHandlerObject>;
pub type AsyncActorMap = HashMap<Endpoint, Box<dyn AbstractAsyncActor>>;

pub(crate) type AsyncHandlerObjectTuple<'a> = (&'a AsyncHandlerObject, Box<dyn Any + Send + Sync>);
