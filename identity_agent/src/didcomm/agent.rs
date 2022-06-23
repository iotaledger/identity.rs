// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use futures::channel::oneshot;
use identity_iota_core::document::IotaDocument;
use libp2p::Multiaddr;
use serde::de::DeserializeOwned;

use crate::agent::Agent;
use crate::agent::AgentId;
use crate::agent::Endpoint;
use crate::agent::Error;
use crate::agent::ErrorLocation;
use crate::agent::HandlerRequest;
use crate::agent::RemoteSendError;
use crate::agent::RequestMode;
use crate::agent::Result as AgentResult;
use crate::didcomm::dcpm::DidCommPlaintextMessage;
use crate::didcomm::AbstractDidCommHandler;
use crate::didcomm::DidCommRequest;
use crate::didcomm::ThreadId;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::RequestMessage;
use crate::p2p::ThreadRequest;

/// The identity of a [`DidCommAgent`].
///
/// Note: Currently an incomplete implementation.
#[derive(Debug, Clone)]
pub struct DidCommAgentIdentity {
  // TODO: This type is meant to be used in a future update.
  #[allow(dead_code)]
  pub document: IotaDocument,
}

/// The internal state of a [`DidCommAgent`].
#[derive(Debug)]
pub struct DidCommAgentState {
  pub(crate) handlers: DidCommHandlerMap,
  pub(crate) threads_receiver: DashMap<ThreadId, oneshot::Receiver<ThreadRequest>>,
  pub(crate) threads_sender: DashMap<ThreadId, oneshot::Sender<ThreadRequest>>,
  // TODO: See above.
  #[allow(dead_code)]
  pub(crate) identity: DidCommAgentIdentity,
}

impl DidCommAgentState {
  pub(crate) fn new(handlers: DidCommHandlerMap, identity: DidCommAgentIdentity) -> Self {
    Self {
      handlers,
      threads_receiver: DashMap::new(),
      threads_sender: DashMap::new(),
      identity,
    }
  }
}

/// A [`DidCommAgent`] is an extension of an [`Agent`] with support for sending and awaiting [`DidCommRequest`]s.
///
/// An agent can be used to send requests to other, remote agents, and fowards incoming requests
/// to attached handlers. It is a frontend for an event loop running in the background, which invokes
/// user-attached handlers. Agents can be cloned without cloning the event loop, and doing so
/// is a cheap operation.
///
/// Handlers are attached at agent build time, using the [`DidCommAgentBuilder`](crate::didcomm::DidCommAgentBuilder).
///
/// While an [`Agent`] only supports attachements of synchronous [`Handler`](crate::agent::Handler)s,
/// a [`DidCommAgent`] additionally supports asynchronous [`DidCommHandler`](crate::didcomm::DidCommHandler)s.
///
/// After shutting down the event loop of an agent using [`DidCommAgent::shutdown`], other clones of the
/// agent will receive [`Error::Shutdown`] when attempting to interact with the event loop.
#[derive(Debug, Clone)]
pub struct DidCommAgent {
  pub(crate) agent: Agent,
  pub(crate) state: Arc<DidCommAgentState>,
}

impl DidCommAgent {
  pub(crate) fn commander_mut(&mut self) -> &mut NetCommander {
    self.agent.commander_mut()
  }

  /// Let this agent handle the given `request`, by invoking the appropriate handler, if attached.
  /// This consumes the agent because it passes itself to the handler.
  /// The agent will thus typically be cloned before calling this method.
  pub(crate) fn handle_request(self, request: InboundRequest) {
    match request.request_mode {
      RequestMode::Asynchronous => self.handle_async_request(request),
      RequestMode::Synchronous => self.agent.handle_sync_request(request),
    }
  }

  /// See [`Agent::start_listening`].
  pub async fn start_listening(&mut self, address: Multiaddr) -> AgentResult<Multiaddr> {
    self.agent.start_listening(address).await
  }

  /// See [`Agent::agent_id`].
  pub fn agent_id(&self) -> AgentId {
    self.agent.agent_id()
  }

  /// See [`Agent::addresses`].
  pub async fn addresses(&mut self) -> AgentResult<Vec<Multiaddr>> {
    self.agent.addresses().await
  }

  /// See [`Agent::add_agent_address`].
  pub async fn add_agent_address(&mut self, agent_id: AgentId, address: Multiaddr) -> AgentResult<()> {
    self.agent.add_agent_address(agent_id, address).await
  }

  /// See [`Agent::add_agent_addresses`].
  pub async fn add_agent_addresses(&mut self, agent_id: AgentId, addresses: Vec<Multiaddr>) -> AgentResult<()> {
    self.agent.add_agent_addresses(agent_id, addresses).await
  }

  /// See [`Agent::shutdown`].
  pub async fn shutdown(self) -> AgentResult<()> {
    self.agent.shutdown().await
  }

  /// See [`Agent::send_request`].
  pub async fn send_request<REQ: HandlerRequest>(
    &mut self,
    agent_id: AgentId,
    request: REQ,
  ) -> AgentResult<REQ::Response> {
    self.agent.send_request(agent_id, request).await
  }

  /// Sends an asynchronous DIDComm request to an agent.
  ///
  /// To receive a possible response, call [`DidCommAgent::await_didcomm_request`] with the same `thread_id`.
  pub async fn send_didcomm_request<REQ: DidCommRequest>(
    &mut self,
    agent_id: AgentId,
    thread_id: &ThreadId,
    message: REQ,
  ) -> AgentResult<()> {
    let endpoint: Endpoint = REQ::endpoint();
    let request_mode: RequestMode = REQ::request_mode();

    let dcpm = DidCommPlaintextMessage::new(thread_id.to_owned(), endpoint.to_string(), message);

    self.create_thread_channels(thread_id);

    let dcpm_vec = serde_json::to_vec(&dcpm).map_err(|err| Error::SerializationFailure {
      location: ErrorLocation::Local,
      context: "send message".to_owned(),
      error_message: err.to_string(),
    })?;

    log::debug!("sending DIDComm request on endpoint `{endpoint}`");

    let message: RequestMessage = RequestMessage::new(endpoint, request_mode, dcpm_vec);

    let response = self.commander_mut().send_request(agent_id, message).await?;

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
  /// [`DidCommAgent::send_didcomm_request`] was called on the same `thread_id` previously.
  /// Calling `send_didcomm_request` multiple times still only allows to await one message on the thread.
  ///
  /// This will return a timeout error if no message is received within the duration passed
  /// to [`DidCommAgentBuilder::timeout`](crate::didcomm::DidCommAgentBuilder::timeout).
  pub async fn await_didcomm_request<T: DeserializeOwned + Send + 'static>(
    &mut self,
    thread_id: &ThreadId,
  ) -> AgentResult<DidCommPlaintextMessage<T>> {
    if let Some(receiver) = self.state.threads_receiver.remove(thread_id) {
      // Receiving + Deserialization
      let inbound_request = tokio::time::timeout(self.agent.state().config.timeout, receiver.1)
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
    // there must be a preceding `send_didcomm_request` on that same thread.
    // Note that on the receiving handler, the very first message of a protocol
    // is not awaited through `await_didcomm_request`, so it does not need to follow these rules.
    self.state.threads_sender.insert(thread_id.to_owned(), sender);
    self.state.threads_receiver.insert(thread_id.to_owned(), receiver);
  }

  #[inline(always)]
  pub(crate) fn handle_async_request(mut self, request: InboundRequest) {
    let _ = tokio::spawn(async move {
      match self.state.handlers.get(&request.endpoint) {
        Some(handler) => {
          let handler: &dyn AbstractDidCommHandler = handler.as_ref();

          handler.handle(self.clone(), request).await;
        }
        None => {
          handler_not_found(&mut self, request).await;
        }
      }
    });
  }
}

/// Invoked when no handler was found that can handle the received request.
/// Attempts to find a thread waiting for the received message,
/// otherwise returns an error to the calling agent.
async fn handler_not_found(handler: &mut DidCommAgent, request: InboundRequest) {
  let result: Result<(), RemoteSendError> =
    match serde_json::from_slice::<DidCommPlaintextMessage<serde_json::Value>>(&request.input) {
      Err(error) => Err(RemoteSendError::DeserializationFailure {
        location: ErrorLocation::Remote,
        context: "DIDComm plaintext message deserialization".to_owned(),
        error_message: error.to_string(),
      }),
      Ok(plaintext_msg) => {
        let thread_id = plaintext_msg.thread_id();

        match handler.state.threads_sender.remove(thread_id) {
          Some(sender) => {
            let thread_request = ThreadRequest {
              endpoint: request.endpoint,
              input: request.input,
            };

            if sender.1.send(thread_request).is_err() {
              log::warn!("unable to send request with thread id `{thread_id}`");
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

  let send_result = crate::agent::send_response(
    handler.commander_mut(),
    result,
    request.response_channel,
    request.request_id,
  )
  .await;

  if let Err(err) = send_result {
    log::error!("could not acknowledge request due to: {err:?}");
  }
}

/// A map from an endpoint to the handler that handles its requests.
pub(crate) type DidCommHandlerMap = HashMap<Endpoint, Box<dyn AbstractDidCommHandler>>;
