// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::collections::HashMap;
use std::result::Result as StdResult;
use std::sync::Arc;

use crate::actor::errors::ErrorLocation;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::p2p::RequestMessage;
use crate::ActorConfig;
use crate::ActorRequest;
use crate::Endpoint;
use crate::Error;
use crate::RemoteSendError;
use crate::RequestContext;
use crate::RequestHandler;
use crate::RequestMode;
use crate::Result;
use crate::Synchronous;
use crate::SynchronousInvocationStrategy;

use identity_core::common::OneOrMany;
use libp2p::Multiaddr;
use libp2p::PeerId;
use uuid::Uuid;

use super::generic_actor::GenericActor;

pub trait ActorStateExtension: 'static + Send + Sync + Sized {}

impl ActorStateExtension for () {}

pub(crate) struct ActorState<EXT>
where
  EXT: ActorStateExtension,
{
  pub(crate) handlers: HandlerMap,
  pub(crate) objects: ObjectMap,
  pub(crate) peer_id: PeerId,
  pub(crate) config: ActorConfig,
  pub(crate) extension: EXT,
}

/// The [`Actor`] can be used to send and receive messages to and from other actors.
///
/// An actor is a frontend for an event loop running in the background, which invokes
/// user-registered handlers and injects a copy of the actor into it. Actors can thus be cloned
/// without cloning the event loop, and doing so is a relatively cheap operation.
/// Handlers are registered at actor build time, using the [`ActorBuilder`](crate::ActorBuilder).
///
/// After shutting down the event loop of an actor using [`Actor::shutdown`], other clones of the
/// actor will receive [`Error::Shutdown`] when attempting to interact with the event loop.
pub struct Actor<EXT = ()>
where
  EXT: ActorStateExtension,
{
  #[cfg(not(feature = "primitives"))]
  pub(crate) commander: NetCommander,
  #[cfg(feature = "primitives")]
  pub commander: NetCommander,
  pub(crate) state: Arc<ActorState<EXT>>,
}

impl<EXT> Clone for Actor<EXT>
where
  EXT: ActorStateExtension,
{
  fn clone(&self) -> Self {
    Self {
      commander: self.commander.clone(),
      state: Arc::clone(&self.state),
    }
  }
}

impl<EXT> Actor<EXT>
where
  EXT: ActorStateExtension,
{
  pub(crate) fn from_builder(
    handlers: HandlerMap,
    objects: ObjectMap,
    config: ActorConfig,
    peer_id: PeerId,
    commander: NetCommander,
    extension: EXT,
  ) -> Result<Self> {
    let actor = Self {
      commander,
      state: Arc::new(ActorState {
        handlers,
        objects,
        peer_id,
        config,
        extension,
      }),
    };

    Ok(actor)
  }

  pub(crate) fn handlers(&self) -> &HandlerMap {
    &self.state.as_ref().handlers
  }

  /// Start listening on the given `address`. Returns the first address that the actor started listening on, which may
  /// be different from `address` itself, e.g. when passing addresses like `/ip4/0.0.0.0/tcp/0`. Even when passing a
  /// single address, multiple addresses may end up being listened on. To obtain all those addresses, use
  /// [`Actor::addresses`]. Note that even when the same address is passed, the returned address is not deterministic,
  /// and should thus not be relied upon.
  pub async fn start_listening(&mut self, address: Multiaddr) -> crate::Result<Multiaddr> {
    self.commander.start_listening(address).await
  }

  /// Returns the [`PeerId`] that other peers can securely identify this actor with.
  pub fn peer_id(&self) -> PeerId {
    self.state.peer_id
  }

  /// Return all addresses that are currently being listened on.
  pub async fn addresses(&mut self) -> crate::Result<Vec<Multiaddr>> {
    self.commander.get_addresses().await
  }

  #[inline(always)]
  pub(crate) fn handle_sync_request(mut self, request: InboundRequest) {
    cfg_if::cfg_if! {
      if #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))] {
        let spawn = tokio::spawn;
      } else {
        let spawn = wasm_bindgen_futures::spawn_local;
      }
    }

    let _ = spawn(async move {
      if self.state.handlers.contains_key(&request.endpoint) {
        let mut actor = self.clone();

        match self.get_handler(&request.endpoint).and_then(|handler_ref| {
          let input = handler_ref.0.handler.deserialize_request(request.input)?;
          Ok((handler_ref.0, handler_ref.1, input))
        }) {
          Ok((handler_ref, object, input)) => {
            let handler: &dyn RequestHandler = handler_ref.handler.as_ref();

            let request_context: RequestContext<()> = RequestContext::new((), request.peer_id, request.endpoint);

            SynchronousInvocationStrategy::invoke_handler(
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

            let result = SynchronousInvocationStrategy::handler_deserialization_failure(
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
        SynchronousInvocationStrategy::endpoint_not_found(&mut self, request).await;
      }
    });
  }

  pub(crate) fn get_handler(&self, endpoint: &Endpoint) -> StdResult<HandlerObjectTuple<'_>, RemoteSendError> {
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

  /// Shut this actor down. This will break the event loop in the background immediately,
  /// returning an error for all current handlers that interact with their copy of the
  /// actor or those waiting on messages. The actor will thus stop listening on all addresses.
  ///
  /// Calling this and other methods, which interact with the event loop, on an actor that was shutdown
  /// will return [`Error::Shutdown`].
  pub async fn shutdown(mut self) -> Result<()> {
    // Consuming self drops the internal commander. If this is the last copy of the commander,
    // the event loop will break as a result. However, if copies exist, such as in running handlers,
    // this function will return while the event loop keeps running. Ideally we could then join on the background task
    // to wait for all handlers to finish gracefully. However, not all spawn functions return a JoinHandle,
    // such as wasm_bindgen_futures::spawn_local. The current alternative is to use a non-graceful exit,
    // which breaks the event loop immediately and returns an error through all open channels that require a result.
    self.commander.shutdown().await
  }

  /// Associate the given `peer_id` with an `address`. This needs to be done before sending a
  /// request to this [`PeerId`].
  pub async fn add_address(&mut self, peer_id: PeerId, address: Multiaddr) -> crate::Result<()> {
    self.commander.add_addresses(peer_id, OneOrMany::One(address)).await
  }

  /// Associate the given `peer_id` with multiple `addresses`. This needs to be done before sending a
  /// request to this [`PeerId`].
  pub async fn add_addresses(&mut self, peer_id: PeerId, addresses: Vec<Multiaddr>) -> crate::Result<()> {
    self.commander.add_addresses(peer_id, OneOrMany::Many(addresses)).await
  }

  /// Sends a synchronous request to a peer and returns its response.
  pub async fn send_request<REQ: ActorRequest<Synchronous>>(
    &mut self,
    peer: PeerId,
    request: REQ,
  ) -> Result<REQ::Response> {
    self.send_named_request(peer, REQ::endpoint(), request).await
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
      location: ErrorLocation::Local,
      context: "send request".to_owned(),
      error_message: err.to_string(),
    })?;

    let message = RequestMessage::new(name, request_mode, request_vec)?;

    log::debug!("Sending `{}` message", name);

    let response = self.commander.send_request(peer, message).await?;

    let response: Vec<u8> =
      serde_json::from_slice::<StdResult<Vec<u8>, RemoteSendError>>(&response.0).map_err(|err| {
        Error::DeserializationFailure {
          location: ErrorLocation::Local,
          context: "send request".to_owned(),
          error_message: err.to_string(),
        }
      })??;

    serde_json::from_slice::<REQ::Response>(&response).map_err(|err| Error::DeserializationFailure {
      location: ErrorLocation::Local,
      context: "send request".to_owned(),
      error_message: err.to_string(),
    })
  }
}

impl GenericActor for Actor {
  type Extension = ();

  fn from_actor_builder(
    handlers: HandlerMap,
    objects: ObjectMap,
    config: ActorConfig,
    peer_id: PeerId,
    commander: NetCommander,
    extension: Self::Extension,
  ) -> crate::Result<Self> {
    Self::from_builder(handlers, objects, config, peer_id, commander, extension)
  }

  fn handle_request(self, request: InboundRequest) {
    if request.request_mode == RequestMode::Asynchronous {
      todo!("return `NotSupported` error or similar");
    }

    self.handle_sync_request(request)
  }
}

/// A map from an identifier to an object that contains the
/// shared state of the associated handler functions.
pub(crate) type ObjectMap = HashMap<ObjectId, Box<dyn Any + Send + Sync>>;

/// An actor-internal identifier for the object representing the shared state of one or more handlers.
pub(crate) type ObjectId = Uuid;

/// A [`RequestHandler`] and the id of its associated shared state object.
pub struct HandlerObject {
  pub(crate) handler: Box<dyn RequestHandler>,
  pub(crate) object_id: ObjectId,
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
