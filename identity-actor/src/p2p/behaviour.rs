// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::AsyncRead;
use futures::AsyncWrite;
use futures::AsyncWriteExt;
use libp2p::core::upgrade;
use libp2p::core::ProtocolName;
use libp2p::request_response::RequestResponse;
use libp2p::request_response::RequestResponseCodec;
use libp2p::request_response::RequestResponseEvent;
use libp2p::swarm::NetworkBehaviour;
use libp2p::Multiaddr;
use libp2p::PeerId;

use tokio::io::{self};

#[derive(Debug, Clone)]
pub struct DidCommProtocol();
#[derive(Clone)]
pub struct DidCommCodec();
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DidCommRequest(pub Vec<u8>);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DidCommResponse(pub Vec<u8>);

impl ProtocolName for DidCommProtocol {
  fn protocol_name(&self) -> &[u8] {
    "/didcomm/1.0.0".as_bytes()
  }
}

#[async_trait::async_trait]
impl RequestResponseCodec for DidCommCodec {
  type Protocol = DidCommProtocol;
  type Request = DidCommRequest;
  type Response = DidCommResponse;

  async fn read_request<T>(&mut self, _protocol: &Self::Protocol, io: &mut T) -> io::Result<Self::Request>
  where
    T: AsyncRead + Unpin + Send,
  {
    let vec = upgrade::read_length_prefixed(io, 1_000_000).await?;
    Ok(DidCommRequest(vec))
  }

  async fn read_response<T>(&mut self, _protocol: &Self::Protocol, io: &mut T) -> io::Result<Self::Response>
  where
    T: AsyncRead + Unpin + Send,
  {
    let vec = upgrade::read_length_prefixed(io, 1_000_000).await?;

    Ok(DidCommResponse(vec))
  }

  async fn write_request<T>(
    &mut self,
    _protocol: &Self::Protocol,
    io: &mut T,
    DidCommRequest(data): Self::Request,
  ) -> io::Result<()>
  where
    T: AsyncWrite + Unpin + Send,
  {
    upgrade::write_length_prefixed(io, data).await?;
    io.close().await
  }

  async fn write_response<T>(
    &mut self,
    _protocol: &Self::Protocol,
    io: &mut T,
    DidCommResponse(data): Self::Response,
  ) -> io::Result<()>
  where
    T: AsyncWrite + Unpin + Send,
  {
    upgrade::write_length_prefixed(io, data).await?;
    io.close().await
  }
}

pub struct DidCommBehaviour {
  inner: RequestResponse<DidCommCodec>,
}

impl NetworkBehaviour for DidCommBehaviour {
  type ProtocolsHandler = <RequestResponse<DidCommCodec> as NetworkBehaviour>::ProtocolsHandler;

  type OutEvent = RequestResponseEvent<DidCommRequest, DidCommResponse>;

  fn new_handler(&mut self) -> Self::ProtocolsHandler {
    self.inner.new_handler()
  }

  fn inject_event(
    &mut self,
    peer_id: PeerId,
    connection: libp2p::core::connection::ConnectionId,
    event: <<Self::ProtocolsHandler as libp2p::swarm::IntoProtocolsHandler>::Handler as libp2p::swarm::ProtocolsHandler>::OutEvent,
  ) {
    self.inner.inject_event(peer_id, connection, event);
  }

  fn poll(
    &mut self,
    cx: &mut std::task::Context<'_>,
    params: &mut impl libp2p::swarm::PollParameters,
  ) -> std::task::Poll<libp2p::swarm::NetworkBehaviourAction<Self::OutEvent, Self::ProtocolsHandler>> {
    self.inner.poll(cx, params)
  }

  fn addresses_of_peer(&mut self, peer_id: &PeerId) -> Vec<Multiaddr> {
    self.inner.addresses_of_peer(peer_id)
  }

  fn inject_connected(&mut self, peer_id: &PeerId) {
    self.inner.inject_connected(peer_id);
  }

  fn inject_disconnected(&mut self, peer_id: &PeerId) {
    self.inner.inject_disconnected(peer_id);
  }

  fn inject_connection_established(
    &mut self,
    peer_id: &PeerId,
    connection_id: &libp2p::core::connection::ConnectionId,
    endpoint: &libp2p::core::ConnectedPoint,
    failed_addresses: Option<&Vec<Multiaddr>>,
  ) {
    self
      .inner
      .inject_connection_established(peer_id, connection_id, endpoint, failed_addresses);
  }

  fn inject_connection_closed(
    &mut self,
    peer_id: &PeerId,
    connection_id: &libp2p::core::connection::ConnectionId,
    connected_point: &libp2p::core::ConnectedPoint,
    handler: <Self::ProtocolsHandler as libp2p::swarm::IntoProtocolsHandler>::Handler,
  ) {
    self
      .inner
      .inject_connection_closed(peer_id, connection_id, connected_point, handler);
  }

  fn inject_address_change(
    &mut self,
    peer_id: &PeerId,
    connection_id: &libp2p::core::connection::ConnectionId,
    old: &libp2p::core::ConnectedPoint,
    new: &libp2p::core::ConnectedPoint,
  ) {
    self.inner.inject_address_change(peer_id, connection_id, old, new);
  }

  fn inject_dial_failure(
    &mut self,
    peer_id: Option<PeerId>,
    handler: Self::ProtocolsHandler,
    error: &libp2p::swarm::DialError,
  ) {
    self.inner.inject_dial_failure(peer_id, handler, error);
  }

  fn inject_listen_failure(
    &mut self,
    local_addr: &Multiaddr,
    send_back_addr: &Multiaddr,
    handler: Self::ProtocolsHandler,
  ) {
    self.inner.inject_listen_failure(local_addr, send_back_addr, handler);
  }

  fn inject_new_listener(&mut self, id: libp2p::core::connection::ListenerId) {
    self.inner.inject_new_listener(id);
  }

  fn inject_new_listen_addr(&mut self, id: libp2p::core::connection::ListenerId, addr: &Multiaddr) {
    self.inner.inject_new_listen_addr(id, addr);
  }

  fn inject_expired_listen_addr(&mut self, id: libp2p::core::connection::ListenerId, addr: &Multiaddr) {
    self.inner.inject_expired_listen_addr(id, addr);
  }

  fn inject_listener_error(
    &mut self,
    id: libp2p::core::connection::ListenerId,
    err: &(dyn std::error::Error + 'static),
  ) {
    self.inner.inject_listener_error(id, err);
  }

  fn inject_listener_closed(&mut self, id: libp2p::core::connection::ListenerId, reason: Result<(), &std::io::Error>) {
    self.inner.inject_listener_closed(id, reason);
  }

  fn inject_new_external_addr(&mut self, addr: &Multiaddr) {
    self.inner.inject_new_external_addr(addr);
  }

  fn inject_expired_external_addr(&mut self, addr: &Multiaddr) {
    self.inner.inject_expired_external_addr(addr);
  }
}
