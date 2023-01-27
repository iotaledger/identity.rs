// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;

use identity_core::common::OneOrMany;
use libp2p::request_response::InboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;
use libp2p::Multiaddr;

use crate::agent::errors::ErrorLocation;
use crate::agent::AbstractHandler;
use crate::agent::AgentState;
use crate::agent::Endpoint;
use crate::agent::Error;
use crate::agent::HandlerRequest;
use crate::agent::RemoteSendError;
use crate::agent::RequestContext;
use crate::agent::RequestMode;
use crate::agent::Result as AgentResult;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::RequestMessage;
use crate::p2p::ResponseMessage;

/// A map from an endpoint to the handler that handles its requests.
pub(crate) type HandlerMap = HashMap<Endpoint, Box<dyn AbstractHandler>>;

/// The cryptographic identifier of an agent on the network.
pub type AgentId = libp2p::PeerId;

/// An agent can be used to send requests to other, remote agents, and fowards incoming requests
/// to attached handlers.
///
/// An agent is a frontend for an event loop running in the background, which invokes
/// user-attached handlers. Agents can be cloned without cloning the event loop, and doing so
/// is a cheap operation.
/// Handlers are attached at agent build time, using the [`AgentBuilder`](crate::agent::AgentBuilder).
///
/// After shutting down the event loop of an agent using [`Agent::shutdown`], other clones of the
/// agent will receive [`Error::Shutdown`] when attempting to interact with the event loop.
#[derive(Debug)]
pub struct Agent {
  commander: NetCommander,
  state: Arc<AgentState>,
}

// Implement Clone for the sake of documenting that it is a cheap operation in this case.
impl Clone for Agent {
  /// Produce a shallow copy of the agent, which uses the same event loop as the
  /// agent that it was cloned from.
  fn clone(&self) -> Self {
    Self {
      commander: self.commander.clone(),
      state: self.state.clone(),
    }
  }
}

impl Agent {
  pub(crate) fn new(commander: NetCommander, state: Arc<AgentState>) -> Agent {
    Self { commander, state }
  }

  pub(crate) fn state(&self) -> &AgentState {
    self.state.as_ref()
  }

  /// Returns the [`AgentId`] that other peers can securely identify this agent with.
  pub fn agent_id(&self) -> AgentId {
    self.state().agent_id
  }

  pub(crate) fn commander_mut(&mut self) -> &mut NetCommander {
    &mut self.commander
  }

  /// Start listening on the given `address`. Returns the first address that the agent started listening on, which may
  /// be different from `address` itself, for example when passing addresses like `/ip4/0.0.0.0/tcp/0`. Even when
  /// passing a single address, multiple addresses may end up being listened on. To obtain all those addresses, use
  /// [`Agent::addresses`]. Note that even when the same address is passed, the returned address is not deterministic,
  /// and should thus not be relied upon.
  pub async fn start_listening(&mut self, address: Multiaddr) -> AgentResult<Multiaddr> {
    self.commander_mut().start_listening(address).await
  }

  /// Return all addresses that are currently being listened on.
  pub async fn addresses(&mut self) -> AgentResult<Vec<Multiaddr>> {
    self.commander_mut().get_addresses().await
  }

  /// Shut this agent down. This will break the event loop in the background immediately,
  /// returning an error for all current handlers that interact with their copy of the
  /// agent or those waiting on messages. The agent will thus stop listening on all addresses.
  ///
  /// Calling this and other methods, which interact with the event loop, on an agent that was shutdown
  /// will return [`Error::Shutdown`].
  pub async fn shutdown(mut self) -> AgentResult<()> {
    // Consuming self drops the internal commander. If this is the last copy of the commander,
    // the event loop will break as a result. However, if copies exist, such as in running handlers,
    // this function will return while the event loop keeps running. Ideally we could then join on the background task
    // to wait for all handlers to finish gracefully. This was not implemented that way, because of a previous
    // dependency on wasm_bindgen_futures::spawn_local which does not return a JoinHandle. It would be an option to
    // change it, now that we're using tokio exclusively.
    // The current implementation uses a non-graceful exit, which breaks the event loop immediately
    // and returns an error through all open channels that require a result.
    self.commander_mut().shutdown().await
  }

  /// Associate the given `agent_id` with an `address`. This `address`, or another one that was added,
  /// will be use to send requests to `agent_id`.
  pub async fn add_agent_address(&mut self, agent_id: AgentId, address: Multiaddr) -> AgentResult<()> {
    self
      .commander_mut()
      .add_addresses(agent_id, OneOrMany::One(address))
      .await
  }

  /// Associate the given `agent_id` with multiple `addresses`. One of the `addresses`, or another one that was added,
  /// will be use to send requests to `agent_id`.
  pub async fn add_agent_addresses(&mut self, agent_id: AgentId, addresses: Vec<Multiaddr>) -> AgentResult<()> {
    self
      .commander_mut()
      .add_addresses(agent_id, OneOrMany::Many(addresses))
      .await
  }

  /// Sends a synchronous request to an agent, identified through `agent_id`, and returns its response.
  ///
  /// An address needs to be available for the given `agent_id`, which can be added
  /// with [`Agent::add_agent_address`] or [`Agent::add_agent_addresses`].
  pub async fn send_request<REQ: HandlerRequest>(
    &mut self,
    agent_id: AgentId,
    request: REQ,
  ) -> AgentResult<REQ::Response> {
    let endpoint: Endpoint = REQ::endpoint();
    let request_mode: RequestMode = REQ::request_mode();

    let request_vec = serde_json::to_vec(&request).map_err(|err| Error::SerializationFailure {
      location: ErrorLocation::Local,
      context: "send request".to_owned(),
      error_message: err.to_string(),
    })?;

    log::debug!("sending request on endpoint `{endpoint}`");

    let request: RequestMessage = RequestMessage::new(endpoint, request_mode, request_vec);

    let response: ResponseMessage = self.commander_mut().send_request(agent_id, request).await?;

    let response: Vec<u8> =
      serde_json::from_slice::<Result<Vec<u8>, RemoteSendError>>(&response.0).map_err(|err| {
        Error::DeserializationFailure {
          location: ErrorLocation::Local,
          context: "send request (result)".to_owned(),
          error_message: err.to_string(),
        }
      })??;

    serde_json::from_slice::<REQ::Response>(&response).map_err(|err| Error::DeserializationFailure {
      location: ErrorLocation::Local,
      context: "send request".to_owned(),
      error_message: err.to_string(),
    })
  }

  /// Let this agent handle the given `request`, by invoking the appropriate handler, if attached.
  /// This consumes the agent because it passes itself to the handler.
  /// The agent will thus typically be cloned before calling this method.
  pub(crate) fn handle_request(mut self, request: InboundRequest) {
    if request.request_mode == RequestMode::Synchronous {
      self.handle_sync_request(request)
    } else {
      tokio::spawn(async move {
        if let Err(error) = send_response(
          self.commander_mut(),
          Result::<(), RemoteSendError>::Err(RemoteSendError::UnexpectedRequest(
            "asynchronous requests are not supported".to_owned(),
          )),
          request.response_channel,
          request.request_id,
        )
        .await
        {
          log::error!(
            "unable to respond to synchronous request on endpoint `{}` due to: {error}",
            request.endpoint
          );
        }
      });
    }
  }

  #[inline(always)]
  pub(crate) fn handle_sync_request(mut self, request: InboundRequest) {
    tokio::spawn(async move {
      match self.state.handlers.get(&request.endpoint) {
        Some(handler) => {
          let context: RequestContext<Vec<u8>> =
            RequestContext::new(request.input, request.peer_id, request.endpoint.clone());
          let result: Result<Vec<u8>, RemoteSendError> = handler.handle(context).await;

          if let Err(error) = send_response(
            self.commander_mut(),
            result,
            request.response_channel,
            request.request_id,
          )
          .await
          {
            log::error!(
              "unable to respond to synchronous request on endpoint `{}` due to: {error}",
              request.endpoint
            );
          }
        }
        None => {
          endpoint_not_found(&mut self, request).await;
        }
      }
    });
  }
}

pub(crate) async fn send_response<T: serde::Serialize>(
  commander: &mut NetCommander,
  response: Result<T, RemoteSendError>,
  channel: ResponseChannel<ResponseMessage>,
  request_id: RequestId,
) -> AgentResult<Result<(), InboundFailure>> {
  let response: Vec<u8> = serde_json::to_vec(&response).map_err(|err| crate::agent::Error::SerializationFailure {
    location: ErrorLocation::Local,
    context: "send response".to_owned(),
    error_message: err.to_string(),
  })?;
  commander.send_response(response, channel, request_id).await
}

#[inline(always)]
async fn endpoint_not_found(handler: &mut Agent, request: InboundRequest) {
  let response: Result<Vec<u8>, RemoteSendError> =
    Err(RemoteSendError::UnexpectedRequest(request.endpoint.to_string()));

  let send_result = send_response(
    handler.commander_mut(),
    response,
    request.response_channel,
    request.request_id,
  )
  .await;

  if let Err(err) = send_result {
    log::error!(
      "could not return error for request on endpoint `{}` due to: {err:?}",
      request.endpoint
    );
  }
}
