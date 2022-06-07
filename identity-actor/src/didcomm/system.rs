// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use futures::channel::oneshot;
use identity_iota_core::document::IotaDocument;
use libp2p::Multiaddr;
use libp2p::PeerId;
use serde::de::DeserializeOwned;

use crate::actor::ActorRequest;
use crate::actor::Endpoint;
use crate::actor::Error;
use crate::actor::ErrorLocation;
use crate::actor::RemoteSendError;
use crate::actor::RequestMode;
use crate::actor::Result as ActorResult;
use crate::actor::System;
use crate::didcomm::dcpm::DidCommPlaintextMessage;
use crate::didcomm::AbstractDidCommActor;
use crate::didcomm::DidCommRequest;
use crate::didcomm::ThreadId;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::RequestMessage;
use crate::p2p::ThreadRequest;

/// The identity of a [`DidCommSystem`].
///
/// Note: Currently an incomplete implementation.
#[derive(Debug, Clone)]
pub struct ActorIdentity {
  // TODO: This type is meant to be used in a future update.
  #[allow(dead_code)]
  pub(crate) document: IotaDocument,
}

/// The internal state of a [`System`].
#[derive(Debug)]
pub struct DidCommSystemState {
  pub(crate) actors: DidCommActorMap,
  pub(crate) threads_receiver: DashMap<ThreadId, oneshot::Receiver<ThreadRequest>>,
  pub(crate) threads_sender: DashMap<ThreadId, oneshot::Sender<ThreadRequest>>,
  // TODO: See above.
  #[allow(dead_code)]
  pub(crate) identity: ActorIdentity,
}

impl DidCommSystemState {
  pub(crate) fn new(actors: DidCommActorMap, identity: ActorIdentity) -> Self {
    Self {
      actors,
      threads_receiver: DashMap::new(),
      threads_sender: DashMap::new(),
      identity,
    }
  }
}

/// An actor system fowards incoming requests to attached actors and can be used to send requests to remote actors.
///
/// An actor system is a frontend for an event loop running in the background, which invokes
/// user-attached actors. Systems can be cloned without cloning the event loop, and doing so
/// is a cheap operation.
/// Actors are attached at system build time, using the [`DidCommSystemBuilder`](crate::didcomm::DidCommSystemBuilder).
///
/// After shutting down the event loop of a system using [`DidCommSystem::shutdown`], other clones of the
/// system will receive [`Error::Shutdown`] when attempting to interact with the event loop.
#[derive(Debug, Clone)]
pub struct DidCommSystem {
  pub(crate) system: System,
  pub(crate) state: Arc<DidCommSystemState>,
}

impl DidCommSystem {
  pub(crate) fn commander_mut(&mut self) -> &mut NetCommander {
    self.system.commander_mut()
  }

  /// Let this actor handle the given `request`, by invoking a handler function.
  /// This consumes the actor because it passes the actor to the handler.
  /// The actor will thus typically be cloned before calling this method.
  pub(crate) fn handle_request(self, request: InboundRequest) {
    match request.request_mode {
      RequestMode::Asynchronous => self.handle_async_request(request),
      RequestMode::Synchronous => self.system.handle_sync_request(request),
    }
  }

  /// See [`System::start_listening`].
  pub async fn start_listening(&mut self, address: Multiaddr) -> ActorResult<Multiaddr> {
    self.system.start_listening(address).await
  }

  /// See [`System::peer_id`].
  pub fn peer_id(&self) -> PeerId {
    self.system.peer_id()
  }

  /// See [`System::addresses`].
  pub async fn addresses(&mut self) -> ActorResult<Vec<Multiaddr>> {
    self.system.addresses().await
  }

  /// See [`System::add_address`].
  pub async fn add_address(&mut self, peer_id: PeerId, address: Multiaddr) -> ActorResult<()> {
    self.system.add_peer_address(peer_id, address).await
  }

  /// See [`System::add_addresses`].
  pub async fn add_addresses(&mut self, peer_id: PeerId, addresses: Vec<Multiaddr>) -> ActorResult<()> {
    self.system.add_peer_addresses(peer_id, addresses).await
  }

  /// See [`System::shutdown`].
  pub async fn shutdown(self) -> ActorResult<()> {
    self.system.shutdown().await
  }

  /// See [`System::send_request`].
  pub async fn send_request<REQ: ActorRequest>(&mut self, peer: PeerId, request: REQ) -> ActorResult<REQ::Response> {
    self.system.send_request(peer, request).await
  }

  /// Sends an asynchronous message to a peer. To receive a potential response, use [`DidCommSystem::await_message`],
  /// with the same `thread_id`.
  pub async fn send_message<REQ: DidCommRequest>(
    &mut self,
    peer: PeerId,
    thread_id: &ThreadId,
    message: REQ,
  ) -> ActorResult<()> {
    let endpoint: Endpoint = REQ::endpoint();
    let request_mode: RequestMode = REQ::request_mode();

    let dcpm = DidCommPlaintextMessage::new(thread_id.to_owned(), endpoint.to_string(), message);

    self.create_thread_channels(thread_id);

    let dcpm_vec = serde_json::to_vec(&dcpm).map_err(|err| Error::SerializationFailure {
      location: ErrorLocation::Local,
      context: "send message".to_owned(),
      error_message: err.to_string(),
    })?;

    log::debug!("sending `{}` message", endpoint);
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
  /// [`DidCommSystem::send_message`] was called on the same `thread_id` previously.
  /// This will return a timeout error if no message is received within the duration passed
  /// to [`DidCommSystemBuilder::timeout`](crate::didcomm::DidCommSystemBuilder::timeout).
  pub async fn await_message<T: DeserializeOwned + Send + 'static>(
    &mut self,
    thread_id: &ThreadId,
  ) -> ActorResult<DidCommPlaintextMessage<T>> {
    if let Some(receiver) = self.state.threads_receiver.remove(thread_id) {
      // Receiving + Deserialization
      let inbound_request = tokio::time::timeout(self.system.state().config.timeout, receiver.1)
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

      Ok(message)
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

  #[inline(always)]
  pub(crate) fn handle_async_request(mut self, request: InboundRequest) {
    let _ = tokio::spawn(async move {
      match self.state.actors.get(&request.endpoint) {
        Some(actor) => {
          let handler: &dyn AbstractDidCommActor = actor.as_ref();

          handler.handle(self.clone(), request).await;
        }
        None => {
          actor_not_found(&mut self, request).await;
        }
      }
    });
  }
}

/// Invoked when no actor was found that can handle the received request.
/// Attempts to find a thread waiting for the received message,
/// otherwise returns an error to the peer.
async fn actor_not_found(actor: &mut DidCommSystem, request: InboundRequest) {
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
    log::error!("could not acknowledge request due to: {err:?}");
  }
}

/// A map from an endpoint to the actor that handles its requests.
pub(crate) type DidCommActorMap = HashMap<Endpoint, Box<dyn AbstractDidCommActor>>;
