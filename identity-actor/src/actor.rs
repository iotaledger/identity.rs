// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{any::Any, ops::Deref, sync::Arc};

use crate::{
  asyncfn::AsyncFn,
  errors::{Error, Result},
  traits::{ActorRequest, RequestHandler},
  types::NamedMessage,
};
use communication_refactored::{ListenErr, Multiaddr, PeerId};
use communication_refactored::{ReceiveRequest, ShCommunication};
use dashmap::DashMap;
use futures::{channel::mpsc, Future, StreamExt};
use tokio::task::{self, JoinHandle};
use uuid::Uuid;

pub struct HandlerBuilder {
  object_id: Uuid,
  handlers: Arc<DashMap<String, (Uuid, Box<dyn RequestHandler>)>>,
}

impl HandlerBuilder {
  pub fn add_method<H, R, F, C>(self, cmd: &'static str, handler: AsyncFn<H, R, F, C>) -> Self
  where
    H: Clone + Send + Sync,
    R: ActorRequest + Send + Sync + 'static,
    F: Future<Output = R::Response> + Send + 'static,
    C: 'static + Send + Sync + Fn(H, R) -> F,
  {
    self.handlers.insert(cmd.into(), (self.object_id, Box::new(handler)));
    self
  }
}

pub struct Actor {
  comm: ShCommunication<NamedMessage, NamedMessage, NamedMessage>,
  handlers: Arc<DashMap<String, (Uuid, Box<dyn RequestHandler>)>>,
  objects: Arc<DashMap<Uuid, Box<dyn Any + Send + Sync + 'static>>>,
  listener_handle: Option<JoinHandle<Result<()>>>,
}

impl Actor {
  pub(crate) async fn from_builder(
    receiver: mpsc::Receiver<ReceiveRequest<NamedMessage, NamedMessage>>,
    mut comm: ShCommunication<NamedMessage, NamedMessage, NamedMessage>,
    handlers: DashMap<String, (Uuid, Box<dyn RequestHandler>)>,
    objects: DashMap<Uuid, Box<dyn Any + Send + Sync + 'static>>,
    listening_addresses: Vec<Multiaddr>,
  ) -> Result<Self> {
    let handlers = Arc::new(handlers);
    let objects = Arc::new(objects);

    let listener_handle = if !listening_addresses.is_empty() {
      Some(Self::spawn_listener(
        receiver,
        Arc::clone(&objects),
        Arc::clone(&handlers),
      ))
    } else {
      None
    };

    for addr in listening_addresses {
      comm.start_listening(addr).await?;
    }

    Ok(Self {
      comm,
      handlers,
      objects,
      listener_handle,
    })
  }

  pub fn add_handler<H>(&mut self, handler: H) -> HandlerBuilder
  where
    H: Clone + Any + Send + Sync,
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
    mut receiver: mpsc::Receiver<ReceiveRequest<NamedMessage, NamedMessage>>,
    objects: Arc<DashMap<Uuid, Box<dyn Any + Send + Sync>>>,
    handlers: Arc<DashMap<String, (Uuid, Box<dyn RequestHandler>)>>,
  ) -> JoinHandle<Result<()>> {
    task::spawn(async move {
      let mut handles = vec![];
      loop {
        if let Some(receive_request) = receiver.next().await {
          let handler_handle = Self::spawn_handler(receive_request, Arc::clone(&objects), Arc::clone(&handlers));
          handles.push(handler_handle);
        } else {
          return Ok(());
        }
      }
    })
  }

  fn spawn_handler(
    receive_request: ReceiveRequest<NamedMessage, NamedMessage>,
    objects: Arc<DashMap<Uuid, Box<dyn Any + Send + Sync>>>,
    handlers: Arc<DashMap<String, (Uuid, Box<dyn RequestHandler>)>>,
  ) -> JoinHandle<Result<()>> {
    task::spawn(async move {
      let request = receive_request.request;
      let response_tx = receive_request.response_tx;
      let request_name = request.name;

      let response_data = match handlers.get(&request_name) {
        Some(handler_tuple) => {
          let object_id = handler_tuple.0;
          let handler = &handler_tuple.1;

          let obj = if let Some(object) = objects.get(&object_id) {
            let object_clone = handler.clone_object(object.deref());
            object_clone
          } else {
            todo!("Unhandled object-not-found case.")
          };

          handler.invoke(obj, request.data).await
        }
        // TODO: Send error *back to peer*
        None => todo!("Unhandled handler-not-found case"), //return Err(Error::UnknownRequest(request.name)),
      };

      let response = NamedMessage::new(request_name, response_data);

      // TODO: can return an error when
      // - connection times out, when
      // - when handler takes too long to respond (configurable via SwarmBuilder.with_timeout)
      // - error on the transport layer
      // - potentially others...
      response_tx
        .send(response)
        .map_err(|response| Error::CouldNotRespond(response.name))
    })
  }

  pub async fn stop_handling_requests(self) -> Result<()> {
    // TODO: aborting means that even requests that have been received and are being processed are cancelled
    // We should instead use some signalling mechanism that breaks the loop
    if let Some(listener_handle) = self.listener_handle {
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
    let request = NamedMessage::new(name, serde_json::to_vec(&command).unwrap());
    log::info!("Sending NamedMessage: {:?}, json payload: {:?}", request, serde_json::to_value(&command).unwrap());
    let response = self.comm.send_request(peer, request).await.unwrap();

    log::info!("Json response: {:?}", serde_json::from_slice::<'_, serde_json::Value>(&response.data).unwrap());

    // Map to a `could not deserialize` error
    // And deserialize to a Result<Request::Response>
    // as the request we sent could have been an unkown one (probably other errors exist)
    let response = serde_json::from_slice(&response.data).unwrap();
    Ok(response)
  }

  pub async fn join(self) {
    if let Some(listener_handle) = self.listener_handle {
      listener_handle.await.unwrap().unwrap();
    }
  }
}
