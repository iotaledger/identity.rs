// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{any::Any, ops::Deref, sync::Arc};

use crate::{
  asyncfn::AsyncFn,
  endpoint::Endpoint,
  errors::{RemoteSendError, Result},
  traits::{ActorRequest, RequestHandler},
  types::{RequestContext, RequestMessage, ResponseMessage},
};

use dashmap::DashMap;
use futures::{
  channel::{
    mpsc::{self},
    oneshot::Sender,
  },
  Future, StreamExt,
};
use libp2p::{Multiaddr, PeerId};
use p2p::{ListenErr, ReceiveRequest, StrongholdP2p};
use tokio::{
  sync::Mutex,
  task::{self, JoinHandle},
};
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

pub struct HandlerBuilder {
  pub(crate) object_id: Uuid,
  pub(crate) handlers: Arc<HandlerMap>,
}

impl HandlerBuilder {
  pub fn add_handler<OBJ, REQ, FUT, FUN>(self, cmd: &'static str, handler: FUN) -> Result<Self>
  where
    OBJ: Clone + Send + Sync + 'static,
    REQ: ActorRequest + Send + Sync + 'static,
    FUT: Future<Output = REQ::Response> + Send + 'static,
    FUN: 'static + Send + Sync + Fn(OBJ, Actor, RequestContext<REQ>) -> FUT,
  {
    let handler = AsyncFn::new(handler);
    self
      .handlers
      .insert(Endpoint::new(cmd)?, (self.object_id, Box::new(handler)));
    Ok(self)
  }
}

#[derive(Clone)]
pub struct Actor {
  comm: StrongholdP2p<RequestMessage, ResponseMessage>,
  handlers: Arc<HandlerMap>,
  objects: Arc<ObjectMap>,
  listener_handle: Arc<Mutex<Option<JoinHandle<Result<()>>>>>,
}

impl Actor {
  pub(crate) async fn from_builder(
    receiver: mpsc::Receiver<ReceiveRequest<RequestMessage, ResponseMessage>>,
    comm: StrongholdP2p<RequestMessage, ResponseMessage>,
    handlers: HandlerMap,
    objects: ObjectMap,
    listening_addresses: Vec<Multiaddr>,
  ) -> Result<Self> {
    let handlers = Arc::new(handlers);
    let objects = Arc::new(objects);

    let mut actor = Self {
      comm,
      handlers: Arc::clone(&handlers),
      objects: Arc::clone(&objects),
      listener_handle: Arc::new(Mutex::new(None)),
    };

    // TODO: Always start listener, change `listener_handle` in actor accordingly.
    // if !listening_addresses.is_empty() {
    let handle = actor.clone().spawn_listener(receiver);

    actor.listener_handle.lock().await.replace(handle);
    // };

    for addr in listening_addresses {
      actor.comm.start_listening(addr).await?;
    }

    Ok(actor)
  }

  pub fn add_state<OBJ>(&mut self, handler: OBJ) -> HandlerBuilder
  where
    OBJ: Clone + Send + Sync + 'static,
  {
    let object_id = Uuid::new_v4();
    self.objects.insert(object_id, Box::new(handler));
    HandlerBuilder {
      object_id,
      handlers: Arc::clone(&self.handlers),
    }
  }

  pub fn handlers(&self) -> &HandlerMap {
    self.handlers.as_ref()
  }

  pub async fn start_listening(&mut self, address: Multiaddr) -> std::result::Result<Multiaddr, ListenErr> {
    self.comm.start_listening(address).await
  }

  pub fn peer_id(&self) -> PeerId {
    self.comm.get_peer_id()
  }

  pub async fn stop_listening(&mut self) {
    self.comm.stop_listening().await;
  }

  pub async fn addrs(&mut self) -> Vec<Multiaddr> {
    let listeners = self.comm.get_listeners().await;
    listeners.into_iter().map(|l| l.addrs).flatten().collect()
  }

  /// Start handling incoming requests. This method does not return unless [`stop_listening`] is called.
  /// This method should only be called once on any given instance.
  /// A second caller would immediately receive an [`Error::LockInUse`].
  fn spawn_listener(
    self,
    mut receiver: mpsc::Receiver<ReceiveRequest<RequestMessage, ResponseMessage>>,
  ) -> JoinHandle<Result<()>> {
    task::spawn(async move {
      loop {
        if let Some(receive_request) = receiver.next().await {
          self.clone().spawn_handler(receive_request);
        } else {
          return Ok(());
        }
      }
    })
  }

  fn spawn_handler(self, receive_request: ReceiveRequest<RequestMessage, ResponseMessage>) -> JoinHandle<Result<()>> {
    task::spawn(async move {
      let request = receive_request.request;
      let response_tx = receive_request.response_tx;
      let endpoint = request.endpoint;

      match self.get_handler(&endpoint) {
        Ok((handler, object)) => {
          Self::send_ack(response_tx);
          let request_context: RequestContext<()> = RequestContext::new((), receive_request.peer, endpoint);
          let input = handler.value().1.deserialize_request(request.data).unwrap();
          handler
            .value()
            .1
            .invoke(self.clone(), request_context, object, input)
            .await;
        }
        Err(error) => match self.get_handler(&endpoint.clone().to_catch_all()) {
          Ok((handler, object)) => {
            Self::send_ack(response_tx);
            let request_context: RequestContext<()> = RequestContext::new((), receive_request.peer, endpoint);
            let input = handler.value().1.deserialize_request(request.data).unwrap();
            handler
              .value()
              .1
              .invoke(self.clone(), request_context, object, input)
              .await;
          }
          Err(_) => {
            let response = serde_json::to_vec(&error).unwrap();
            if response_tx.send(response).is_err() {
              log::error!("could not respond to `{}` request", endpoint);
            }
          }
        },
      }

      Ok(())
    })
  }

  fn get_handler(&self, endpoint: &Endpoint) -> std::result::Result<HandlerObjectTuple<'_>, RemoteSendError> {
    match self.handlers.get(endpoint) {
      Some(handler_tuple) => {
        let object_id = handler_tuple.0;

        if let Some(object) = self.objects.get(&object_id) {
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

  fn send_ack(response_tx: Sender<Vec<u8>>) {
    // TODO: can return an error when
    // - connection times out, when
    // - when handler takes too long to respond (configurable via SwarmBuilder.with_timeout)
    // - error on the transport layer
    // - potentially others...
    let response = serde_json::to_vec(&serde_json::Value::Null).unwrap();
    let response_result = response_tx.send(response);

    if response_result.is_err() {
      log::error!("could not respond to request");
      // log::error!("could not respond to `{}` request", request_name);
    }
  }

  pub async fn stop_handling_requests(self) -> Result<()> {
    // TODO: aborting means that even requests that have been received and are being processed are cancelled
    // We should instead use some signalling mechanism that breaks the loop
    if let Some(listener_handle) = self.listener_handle.lock().await.take() {
      listener_handle.abort();
      let _ = listener_handle.await;
    }
    Ok(())
  }

  pub async fn add_peer(&mut self, peer: PeerId, addr: Multiaddr) {
    self.comm.add_address(peer, addr).await;
  }

  pub async fn send_request<Request: ActorRequest>(
    &mut self,
    peer: PeerId,
    command: Request,
  ) -> Result<Request::Response> {
    self.send_named_request(peer, &*command.request_name(), command).await
  }

  pub async fn send_named_request<Request: ActorRequest>(
    &mut self,
    peer: PeerId,
    name: &str,
    command: Request,
  ) -> Result<Request::Response> {
    let request = RequestMessage::new(name, serde_json::to_vec(&command).unwrap())?;

    log::debug!("Sending `{}` request", request.endpoint);

    let response = self.comm.send_request(peer, request).await?;

    let request_response: serde_json::Result<Request::Response> = serde_json::from_slice(&response);

    match request_response {
      Ok(res) => Ok(res),
      Err(err) => Err(crate::errors::Error::DeserializationFailure(err.to_string())),
    }
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
          .invoke(self.clone(), request_context, state, type_erased_input)
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
      // TODO: Re-wrap HandlerInvocationError as HookInvocationError?
      Err(error) => Err(error),
    }
  }

  pub async fn join(self) {
    if let Some(listener_handle) = self.listener_handle.lock().await.take() {
      listener_handle.await.unwrap().unwrap();
    }
  }
}
