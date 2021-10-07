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

pub struct HandlerBuilder {
  object_id: Uuid,
  handlers: Arc<HandlerMap>,
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
    let handle = actor
      .clone()
      .spawn_listener(receiver, Arc::clone(&objects), Arc::clone(&handlers));

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
    objects: Arc<ObjectMap>,
    handlers: Arc<HandlerMap>,
  ) -> JoinHandle<Result<()>> {
    task::spawn(async move {
      let mut handles = vec![];
      loop {
        if let Some(receive_request) = receiver.next().await {
          let handler_handle = self
            .clone()
            .spawn_handler(receive_request, Arc::clone(&objects), Arc::clone(&handlers));
          handles.push(handler_handle);
        } else {
          return Ok(());
        }
      }
    })
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

  fn spawn_handler(
    self,
    receive_request: ReceiveRequest<RequestMessage, ResponseMessage>,
    objects: Arc<ObjectMap>,
    handlers: Arc<HandlerMap>,
  ) -> JoinHandle<Result<()>> {
    task::spawn(async move {
      let request = receive_request.request;
      let response_tx = receive_request.response_tx;
      let request_name = request.name;

      match handlers.get(&request_name) {
        Some(handler_tuple) => {
          log::info!("Invoking named handler for {}", request_name);

          let object_id = handler_tuple.0;
          let handler = &handler_tuple.1;

          if let Some(object) = objects.get(&object_id) {
            let object_clone = handler.clone_object(object.deref());
            let request_context: RequestContext<()> =
              RequestContext::new((), receive_request.peer, request_name.clone());

            // Send dummy ack that message was received
            Self::send_ack(response_tx);

            handler.invoke(self, request_context, object_clone, request.data).await
          } else {
            // SAFETY: Serialization of this type never fails
            serde_json::to_vec(&RemoteSendError::HandlerInvocationError(format!(
              "no object set for {}",
              request_name
            )))
            .unwrap()
          }
        }
        None => {
          match handlers.get(&request_name.clone().to_catch_all()) {
            Some(handler_tuple) => {
              log::info!("Invoking catch_all handler for {}", request_name);

              let object_id = handler_tuple.0;
              let handler = &handler_tuple.1;

              if let Some(object) = objects.get(&object_id) {
                let object_clone = handler.clone_object(object.deref());
                let request_context: RequestContext<()> =
                  RequestContext::new((), receive_request.peer, request_name.clone());

                // Send dummy ack that message was received
                Self::send_ack(response_tx);

                handler.invoke(self, request_context, object_clone, request.data).await
              } else {
                log::info!("No catch_all handler for {}", request_name);
                // SAFETY: Serialization of this type never fails
                serde_json::to_vec(&RemoteSendError::HandlerInvocationError(format!(
                  "no object set for {}",
                  request_name
                )))
                .unwrap()
              }
            }
            None => {
              // SAFETY: Serialization of this type never fails
              serde_json::to_vec(&RemoteSendError::UnknownRequest(request_name.to_string())).unwrap()
            }
          }
        }
      };

      Ok(())
    })
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

    log::info!(
      "Sending `{}` request with payload: {}",
      request.name,
      serde_json::to_string_pretty(&command).unwrap()
    );

    let response = self.comm.send_request(peer, request).await?;

    let request_response: serde_json::Result<Request::Response> = serde_json::from_slice(&response);

    match request_response {
      Ok(res) => Ok(res),
      Err(err) => Err(crate::errors::Error::ResponseDeserializationFailure(err.to_string())),
    }
  }

  pub async fn join(self) {
    if let Some(listener_handle) = self.listener_handle.lock().await.take() {
      listener_handle.await.unwrap().unwrap();
    }
  }
}
