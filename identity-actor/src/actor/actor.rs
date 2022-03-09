// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::collections::HashMap;
use std::result::Result as StdResult;
use std::sync::Arc;

use crate::didcomm::message::DidCommPlaintextMessage;
use crate::didcomm::termination::DidCommTermination;
use crate::didcomm::thread_id::ThreadId;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::RequestMessage;
use crate::p2p::ThreadRequest;
use crate::ActorConfig;
use crate::ActorRequest;
use crate::Asynchronous;
use crate::Endpoint;
use crate::Error;
use crate::InvocationStrategy;
use crate::RemoteSendError;
use crate::RequestContext;
use crate::RequestHandler;
use crate::RequestMode;
use crate::Result;
use crate::SyncMode;
use crate::Synchronous;

use dashmap::DashMap;
use futures::channel::oneshot;
use identity_core::common::OneOrMany;
use libp2p::Multiaddr;
use libp2p::PeerId;
use libp2p::TransportError;
use serde::de::DeserializeOwned;
use uuid::Uuid;

pub(crate) struct ActorState {
  pub(crate) handlers: HandlerMap,
  pub(crate) objects: ObjectMap,
  pub(crate) threads_receiver: DashMap<ThreadId, oneshot::Receiver<ThreadRequest>>,
  pub(crate) threads_sender: DashMap<ThreadId, oneshot::Sender<ThreadRequest>>,
  pub(crate) peer_id: PeerId,
  pub(crate) config: ActorConfig,
}

/// The main type of the actor crate, used to send messages to remote actors.
/// Cloning an actor produces a shallow copy and is a relatively cheap operation.
#[derive(Clone)]
pub struct Actor {
  pub(crate) commander: NetCommander,
  pub(crate) state: Arc<ActorState>,
}

impl Actor {
  pub(crate) async fn from_builder(
    commander: NetCommander,
    handlers: HandlerMap,
    objects: ObjectMap,
    peer_id: PeerId,
    config: ActorConfig,
  ) -> Result<Self> {
    let actor = Self {
      commander,
      state: Arc::new(ActorState {
        handlers,
        objects,
        threads_receiver: DashMap::new(),
        threads_sender: DashMap::new(),
        peer_id,
        config,
      }),
    };

    Ok(actor)
  }

  fn handlers(&self) -> &HandlerMap {
    &self.state.as_ref().handlers
  }

  // TODO: Return crate::Result?
  /// Start listening on the given `address`. Returns the first address that the actor started listening on, which may
  /// be different from `address` itself, e.g. when passing addresses like `/ip4/0.0.0.0/tcp/0`. Even when passing a
  /// single address, multiple addresses may end up being listened on. To obtain all those addresses, use
  /// [`Actor::addresses`]. Note that even when the same address is passed, the returned address is not deterministic,
  /// and should thus not be relied upon.
  pub async fn start_listening(&mut self, address: Multiaddr) -> StdResult<Multiaddr, TransportError<std::io::Error>> {
    self.commander.start_listening(address).await
  }

  /// Returns the [`PeerId`] that other peers can securely identify this actor with.
  pub fn peer_id(&self) -> PeerId {
    self.state.peer_id
  }

  /// Return all addresses that are currently being listened on.
  pub async fn addresses(&mut self) -> Vec<Multiaddr> {
    self.commander.get_addresses().await
  }

  #[inline(always)]
  pub(crate) fn handle_request<STR: InvocationStrategy + Send + Sync + 'static>(mut self, request: InboundRequest) {
    let _ = tokio::spawn(async move {
      if self.state.handlers.contains_key(&request.endpoint) {
        let mut actor = self.clone();

        match self.get_handler(&request.endpoint).and_then(|handler_ref| {
          let input = handler_ref.0.handler.deserialize_request(request.input)?;
          Ok((handler_ref.0, handler_ref.1, input))
        }) {
          Ok((handler_ref, object, input)) => {
            let handler: &dyn RequestHandler = handler_ref.handler.as_ref();

            let request_context: RequestContext<()> = RequestContext::new((), request.peer_id, request.endpoint);

            STR::invoke_handler(
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

            let result =
              STR::handler_deserialization_failure(&mut actor, request.response_channel, request.request_id, error)
                .await;

            if let Err(err) = result {
              log::error!(
                "could not send error for request on endpoint `{}` due o: {err:?}",
                request.endpoint
              );
            }
          }
        }
      } else {
        STR::endpoint_not_found(&mut self, request).await;
      }
    });
  }

  fn get_handler(&self, endpoint: &Endpoint) -> StdResult<HandlerObjectTuple<'_>, RemoteSendError> {
    match self.state.handlers.get(endpoint) {
      Some(handler_object) => {
        let object_id = handler_object.object_id;

        if let Some(object) = self.state.objects.get(&object_id) {
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

  /// Shutdown this actor. The actor will stop listening on all addresses.
  /// This will break the event loop in the background immediately, returning an error
  /// for all current handlers that interact with their copy of the actor or those waiting on messages.
  pub async fn shutdown(mut self) -> Result<()> {
    self.commander.shutdown().await;
    // Consuming self drops the internal commander. If this is the last copy of the commander,
    // the event loop will break as a result. However, if copies exist, such as in running handlers,
    // this will return while the event loop keeps running. Ideally we could then join on the background task
    // to wait for all handlers to finish gracefully. However, not all spawn functions return a JoinHandle,
    // such as wasm_bindgen_futures::spawn_local. The current alternative is to use a non-graceful exit,
    // which breaks the event loop immediately and returns an error through all open channels that require a result.

    Ok(())
  }

  /// Associate the given `peer_id` with an `address`. This needs to be done before sending a
  /// request to some [`PeerId`].
  pub async fn add_address(&mut self, peer_id: PeerId, address: Multiaddr) {
    self.commander.add_addresses(peer_id, OneOrMany::One(address)).await;
  }

  /// Associate the given `peer_id` with multiple `addresses`. This needs to be done before sending a
  /// request to some [`PeerId`].
  pub async fn add_addresses(&mut self, peer_id: PeerId, addresses: Vec<Multiaddr>) {
    self.commander.add_addresses(peer_id, OneOrMany::Many(addresses)).await;
  }

  /// Sends an asynchronous message to a peer. To receive a potential response, use [`Actor::await_message`],
  /// with the same `thread_id`.
  pub async fn send_message<REQ: ActorRequest<Asynchronous>>(
    &mut self,
    peer: PeerId,
    thread_id: &ThreadId,
    command: REQ,
  ) -> Result<()> {
    self
      .send_named_message(peer, &command.endpoint(), thread_id, command)
      .await
  }

  #[doc(hidden)]
  /// Helper function for bindings, prefer [`Actor::send_message`] whenever possible.
  pub(crate) async fn send_named_message<REQ: ActorRequest<Asynchronous>>(
    &mut self,
    peer: PeerId,
    name: &str,
    thread_id: &ThreadId,
    message: REQ,
  ) -> Result<()> {
    let request_mode: RequestMode = message.request_mode();

    let dcpm = DidCommPlaintextMessage::new(thread_id.to_owned(), name.to_owned(), message);

    let dcpm = self.call_send_message_hook(peer, dcpm).await?;

    self.create_thread_channels(thread_id);

    let dcpm_vec = serde_json::to_vec(&dcpm).map_err(|err| Error::SerializationFailure {
      // TODO: Could use `function_name` crate for these errors. Necessary?
      location: "[send_named_message]".to_owned(),
      message: err.to_string(),
    })?;

    let message = RequestMessage::new(name, request_mode, dcpm_vec)?;

    log::debug!("Sending `{}` message", name);

    let response = self.commander.send_request(peer, message).await?;

    serde_json::from_slice::<StdResult<(), RemoteSendError>>(&response.0).map_err(|err| {
      Error::DeserializationFailure {
        location: "[send_named_message]".to_owned(),
        message: err.to_string(),
      }
    })??;

    Ok(())
  }

  /// Sends a synchronous request to a peer and returns its response.
  pub async fn send_request<REQ: ActorRequest<Synchronous>>(
    &mut self,
    peer: PeerId,
    message: REQ,
  ) -> Result<REQ::Response> {
    self
      .send_named_request(peer, message.endpoint().as_ref(), message)
      .await
  }

  #[doc(hidden)]
  /// Helper function for bindings, prefer [`Actor::send_request`] whenever possible.
  pub(crate) async fn send_named_request<REQ: ActorRequest<Synchronous>>(
    &mut self,
    peer: PeerId,
    name: &str,
    request: REQ,
  ) -> Result<REQ::Response> {
    let request_mode: RequestMode = request.request_mode();

    let request_vec = serde_json::to_vec(&request).map_err(|err| Error::SerializationFailure {
      location: "[send_named_request]".to_owned(),
      message: err.to_string(),
    })?;

    let message = RequestMessage::new(name, request_mode, request_vec)?;

    log::debug!("Sending `{}` message", name);

    let response = self.commander.send_request(peer, message).await?;

    let response: Vec<u8> =
      serde_json::from_slice::<StdResult<Vec<u8>, RemoteSendError>>(&response.0).map_err(|err| {
        Error::DeserializationFailure {
          location: "[send_named_request]".to_owned(),
          message: err.to_string(),
        }
      })??;

    serde_json::from_slice::<REQ::Response>(&response).map_err(|err| Error::DeserializationFailure {
      location: "[send_named_request]".to_owned(),
      message: err.to_string(),
    })
  }

  #[inline(always)]
  async fn call_send_message_hook<MOD: SyncMode, REQ: ActorRequest<MOD>>(
    &self,
    peer: PeerId,
    input: REQ,
  ) -> Result<REQ> {
    let mut endpoint = Endpoint::new(input.endpoint())?;
    endpoint.is_hook = true;

    if self.handlers().contains_key(&endpoint) {
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

  /// Wait for a message on a given `thread_id`. This can only be called successfully if
  /// [`Actor::send_message`] was used previously. This will return a timeout error if no message
  /// is received within the duration passed to [`ActorBuilder::timeout`](crate::ActorBuilder::timeout).
  pub async fn await_message<T: DeserializeOwned + Send + 'static>(
    &mut self,
    thread_id: &ThreadId,
  ) -> Result<DidCommPlaintextMessage<T>> {
    if let Some(receiver) = self.state.threads_receiver.remove(thread_id) {
      // Receival + Deserialization
      let inbound_request = tokio::time::timeout(self.state.config.timeout, receiver.1)
        .await
        .map_err(|_| Error::AwaitTimeout(receiver.0.clone()))?
        .map_err(|_| Error::ThreadNotFound(receiver.0))?;

      let message: DidCommPlaintextMessage<T> =
        serde_json::from_slice(inbound_request.input.as_ref()).map_err(|err| Error::DeserializationFailure {
          location: "[await_message]".to_owned(),
          message: err.to_string(),
        })?;

      log::debug!("awaited message {}", inbound_request.endpoint);

      // Hooking
      let mut hook_endpoint: Endpoint = inbound_request.endpoint;
      hook_endpoint.is_hook = true;

      if self.handlers().contains_key(&hook_endpoint) {
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
    self.state.threads_sender.insert(thread_id.to_owned(), sender);
    self.state.threads_receiver.insert(thread_id.to_owned(), receiver);
  }

  /// Call the hook identified by the given `endpoint` with some `input`.
  pub async fn call_hook<I, O>(&self, endpoint: Endpoint, peer: PeerId, input: I) -> StdResult<O, RemoteSendError>
  where
    I: Send + 'static,
    O: 'static,
  {
    match self.get_handler(&endpoint) {
      Ok(handler_object) => {
        let handler: &dyn RequestHandler = handler_object.0.handler.as_ref();
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

/// A map from an identifier to an object that contains the
/// shared state of the associated handler functions.
pub(crate) type ObjectMap = HashMap<ObjectId, Box<dyn Any + Send + Sync>>;

/// An actor-internal identifier for the object representing the shared state of one or more handlers.
pub(crate) type ObjectId = Uuid;

/// A [`RequestHandler`] and the id of its associated shared state object.
pub(crate) struct HandlerObject {
  handler: Box<dyn RequestHandler>,
  object_id: ObjectId,
}

impl HandlerObject {
  pub(crate) fn new(object_id: ObjectId, handler: Box<dyn RequestHandler>) -> Self {
    Self { object_id, handler }
  }
}

/// A map from an endpoint to the identifier of the shared state object
/// and the method that handles that particular request.
pub(crate) type HandlerMap = HashMap<Endpoint, HandlerObject>;

pub(crate) type HandlerObjectTuple<'a> = (&'a HandlerObject, Box<dyn Any + Send + Sync>);
