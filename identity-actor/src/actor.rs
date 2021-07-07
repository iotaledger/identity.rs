// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::{
  errors::{Error, Result},
  types::{ActorRequest, NamedMessage},
};
use communication_refactored::{Multiaddr, PeerId};
use communication_refactored::{ReceiveRequest, ShCommunication, TransportErr};
use dashmap::DashMap;
use futures::{channel::mpsc, StreamExt};
use tokio::task::{self, JoinHandle};

use crate::RequestHandler;

pub struct Actor {
  comm: ShCommunication<NamedMessage, NamedMessage, NamedMessage>,
  handler_map: Arc<DashMap<String, Box<dyn Send + Sync + FnMut(Vec<u8>) -> Vec<u8>>>>,
  listener_handle: JoinHandle<Result<()>>,
}

impl Actor {
  pub(crate) async fn from_builder(
    receiver: mpsc::Receiver<ReceiveRequest<NamedMessage, NamedMessage>>,
    comm: ShCommunication<NamedMessage, NamedMessage, NamedMessage>,
    handler_map: DashMap<String, Box<dyn Send + Sync + FnMut(Vec<u8>) -> Vec<u8>>>,
    listening_addresses: Vec<Multiaddr>,
  ) -> Result<Self> {
    if listening_addresses.is_empty() {
      comm.start_listening(None).await?;
    }

    for addr in listening_addresses {
      comm.start_listening(Some(addr)).await?;
    }

    let handler_map = Arc::new(handler_map);

    let listener_handle = Self::spawn_listener(receiver, Arc::clone(&handler_map));

    Ok(Self {
      comm,
      handler_map,
      listener_handle,
    })
  }

  pub fn set_handler<H: RequestHandler + 'static>(&self, command_name: &str, handler: H) {
    set_handler(command_name, handler, &self.handler_map);
  }

  pub async fn start_listening(&self, address: Option<Multiaddr>) -> std::result::Result<Multiaddr, TransportErr> {
    self.comm.start_listening(address).await
  }

  pub fn peer_id(&self) -> PeerId {
    self.comm.get_peer_id()
  }

  pub fn stop_listening(&self) {
    self.comm.stop_listening();
  }

  pub fn addrs(&self) -> Vec<Multiaddr> {
    let listeners = self.comm.get_listeners();
    listeners.into_iter().map(|l| l.addrs).flatten().collect()
  }

  /// Start handling incoming requests. This method does not return unless [`stop_listening`] is called.
  /// This method should only be called once on any given instance.
  /// A second caller would immediately receive an [`Error::LockInUse`].
  fn spawn_listener(
    mut receiver: mpsc::Receiver<ReceiveRequest<NamedMessage, NamedMessage>>,
    handler_map: Arc<DashMap<String, Box<dyn Send + Sync + FnMut(Vec<u8>) -> Vec<u8>>>>,
  ) -> JoinHandle<Result<()>> {
    task::spawn(async move {
      let mut handles = vec![];
      loop {
        if let Some(receive_request) = receiver.next().await {
          let handler_handle = Self::spawn_handler(receive_request, Arc::clone(&handler_map));
          handles.push(handler_handle);
        } else {
          return Ok(());
        }
      }
    })
  }

  fn spawn_handler(
    receive_request: ReceiveRequest<NamedMessage, NamedMessage>,
    handler_map: Arc<DashMap<String, Box<dyn Send + Sync + FnMut(Vec<u8>) -> Vec<u8>>>>,
  ) -> JoinHandle<Result<()>> {
    task::spawn(async move {
      let request = receive_request.request;
      let response_tx = receive_request.response_tx;
      let request_name = request.name;

      let response_data = match handler_map.get_mut(&request_name) {
        Some(mut handler) => handler(request.data),
        // TODO: Send error *back to peer*
        None => todo!(), //return Err(Error::UnknownRequest(request.name)),
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
    self.listener_handle.abort();
    let _ = self.listener_handle.await;
    Ok(())
  }

  pub fn add_peer(&self, peer: PeerId, addr: Multiaddr) {
    self.comm.add_address(peer, addr);
  }

  pub async fn send_request<Request: ActorRequest>(&self, peer: PeerId, command: Request) -> Result<Request::Response> {
    let request = NamedMessage::new(Request::request_name(), serde_json::to_vec(&command).unwrap());
    let recv = self.comm.send_request(peer, request);
    let response = recv.response_rx.await.unwrap()?;

    // Map to a `could not deserialize` error
    // And deserialize to a Result<Request::Response>
    // as the request we sent could have been an unkown one (probably other errors exist)
    let response = serde_json::from_slice(&response.data).unwrap();
    Ok(response)
  }
}

pub(crate) fn set_handler<H: RequestHandler + 'static>(
  command_name: &str,
  mut handler: H,
  handler_map: &DashMap<String, Box<dyn Send + Sync + FnMut(Vec<u8>) -> Vec<u8>>>,
) {
  // An approach to directly produce a future from the closure, to work around the lack of async closures.
  // However, we cannot move the handler in directly, because
  // "cannot move out of `handler`, a captured variable in an `FnMut` closure"
  // The issue is always that the produced Future has a reference to the handler
  // let other =
  //   Box::new(move |bytes: Vec<u8>| {
  //     let future = Box::new(async {
  //       let force_bytes_move = bytes;
  //       let request = serde_json::from_slice(&force_bytes_move).unwrap();
  //       let ret = handler.handle(request).await.unwrap();
  //       serde_json::to_vec(&ret).unwrap()
  //     });
  //     future
  //   });
  let closure = Box::new(move |obj_bytes: Vec<u8>| {
    let request = serde_json::from_slice(&obj_bytes).unwrap();
    let ret = futures::executor::block_on(handler.handle(request)).unwrap();
    serde_json::to_vec(&ret).unwrap()
  });

  handler_map.insert(command_name.into(), closure);
}
