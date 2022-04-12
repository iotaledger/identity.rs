// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::collections::HashMap;
use std::result::Result as StdResult;
use std::sync::Arc;

use crate::actor::errors::ErrorLocation;
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
use identity_comm::envelope::CEKAlgorithm;
use identity_comm::envelope::DidCommEncryptedMessage;
use identity_comm::envelope::EncryptionAlgorithm;
use identity_core::common::OneOrMany;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDIDUrl;
use libp2p::Multiaddr;
use libp2p::PeerId;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use super::actor_identity::ActorIdentity;

pub(crate) struct ActorState {
  pub(crate) handlers: HandlerMap,
  pub(crate) objects: ObjectMap,
  pub(crate) threads_receiver: DashMap<ThreadId, oneshot::Receiver<ThreadRequest>>,
  pub(crate) threads_sender: DashMap<ThreadId, oneshot::Sender<ThreadRequest>>,
  pub(crate) peer_id: PeerId,
  pub(crate) config: ActorConfig,
  pub(crate) did_comm_config: DIDCommConfig,
  pub(crate) identity: Option<ActorIdentity>,
}

#[derive(Debug)]
pub(crate) struct DIDCommKeyConfig {
  /// The public key of the peer to used for encryption.
  pub(crate) peer_key: PublicKey,
  /// The url identifying the key of the local identity to use for encryption.
  pub(crate) own_key: IotaDIDUrl,
}

impl DIDCommKeyConfig {
  pub fn new(own_key: IotaDIDUrl, peer_key: PublicKey) -> Self {
    Self { own_key, peer_key }
  }
}

#[derive(Debug)]
pub(crate) struct DIDCommConfig {
  // TODO: String should be KeyLocation?
  pub(crate) peer_keys: DashMap<PeerId, DIDCommKeyConfig>,
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
#[derive(Clone)]
pub struct Actor {
  #[cfg(not(feature = "primitives"))]
  pub(crate) commander: NetCommander,
  #[cfg(feature = "primitives")]
  pub commander: NetCommander,
  pub(crate) state: Arc<ActorState>,
}

impl Actor {
  pub(crate) async fn from_builder(
    commander: NetCommander,
    handlers: HandlerMap,
    objects: ObjectMap,
    peer_id: PeerId,
    config: ActorConfig,
    identity: Option<ActorIdentity>,
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
        did_comm_config: DIDCommConfig {
          peer_keys: DashMap::new(),
        },
        identity,
      }),
    };

    Ok(actor)
  }

  fn handlers(&self) -> &HandlerMap {
    &self.state.as_ref().handlers
  }

  pub(crate) fn try_identity(&self) -> Result<&ActorIdentity> {
    self.state.identity.as_ref().ok_or(crate::Error::IdentityMissing)
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
  pub(crate) fn handle_request<STR: InvocationStrategy>(mut self, mut request: InboundRequest) {
    cfg_if::cfg_if! {
      if #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))] {
        let spawn = tokio::spawn;
      } else {
        let spawn = wasm_bindgen_futures::spawn_local;
      }
    }

    let _ = spawn(async move {
      if let Some(tuple) = self.state.did_comm_config.peer_keys.get(&request.peer_id) {
        request = self.decrypt_request(tuple.value(), request).expect("TODO");
      }

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

  /// Sends an asynchronous message to a peer. To receive a potential response, use [`Actor::await_message`],
  /// with the same `thread_id`.
  pub async fn send_message<REQ: ActorRequest<Asynchronous>>(
    &mut self,
    peer: PeerId,
    thread_id: &ThreadId,
    message: REQ,
  ) -> Result<()> {
    self.send_named_message(peer, REQ::endpoint(), thread_id, message).await
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

    log::trace!("sending message {dcpm:#?}");

    let message: String = if let Some(key) = self.state.did_comm_config.peer_keys.get(&peer) {
      self.encrypt_request(key.value(), &dcpm)?
    } else {
      let plaintext = identity_comm::envelope::Plaintext::pack(&dcpm).map_err(|err| Error::SerializationFailure {
        location: ErrorLocation::Local,
        context: "send message".to_owned(),
        error_message: err.to_string(),
      })?;
      plaintext.0
    };

    let message = RequestMessage::new(name, request_mode, message.into_bytes())?;

    log::debug!("Sending `{name}` message");

    let response = self.commander.send_request(peer, message).await?;

    serde_json::from_slice::<StdResult<(), RemoteSendError>>(&response.0).map_err(|err| {
      Error::DeserializationFailure {
        location: ErrorLocation::Local,
        context: "send message".to_owned(),
        error_message: err.to_string(),
      }
    })??;

    Ok(())
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

  #[inline(always)]
  async fn call_send_message_hook<MOD: SyncMode, REQ: ActorRequest<MOD>>(
    &self,
    peer: PeerId,
    input: REQ,
  ) -> Result<REQ> {
    let mut endpoint = Endpoint::new(REQ::endpoint())?;
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
          location: ErrorLocation::Local,
          context: "await message".to_owned(),
          error_message: err.to_string(),
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

  fn encrypt_request<T: serde::Serialize>(
    &self,
    key_config: &DIDCommKeyConfig,
    dcpm: &DidCommPlaintextMessage<T>,
  ) -> Result<String> {
    let keypair: &KeyPair = self.try_identity()?.keypairs.get(&key_config.own_key).expect("TODO");

    let recipients: &[PublicKey] = &[key_config.peer_key.to_owned()];

    let dcem: DidCommEncryptedMessage = DidCommEncryptedMessage::pack(
      dcpm,
      CEKAlgorithm::ECDH_ES_A256KW,
      EncryptionAlgorithm::A256GCM,
      recipients,
      keypair,
    )
    .map_err(|err| crate::Error::CryptError {
      operation: super::errors::CryptOperation::Encryption,
      location: ErrorLocation::Local,
      context: "request encryption".to_owned(),
      error_message: err.to_string(),
    })?;

    Ok(dcem.0)
  }

  fn decrypt_request(
    &self,
    key_config: &DIDCommKeyConfig,
    mut request: InboundRequest,
  ) -> StdResult<InboundRequest, RemoteSendError> {
    let own_key = self
      .try_identity()
      .map_err(|_| RemoteSendError::IdentityMissing)?
      .keypairs
      .get(&key_config.own_key)
      .expect("TODO");

    let message: String = String::from_utf8(request.input).map_err(|err| RemoteSendError::CryptError {
      operation: super::errors::CryptOperation::Decryption,
      location: ErrorLocation::Remote,
      context: "decoding request bytes into utf8 string".to_owned(),
      error_message: err.to_string(),
    })?;

    let encrypted = DidCommEncryptedMessage(message);
    let unpacked: Vec<u8> = encrypted
      .unpack_vec(
        CEKAlgorithm::ECDH_ES_A256KW,
        EncryptionAlgorithm::A256GCM,
        own_key.private(),
        &key_config.peer_key,
      )
      .map_err(|_err| RemoteSendError::CryptError {
        operation: super::errors::CryptOperation::Decryption,
        location: ErrorLocation::Remote,
        context: "request decryption".to_owned(),
        // TODO: (?) omitted (for now) in an abundance of caution not to leak any information
        error_message: "".to_owned(),
      })?;

    request.input = unpacked;
    Ok(request)
  }

  /// Call the hook identified by the given `endpoint` with some `input`.
  async fn call_hook<I, O>(&self, endpoint: Endpoint, peer: PeerId, input: I) -> StdResult<O, RemoteSendError>
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
pub struct HandlerObject {
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
