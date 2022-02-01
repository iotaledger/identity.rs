// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::future::poll_fn;

use libp2p::request_response::OutboundFailure;
use libp2p::request_response::ResponseChannel;
use libp2p::Multiaddr;
use libp2p::PeerId;
use libp2p::TransportError;

use super::behaviour::DidCommResponse;

#[derive(Clone)]
pub struct NetCommander {
  command_sender: mpsc::Sender<SwarmCommand>,
}

impl NetCommander {
  pub fn new(command_sender: mpsc::Sender<SwarmCommand>) -> Self {
    NetCommander { command_sender }
  }

  pub async fn send_request(&mut self, peer: PeerId, request: Vec<u8>) -> Result<(), OutboundFailure> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::SendRequest {
      peer,
      request,
      response_channel: sender,
    };
    self.send_command(command).await;
    receiver.await.unwrap()
  }

  pub async fn send_response(&mut self, data: Vec<u8>, channel: ResponseChannel<DidCommResponse>) {
    let (sender, _receiver) = oneshot::channel();
    let command = SwarmCommand::SendResponse {
      response: data,
      cmd_response_channel: sender,
      response_channel: channel,
    };
    self.send_command(command).await;
  }

  pub async fn start_listening(&mut self, address: Multiaddr) -> Result<(), TransportError<std::io::Error>> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::StartListening {
      address,
      response_channel: sender,
    };
    self.send_command(command).await;
    receiver.await.unwrap()
  }

  pub async fn add_address(&mut self, peer: PeerId, address: Multiaddr) {
    self.send_command(SwarmCommand::AddAddress { peer, address }).await;
  }

  pub async fn get_addresses(&mut self) -> Vec<Multiaddr> {
    let (sender, receiver) = oneshot::channel();
    self
      .send_command(SwarmCommand::GetAddresses {
        response_channel: sender,
      })
      .await;
    receiver.await.expect("sender was dropped")
  }

  // TODO: Remove this and store the peer id on the actor itself,
  // but only after it's internal representation has been optimized/refactored.
  pub async fn peer_id(&mut self) -> PeerId {
    let (sender, receiver) = oneshot::channel();
    self
      .send_command(SwarmCommand::GetPeerId {
        response_channel: sender,
      })
      .await;
    receiver.await.expect("sender was dropped")
  }

  async fn send_command(&mut self, command: SwarmCommand) {
    let _ = poll_fn(|cx| self.command_sender.poll_ready(cx)).await;
    let _ = self.command_sender.start_send(command);
  }
}

pub enum SwarmCommand {
  SendRequest {
    peer: PeerId,
    request: Vec<u8>,
    response_channel: oneshot::Sender<Result<(), OutboundFailure>>,
  },
  SendResponse {
    response: Vec<u8>,
    cmd_response_channel: oneshot::Sender<Result<(), OutboundFailure>>,
    response_channel: ResponseChannel<DidCommResponse>,
  },
  StartListening {
    address: Multiaddr,
    response_channel: oneshot::Sender<Result<(), TransportError<std::io::Error>>>,
  },
  AddAddress {
    peer: PeerId,
    address: Multiaddr,
  },
  GetAddresses {
    response_channel: oneshot::Sender<Vec<Multiaddr>>,
  },
  GetPeerId {
    response_channel: oneshot::Sender<PeerId>,
  },
}
