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

use crate::actor::Error;
use crate::actor::Result as AgentResult;
use crate::p2p::RequestMessage;
use crate::p2p::ResponseMessage;

/// A thread-safe way to interact with an `EventLoop` running in the background.
#[derive(Debug, Clone)]
pub(crate) struct NetCommander {
  command_sender: mpsc::Sender<SwarmCommand>,
}

impl NetCommander {
  /// Create a new [`NetCommander`] from the sender half of a channel.
  /// The receiver half needs to be passed to the `EventLoop`.
  pub(crate) fn new(command_sender: mpsc::Sender<SwarmCommand>) -> Self {
    NetCommander { command_sender }
  }

  /// Send the `request` to `peer` and returns the response.
  pub(crate) async fn send_request(
    &mut self,
    peer_id: PeerId,
    request: RequestMessage,
  ) -> AgentResult<ResponseMessage> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::SendRequest {
      peer_id,
      request,
      response_channel: sender,
    };
    self.send_command(command).await?;
    receiver
      .await
      .map_err(|_| Error::Shutdown)?
      .map_err(Error::OutboundFailure)
  }

  /// Send `data` as a response for the `request_id` using the provided `channel`.
  /// The inner result signals whether sending the response was successful.
  pub(crate) async fn send_response(
    &mut self,
    data: Vec<u8>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  ) -> AgentResult<Result<(), InboundFailure>> {
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

  /// Start listening on the given address.
  pub(crate) async fn start_listening(&mut self, address: Multiaddr) -> AgentResult<Multiaddr> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::StartListening {
      address,
      response_channel: sender,
    };
    self.send_command(command).await?;
    receiver
      .await
      .map_err(|_| Error::Shutdown)?
      .map_err(|transport_err| Error::TransportError("start listening", transport_err))
  }

  /// Add additional `addresses` to listen on.
  pub(crate) async fn add_addresses(&mut self, peer_id: PeerId, addresses: OneOrMany<Multiaddr>) -> AgentResult<()> {
    self
      .send_command(SwarmCommand::AddAddresses { peer_id, addresses })
      .await
  }

  /// Returns all addresses the event loop is listening on.
  pub(crate) async fn get_addresses(&mut self) -> AgentResult<Vec<Multiaddr>> {
    let (sender, receiver) = oneshot::channel();
    self
      .send_command(SwarmCommand::GetAddresses {
        response_channel: sender,
      })
      .await?;
    receiver.await.map_err(|_| Error::Shutdown)
  }

  /// Shut down the event loop. This will return `Error::Shutdown` from all outstanding requests.
  pub(crate) async fn shutdown(&mut self) -> AgentResult<()> {
    let (sender, receiver) = oneshot::channel();
    self
      .send_command(SwarmCommand::Shutdown {
        response_channel: sender,
      })
      .await?;
    receiver.await.map_err(|_| Error::Shutdown)
  }

  /// Send a command to the event loop.
  async fn send_command(&mut self, command: SwarmCommand) -> AgentResult<()> {
    poll_fn(|cx| self.command_sender.poll_ready(cx))
      .await
      .map_err(|_| Error::Shutdown)?;
    self.command_sender.start_send(command).map_err(|_| Error::Shutdown)
  }
}

/// A command to send to the `EventLoop` with (typically) a channel to return a response through.
///
/// See the [`NetCommander`] methods for documentation.
#[derive(Debug)]
pub(crate) enum SwarmCommand {
  SendRequest {
    peer_id: PeerId,
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
    peer_id: PeerId,
    addresses: OneOrMany<Multiaddr>,
  },
  GetAddresses {
    response_channel: oneshot::Sender<Vec<Multiaddr>>,
  },
  Shutdown {
    response_channel: oneshot::Sender<()>,
  },
}
