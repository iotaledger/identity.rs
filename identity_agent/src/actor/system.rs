// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;

use identity_core::common::OneOrMany;
use libp2p::request_response::InboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;
use libp2p::Multiaddr;
use libp2p::PeerId;

use crate::actor::errors::ErrorLocation;
use crate::actor::AbstractActor;
use crate::actor::ActorRequest;
use crate::actor::Endpoint;
use crate::actor::Error;
use crate::actor::RemoteSendError;
use crate::actor::RequestContext;
use crate::actor::RequestMode;
use crate::actor::Result as ActorResult;
use crate::actor::SystemState;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::RequestMessage;
use crate::p2p::ResponseMessage;

/// An actor system can be used to send requests to remote actors, and fowards incoming requests
/// to attached actors.
///
/// An actor system is a frontend for an event loop running in the background, which invokes
/// user-attached actors. Systems can be cloned without cloning the event loop, and doing so
/// is a cheap operation.
/// Actors are attached at system build time, using the [`SystemBuilder`](crate::actor::SystemBuilder).
///
/// After shutting down the event loop of a system using [`System::shutdown`], other clones of the
/// system will receive [`Error::Shutdown`] when attempting to interact with the event loop.
#[derive(Debug)]
pub struct System {
  commander: NetCommander,
  state: Arc<SystemState>,
}

impl Clone for System {
  /// Produce a shallow copy of the system, which uses the same event loop as the
  /// system that it was cloned from.
  fn clone(&self) -> Self {
    Self {
      commander: self.commander.clone(),
      state: self.state.clone(),
    }
  }
}

impl System {
  pub(crate) fn new(commander: NetCommander, state: Arc<SystemState>) -> System {
    Self { commander, state }
  }

  pub(crate) fn state(&self) -> &SystemState {
    self.state.as_ref()
  }

  /// Returns the [`PeerId`] that other peers can securely identify this system with.
  pub fn peer_id(&self) -> PeerId {
    self.state().peer_id
  }

  pub(crate) fn commander_mut(&mut self) -> &mut NetCommander {
    &mut self.commander
  }

  /// Start listening on the given `address`. Returns the first address that the system started listening on, which may
  /// be different from `address` itself, e.g. when passing addresses like `/ip4/0.0.0.0/tcp/0`. Even when passing a
  /// single address, multiple addresses may end up being listened on. To obtain all those addresses, use
  /// [`System::addresses`]. Note that even when the same address is passed, the returned address is not deterministic,
  /// and should thus not be relied upon.
  pub async fn start_listening(&mut self, address: Multiaddr) -> ActorResult<Multiaddr> {
    self.commander_mut().start_listening(address).await
  }

  /// Return all addresses that are currently being listened on.
  pub async fn addresses(&mut self) -> ActorResult<Vec<Multiaddr>> {
    self.commander_mut().get_addresses().await
  }

  /// Shut this system down. This will break the event loop in the background immediately,
  /// returning an error for all current actors that interact with their copy of the
  /// system or those waiting on messages. The system will thus stop listening on all addresses.
  ///
  /// Calling this and other methods, which interact with the event loop, on a system that was shutdown
  /// will return [`Error::Shutdown`].
  pub async fn shutdown(mut self) -> ActorResult<()> {
    // Consuming self drops the internal commander. If this is the last copy of the commander,
    // the event loop will break as a result. However, if copies exist, such as in running handlers,
    // this function will return while the event loop keeps running. Ideally we could then join on the background task
    // to wait for all handlers to finish gracefully. However, not all spawn functions return a JoinHandle,
    // such as wasm_bindgen_futures::spawn_local. The current alternative is to use a non-graceful exit,
    // which breaks the event loop immediately and returns an error through all open channels that require a result.
    self.commander_mut().shutdown().await
  }

  /// Associate the given `peer_id` with an `address`. This `address`, or another one that was added,
  /// will be use to send requests to this [`PeerId`].
  pub async fn add_peer_address(&mut self, peer_id: PeerId, address: Multiaddr) -> ActorResult<()> {
    self
      .commander_mut()
      .add_addresses(peer_id, OneOrMany::One(address))
      .await
  }

  /// Associate the given `peer_id` with multiple `addresses`. One of the `addresses`, or another one that was added,
  /// will be use to send requests to this [`PeerId`].
  pub async fn add_peer_addresses(&mut self, peer_id: PeerId, addresses: Vec<Multiaddr>) -> ActorResult<()> {
    self
      .commander_mut()
      .add_addresses(peer_id, OneOrMany::Many(addresses))
      .await
  }

  /// Sends a synchronous request to a peer and returns its response.
  ///
  /// An address needs to be available for the given `peer`, which can be added
  /// with [`System::add_peer_address`] or [`System::add_peer_addresses`].
  pub async fn send_request<REQ: ActorRequest>(&mut self, peer: PeerId, request: REQ) -> ActorResult<REQ::Response> {
    let endpoint: Endpoint = REQ::endpoint();
    let request_mode: RequestMode = REQ::request_mode();

    let request_vec = serde_json::to_vec(&request).map_err(|err| Error::SerializationFailure {
      location: ErrorLocation::Local,
      context: "send request".to_owned(),
      error_message: err.to_string(),
    })?;

    log::debug!("Sending `{}` message", endpoint);

    let request: RequestMessage = RequestMessage::new(endpoint, request_mode, request_vec);

    let response: ResponseMessage = self.commander_mut().send_request(peer, request).await?;

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

  /// Let this system handle the given `request`, by invoking a handler function.
  /// This consumes the system because it passes itself to the handler.
  /// The system will thus typically be cloned before calling this method.
  pub(crate) fn handle_request(mut self, request: InboundRequest) {
    if request.request_mode == RequestMode::Synchronous {
      self.handle_sync_request(request)
    } else {
      let _ = tokio::spawn(async move {
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
    let _ = tokio::spawn(async move {
      match self.state.actors.get(&request.endpoint) {
        Some(actor) => {
          let context: RequestContext<Vec<u8>> =
            RequestContext::new(request.input, request.peer_id, request.endpoint.clone());
          let result: Result<Vec<u8>, RemoteSendError> = actor.handle(context).await;

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

/// A map from an endpoint to the actor that handles its requests.
pub(crate) type ActorMap = HashMap<Endpoint, Box<dyn AbstractActor>>;

pub(crate) async fn send_response<T: serde::Serialize>(
  commander: &mut NetCommander,
  response: Result<T, RemoteSendError>,
  channel: ResponseChannel<ResponseMessage>,
  request_id: RequestId,
) -> ActorResult<Result<(), InboundFailure>> {
  let response: Vec<u8> = serde_json::to_vec(&response).map_err(|err| crate::actor::Error::SerializationFailure {
    location: ErrorLocation::Local,
    context: "send response".to_owned(),
    error_message: err.to_string(),
  })?;
  commander.send_response(response, channel, request_id).await
}

#[inline(always)]
async fn endpoint_not_found(actor: &mut System, request: InboundRequest) {
  let response: Result<Vec<u8>, RemoteSendError> =
    Err(RemoteSendError::UnexpectedRequest(request.endpoint.to_string()));

  let send_result = send_response(
    actor.commander_mut(),
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
