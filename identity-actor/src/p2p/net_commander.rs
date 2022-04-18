// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::future::poll_fn;

use identity_core::common::OneOrMany;
use libp2p::request_response::InboundFailure;
use libp2p::request_response::OutboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;
use libp2p::Multiaddr;
use libp2p::PeerId;
use libp2p::TransportError;

use crate::Error;

use super::message::RequestMessage;
use super::message::ResponseMessage;

/// A thread-safe way to interact with an `EventLoop` running in the background.
#[derive(Clone)]
pub struct NetCommander {
  command_sender: mpsc::Sender<SwarmCommand>,
}

impl NetCommander {
  pub fn new(command_sender: mpsc::Sender<SwarmCommand>) -> Self {
    NetCommander { command_sender }
  }

  pub async fn send_request(&mut self, peer: PeerId, request: RequestMessage) -> crate::Result<ResponseMessage> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::SendRequest {
      peer,
      request,
      response_channel: sender,
    };
    self.send_command(command).await?;
    receiver
      .await
      .map_err(|_| Error::Shutdown)?
      .map_err(Error::OutboundFailure)
  }

  pub async fn send_response(
    &mut self,
    data: Vec<u8>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  ) -> crate::Result<Result<(), InboundFailure>> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::SendResponse {
      response: data,
      cmd_response_channel: sender,
      response_channel: channel,
      request_id,
    };
    self.send_command(command).await?;
    receiver.await.map_err(|_| Error::Shutdown)
  }

  pub async fn start_listening(&mut self, address: Multiaddr) -> crate::Result<Multiaddr> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::StartListening {
      address,
      response_channel: sender,
    };
    self.send_command(command).await?;
    receiver
      .await
      .map_err(|_| Error::Shutdown)?
      .map_err(|transport_err| Error::TransportError {
        context: "unable to start listening",
        source: transport_err,
      })
  }

  pub async fn add_addresses(&mut self, peer: PeerId, addresses: OneOrMany<Multiaddr>) -> crate::Result<()> {
    self.send_command(SwarmCommand::AddAddresses { peer, addresses }).await
  }

  pub async fn get_addresses(&mut self) -> crate::Result<Vec<Multiaddr>> {
    let (sender, receiver) = oneshot::channel();
    self
      .send_command(SwarmCommand::GetAddresses {
        response_channel: sender,
      })
      .await?;
    receiver.await.map_err(|_| Error::Shutdown)
  }

  pub async fn shutdown(&mut self) -> crate::Result<()> {
    let (sender, receiver) = oneshot::channel();
    self
      .send_command(SwarmCommand::Shutdown {
        response_channel: sender,
      })
      .await?;
    receiver.await.map_err(|_| Error::Shutdown)
  }

  async fn send_command(&mut self, command: SwarmCommand) -> crate::Result<()> {
    poll_fn(|cx| self.command_sender.poll_ready(cx))
      .await
      .map_err(|_| Error::Shutdown)?;
    self.command_sender.start_send(command).map_err(|_| Error::Shutdown)
  }
}

/// A command to send to the `EventLoop` with (typically) a channel to return a response through.
pub enum SwarmCommand {
  SendRequest {
    peer: PeerId,
    request: RequestMessage,
    response_channel: oneshot::Sender<Result<ResponseMessage, OutboundFailure>>,
  },
  SendResponse {
    response: Vec<u8>,
    cmd_response_channel: oneshot::Sender<Result<(), InboundFailure>>,
    response_channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  },
  StartListening {
    address: Multiaddr,
    response_channel: oneshot::Sender<Result<Multiaddr, TransportError<std::io::Error>>>,
  },
  AddAddresses {
    peer: PeerId,
    addresses: OneOrMany<Multiaddr>,
  },
  GetAddresses {
    response_channel: oneshot::Sender<Vec<Multiaddr>>,
  },
  Shutdown {
    response_channel: oneshot::Sender<()>,
  },
}
