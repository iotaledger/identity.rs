// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
  errors::{Error, Result},
  types::NamedRequest,
};
use communication_refactored::{
  firewall::FirewallConfiguration, Keypair, ReceiveRequest, ShCommunication, ShCommunicationBuilder, TransportErr,
};
use communication_refactored::{InitKeypair, Multiaddr, PeerId};
use dashmap::DashMap;
use futures::{channel::mpsc, lock::Mutex, StreamExt};
use libp2p::tcp::TcpConfig;
use serde::{de::DeserializeOwned, Serialize};

use crate::IdentityRequestHandler;

pub struct IdentityCommunicator {
  comm: ShCommunication<NamedRequest, NamedRequest, NamedRequest>,
  handler_map: DashMap<String, Box<dyn Send + Sync + FnMut(Vec<u8>) -> Vec<u8>>>,
  receiver: Mutex<mpsc::Receiver<ReceiveRequest<NamedRequest, NamedRequest>>>,
}

impl IdentityCommunicator {
  pub fn register_command<H: IdentityRequestHandler + 'static>(&self, command_name: &str, mut handler: H) {
    let closure = Box::new(move |obj_bytes: Vec<u8>| {
      let request = serde_json::from_slice(&obj_bytes).unwrap();
      let ret = futures::executor::block_on(handler.handle(request)).unwrap();
      serde_json::to_vec(&ret).unwrap()
    });

    self.handler_map.insert(command_name.into(), closure);
  }

  pub async fn new() -> Self {
    let id_keys = Keypair::generate_ed25519();

    let transport = TcpConfig::new().nodelay(true);
    let (dummy_tx, _) = mpsc::channel(1);
    let (rq_tx, rq_rx) = mpsc::channel(1);
    let comm = ShCommunicationBuilder::new(dummy_tx, rq_tx, None)
      .with_firewall_config(FirewallConfiguration::allow_all())
      .with_keys(InitKeypair::IdKeys(id_keys))
      .build_with_transport(transport)
      .await;

    Self {
      handler_map: DashMap::new(),
      comm,
      receiver: Mutex::new(rq_rx),
    }
  }

  pub async fn start_listening(&mut self, address: Option<Multiaddr>) -> std::result::Result<Multiaddr, TransportErr> {
    self.comm.start_listening(address).await
  }

  pub fn peer_id(&self) -> PeerId {
    self.comm.get_peer_id()
  }

  pub fn stop_listening(&mut self) {
    self.comm.stop_listening();
  }

  /// Start handling incoming requests. This method does not return unless [`stop_listening`] is called.
  /// This method should only be called once on any given instance.
  /// A second caller would immediately receive an [`Error::LockInUse`].
  pub async fn handle_requests(&self) -> Result<()> {
    let mut receiver = self.receiver.try_lock().ok_or(Error::LockInUse)?;

    loop {
      let ReceiveRequest {
        peer: _,
        request_id: _,
        response_tx,
        request,
      } = receiver.next().await.expect("Is only called on shutdown");

      let response_data = match self.handler_map.get_mut(&request.name) {
        Some(mut handler) => handler(request.data),
        None => {
          // TODO: Return as `Err`
          panic!("Unknown request name");
        }
      };

      let response = NamedRequest {
        name: request.name,
        data: response_data,
      };

      response_tx.send(response).unwrap();
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
    let request = NamedRequest::new("IdentityStorage", serde_json::to_vec(&command).unwrap());
    let recv = self.comm.send_request(peer, request);
    let response = recv.response_rx.await.unwrap()?;

    let response: Ret = serde_json::from_slice(&response.data).unwrap();
    Ok(response)
  }
}
