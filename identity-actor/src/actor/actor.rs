// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::marker::PhantomData;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::sync::Arc;

use crate::p2p::behaviour::DidCommResponse;
use crate::p2p::event_loop::InboundRequest;
use crate::p2p::event_loop::ThreadRequest;
use crate::p2p::net_commander::NetCommander;
use crate::ActorRequest;
use crate::AsyncFn;
use crate::DidCommPlaintextMessage;
use crate::DidCommTermination;
use crate::Endpoint;
use crate::RemoteSendError;
use crate::RequestContext;
use crate::RequestHandler;
use crate::RequestMessage;
use crate::Result;
use crate::ThreadId;

use dashmap::DashMap;
use futures::channel::oneshot;
use futures::Future;
use libp2p::Multiaddr;
use libp2p::PeerId;

use libp2p::request_response::ResponseChannel;
use libp2p::TransportError;
use serde::de::DeserializeOwned;
use uuid::Uuid;

/// A map from an identifier to an object that contains the
/// shared state of the associated handler functions.
type ObjectMap = DashMap<Uuid, Box<dyn Any + Send + Sync>>;
/// A map from a request name to the identifier of the shared state object
/// and the method that handles that particular request.
type HandlerMap = DashMap<Endpoint, (Uuid, Box<dyn RequestHandler>)>;

type HandlerObjectTuple<'a> = (
  dashmap::mapref::one::Ref<'a, Endpoint, (Uuid, Box<dyn RequestHandler>)>,
  Box<dyn Any + Send + Sync>,
);

// TODO: Can this take OBJ as a parameter for more type safety across add_state + add_handler?
pub struct HandlerBuilder<OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
{
  pub(crate) object_id: Uuid,
  pub(crate) actor_state: Arc<ActorState>,
  _marker_obj: PhantomData<&'static OBJ>,
}

impl<OBJ> HandlerBuilder<OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
{
  pub fn add_handler<REQ, FUT, FUN>(self, cmd: &'static str, handler: FUN) -> Result<Self>
  where
    REQ: ActorRequest + Send + Sync + 'static,
    FUT: Future<Output = REQ::Response> + Send + 'static,
    FUN: 'static + Send + Sync + Fn(OBJ, Actor, RequestContext<REQ>) -> FUT,
  {
    let handler = AsyncFn::new(handler);
    self
      .actor_state
      .handlers
      .insert(Endpoint::new(cmd)?, (self.object_id, Box::new(handler)));
    Ok(self)
  }
}

pub struct ActorState {
  pub(crate) handlers: HandlerMap,
  pub(crate) objects: ObjectMap,
  pub(crate) threads_receiver: DashMap<ThreadId, oneshot::Receiver<ThreadRequest>>,
  pub(crate) threads_sender: DashMap<ThreadId, oneshot::Sender<ThreadRequest>>,
  pub(crate) peer_id: PeerId,
}

#[derive(Clone)]
pub struct Actor {
  commander: NetCommander,
  state: Arc<ActorState>,
}

impl Actor {
  pub(crate) async fn from_builder(
    commander: NetCommander,
    handlers: HandlerMap,
    objects: ObjectMap,
    peer_id: PeerId,
  ) -> Result<Self> {
    let actor = Self {
      commander,
      state: Arc::new(ActorState {
        handlers,
        objects,
        threads_receiver: DashMap::new(),
        threads_sender: DashMap::new(),
        peer_id,
      }),
    };

    Ok(actor)
  }

  pub fn add_state<OBJ>(&mut self, handler: OBJ) -> HandlerBuilder<OBJ>
  where
    OBJ: Clone + Send + Sync + 'static,
  {
    let object_id = Uuid::new_v4();
    self.state.objects.insert(object_id, Box::new(handler));
    HandlerBuilder {
      object_id,
      actor_state: Arc::clone(&self.state),
      _marker_obj: PhantomData,
    }
  }

  fn handlers(&self) -> &HandlerMap {
    &self.state.as_ref().handlers
  }

  pub async fn start_listening(&mut self, address: Multiaddr) -> StdResult<(), TransportError<std::io::Error>> {
    self.commander.start_listening(address).await
  }

  pub fn peer_id(&mut self) -> PeerId {
    self.state.peer_id
  }

  pub async fn addresses(&mut self) -> Vec<Multiaddr> {
    self.commander.get_addresses().await
  }

  #[inline(always)]
  pub(crate) fn handle_request(mut self, request: InboundRequest) {
    let _ = tokio::spawn(async move {
      if self.state.handlers.contains_key(&request.endpoint) {
        self.invoke_handler(request).await;
      } else {
        let result: StdResult<(), RemoteSendError> =
          match serde_json::from_slice::<DidCommPlaintextMessage<serde_json::Value>>(&request.input) {
            Err(error) => Err(RemoteSendError::DeserializationFailure(error.to_string())),
            Ok(plaintext_msg) => {
              let thread_id = plaintext_msg.thread_id();

              match self.state.threads_sender.remove(thread_id) {
                Some(sender) => {
                  let thread_request = ThreadRequest {
                    peer_id: request.peer_id,
                    endpoint: request.endpoint,
                    input: request.input,
                  };

                  sender.1.send(thread_request).expect("TODO");

                  Ok(())
                }
                None => {
                  log::info!(
                    "no handler or thread found for the received message `{}`",
                    request.endpoint
                  );
                  // TODO: Should this be a more generic error name, including unknown endpoint + unknown thread?
                  Err(RemoteSendError::UnknownRequest(request.endpoint.to_string()))
                }
              }
            }
          };

        Self::send_response(&mut self.commander, result, request.response_channel).await;
      }
    });
  }

  #[inline(always)]
  async fn invoke_handler(self, inbound_request: InboundRequest) {
    let input: Vec<u8> = inbound_request.input;
    let endpoint: Endpoint = inbound_request.endpoint;
    let peer_id: PeerId = inbound_request.peer_id;
    let response_channel: ResponseChannel<_> = inbound_request.response_channel;

    log::debug!("request for endpoint {endpoint}");

    let handler_object_tuple: StdResult<_, RemoteSendError> = self.get_handler(&endpoint);

    // TODO: Would be nice if we didn't have to clone again. Passing a &mut ref would be nice, but
    // introduces lifetimes in lots of places.
    let actor = self.clone();

    let mut commander = self.commander.clone();

    match handler_object_tuple {
      Ok((handler, object)) => {
        // Send actor-level acknowledgment that the message was received and a handler exists.
        Self::send_response(&mut commander, Ok(()), response_channel).await;

        let request_context: RequestContext<()> = RequestContext::new((), peer_id, endpoint);

        let input = handler.value().1.deserialize_request(input).unwrap();
        match handler.value().1.invoke(actor, request_context, object, input) {
          Ok(invocation) => {
            invocation.await;
          }
          Err(err) => {
            log::error!("{}", err);
          }
        }
      }
      Err(error) => {
        log::debug!("handler error: {error:?}");

        let err_response: StdResult<(), RemoteSendError> = Err(error);
        Self::send_response(&mut commander, err_response, response_channel).await;

        // TODO: If the response could not be sent, log the error.
        // log::error!("could not respond to `{}` request", endpoint);
      }
    }
  }

  fn get_handler(&self, endpoint: &Endpoint) -> std::result::Result<HandlerObjectTuple<'_>, RemoteSendError> {
    match self.state.handlers.get(endpoint) {
      Some(handler_tuple) => {
        let object_id = handler_tuple.0;

        if let Some(object) = self.state.objects.get(&object_id) {
          let object_clone = handler_tuple.1.clone_object(object.deref());
          Ok((handler_tuple, object_clone))
        } else {
          Err(RemoteSendError::HandlerInvocationError(format!(
            "no state set for {}",
            endpoint
          )))
        }
      }
      None => Err(RemoteSendError::UnknownRequest(endpoint.to_string())),
    }
  }

  async fn send_response(
    commander: &mut NetCommander,
    response: StdResult<(), RemoteSendError>,
    channel: ResponseChannel<DidCommResponse>,
  ) {
    log::debug!("responding with {:?}", response);
    let response: Vec<u8> = serde_json::to_vec(&response).unwrap();
    // TODO: This could produce an InboundFailure the function currently does not return. Should we change that?
    commander.send_response(response, channel).await;
  }

  // fn send_ack(response_tx: Sender<Vec<u8>>) {
  //   // TODO: can return an error when
  //   // - connection times out, when
  //   // - when handler takes too long to respond (configurable via SwarmBuilder.with_timeout)
  //   // - error on the transport layer
  //   // - potentially others...
  //   let ack: StdResult<(), RemoteSendError> = Ok(());
  //   let response = serde_json::to_vec(&ack).unwrap();
  //   let response_result = response_tx.send(response);

  //   if response_result.is_err() {
  //     log::error!("could not respond to request");
  //   }
  // }

  pub async fn stop_handling_requests(mut self) -> Result<()> {
    self.commander.stop_listening().await;
    // TODO: Send and impl shutdown.

    Ok(())
  }

  pub async fn add_address(&mut self, peer: PeerId, addr: Multiaddr) {
    self.commander.add_address(peer, addr).await;
  }

  pub async fn send_request<Request: ActorRequest>(
    &mut self,
    _peer: PeerId,
    _command: Request,
  ) -> Result<Request::Response> {
    todo!()
    // self.send_named_request(peer, &*command.request_name(), command).await
  }

  pub async fn send_named_request<Request: ActorRequest>(
    &mut self,
    _peer: PeerId,
    _name: &str,
    _command: Request,
  ) -> Result<Request::Response> {
    todo!()
    // let request = serde_json::to_vec(&RequestMessage::new(name, command)?).unwrap();

    // // log::debug!("Sending `{}` request", request.endpoint);

    // let response = self.commander.send_request(peer, request).await?;

    // let request_response: serde_json::Result<StdResult<Request::Response, RemoteSendError>> =
    //   serde_json::from_slice(&response);

    // match request_response {
    //   Ok(Ok(res)) => Ok(res),
    //   Ok(Err(err)) => Err(err.into()),
    //   Err(err) => Err(crate::Error::DeserializationFailure(err.to_string())),
    // }
  }

  pub async fn send_message<Request: ActorRequest>(
    &mut self,
    peer: PeerId,
    thread_id: &ThreadId,
    command: Request,
  ) -> Result<()> {
    self
      .send_named_message(peer, &command.request_name(), thread_id, command)
      .await
  }

  pub async fn send_named_message<Request: ActorRequest>(
    &mut self,
    peer: PeerId,
    name: &str,
    thread_id: &ThreadId,
    message: Request,
  ) -> Result<()> {
    self.create_thread_channels(thread_id);

    // let message: serde_json::Value = serde_json::to_value(&message).expect("TODO");
    let dcpm = DidCommPlaintextMessage::new(thread_id.to_owned(), name.to_owned(), message);

    let dcpm = self.call_send_message_hook(peer, dcpm).await?;

    let dcpm_vec = serde_json::to_vec(&dcpm).expect("TODO");
    let message = serde_json::to_vec(&RequestMessage::new(name, dcpm_vec)?).unwrap();

    log::debug!("Sending `{}` message", name);

    let response = self.commander.send_request(peer, message).await?;

    log::debug!(
      "ack was: {:#?}",
      serde_json::from_slice::<serde_json::Value>(&response.0).expect("TODO")
    );

    serde_json::from_slice::<StdResult<(), RemoteSendError>>(&response.0).expect("TODO")?;

    Ok(())
  }

  #[inline(always)]
  async fn call_send_message_hook<REQ: ActorRequest>(&self, peer: PeerId, input: REQ) -> Result<REQ> {
    let endpoint = Endpoint::new_hook(input.request_name())?;

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

  // TODO: This should take a T: DeserializeOwned to deserialize into and
  // return a DidCommPlaintextMessage<T> (which requires changing that type)
  pub async fn await_message<T: DeserializeOwned + Send + 'static>(
    &mut self,
    thread_id: &ThreadId,
  ) -> Result<DidCommPlaintextMessage<T>> {
    if let Some(receiver) = self.state.threads_receiver.remove(thread_id) {
      // Receival + Deserialization
      let inbound_request = receiver.1.await.expect("TODO: (?) channel closed");

      let message: DidCommPlaintextMessage<T> = serde_json::from_slice(inbound_request.input.as_ref())
        .map_err(|err| crate::Error::DeserializationFailure(err.to_string()))?;

      log::debug!("awaited message {}", inbound_request.endpoint);

      // Hooking
      let mut hook_endpoint: Endpoint = inbound_request.endpoint;
      hook_endpoint.set_is_hook(true);

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
      Err(crate::Error::ThreadNotFound(thread_id.to_owned()))
    }
  }

  // Creates the channels used to await a message on a thread.
  fn create_thread_channels(&mut self, thread_id: &ThreadId) {
    let (sender, receiver) = oneshot::channel();

    // The logic is that for every received message on a thread,
    // there must be a preceding send_message on that same thread.
    // Note that on the receiving actor, the very first message of a protocol
    // is not awaited through await_message, so it does not need to follow that logic.
    self.state.threads_sender.insert(thread_id.to_owned(), sender);
    self.state.threads_receiver.insert(thread_id.to_owned(), receiver);
  }

  /// Call the hook identified by the given `endpoint`.
  pub async fn call_hook<I, O>(
    &self,
    endpoint: Endpoint,
    peer: PeerId,
    input: I,
  ) -> std::result::Result<O, RemoteSendError>
  where
    I: Send + 'static,
    O: 'static,
  {
    match self.get_handler(&endpoint) {
      Ok(handler_object) => {
        let handler = &handler_object.0.value().1;
        let state = handler_object.1;
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
