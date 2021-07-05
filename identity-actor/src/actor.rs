// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
  errors::{Error, Result},
  types::NamedMessage,
};
use communication_refactored::{
  ReceiveRequest, ShCommunication, TransportErr,
};
use communication_refactored::{Multiaddr, PeerId};
use dashmap::DashMap;
use futures::{channel::mpsc, lock::Mutex, StreamExt};
use serde::{de::DeserializeOwned, Serialize};

use crate::IdentityRequestHandler;

pub struct Communicator {
  comm: ShCommunication<NamedMessage, NamedMessage, NamedMessage>,
  handler_map: DashMap<String, Box<dyn Send + Sync + FnMut(Vec<u8>) -> Vec<u8>>>,
  receiver: Mutex<mpsc::Receiver<ReceiveRequest<NamedMessage, NamedMessage>>>,
}

impl Communicator {
  pub fn register_command<H: IdentityRequestHandler + 'static>(&self, command_name: &str, mut handler: H) {
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

    self.handler_map.insert(command_name.into(), closure);
  }

  pub(crate) fn from_builder(
    receiver: mpsc::Receiver<ReceiveRequest<NamedMessage, NamedMessage>>,
    comm: ShCommunication<NamedMessage, NamedMessage, NamedMessage>,
  ) -> Self {
    Self {
      comm,
      handler_map: DashMap::new(),
      receiver: Mutex::new(receiver),
    }
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

  /// Start handling incoming requests. This method does not return unless [`stop_listening`] is called.
  /// This method should only be called once on any given instance.
  /// A second caller would immediately receive an [`Error::LockInUse`].
  pub async fn handle_requests(&self) -> Result<()> {
    let mut receiver = self.receiver.try_lock().ok_or(Error::LockInUse)?;

    loop {
      if let Some(ReceiveRequest {
        response_tx, request, ..
      }) = receiver.next().await
      {
        let response_data = match self.handler_map.get_mut(&request.name) {
          Some(mut handler) => handler(request.data),
          None => return Err(Error::UnknownRequest(request.name)),
        };

        let response = NamedMessage::new(request.name, response_data);

        response_tx.send(response).unwrap();
      } else {
        return Ok(());
      }
    }
  }

  pub fn add_peer(&self, peer: PeerId, addr: Multiaddr) {
    self.comm.add_address(peer, addr);
  }

  pub async fn send_command<Ret, Cmd>(&self, peer: PeerId, command: Cmd) -> Result<Ret>
  where
    Cmd: Serialize,
    Ret: DeserializeOwned,
  {
    // TODO: Get string from somewhere based on given type
    let request = NamedMessage::new("IdentityStorage", serde_json::to_vec(&command).unwrap());
    let recv = self.comm.send_request(peer, request);
    let response = recv.response_rx.await.unwrap()?;

    let response: Ret = serde_json::from_slice(&response.data).unwrap();
    Ok(response)
  }
}
