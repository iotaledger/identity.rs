// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::errors::Result;
use crate::types::ResponseMessage;
use crate::{types::RequestMessage, Actor};
use dashmap::DashMap;
use futures::{channel::mpsc, AsyncRead, AsyncWrite};
use libp2p::{core::Transport, Multiaddr};
use p2p::firewall::FirewallConfiguration;
use p2p::{EventChannel, Executor, InitKeypair, ReceiveRequest, StrongholdP2pBuilder};

pub struct ActorBuilder {
  receiver: mpsc::Receiver<ReceiveRequest<RequestMessage, ResponseMessage>>,
  comm_builder: StrongholdP2pBuilder<RequestMessage, ResponseMessage>,
  listening_addresses: Vec<Multiaddr>,
}

const DEFAULT_CAPACITY: usize = 1024;

impl ActorBuilder {
  pub fn new() -> Self {
    let (sender, receiver) = EventChannel::new(DEFAULT_CAPACITY, p2p::ChannelSinkConfig::BufferLatest);
    let (firewall_sender, _) = mpsc::channel(1);

    let comm_builder =
      StrongholdP2pBuilder::new(firewall_sender, sender, None).with_firewall_config(FirewallConfiguration::allow_all());
    Self {
      receiver,
      comm_builder,
      listening_addresses: vec![],
    }
  }

  #[cfg(feature = "tcp")]
  pub async fn build(self) -> Result<Actor> {
    let comm = self.comm_builder.build().await?;
    let handlers = DashMap::new();
    let objects = DashMap::new();
    Actor::from_builder(self.receiver, comm, handlers, objects, self.listening_addresses).await
  }

  pub async fn build_with_transport_and_executor<TRA, EXE>(self, transport: TRA, executor: EXE) -> Result<Actor>
  where
    TRA: Transport + Sized + Clone + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
    EXE: Executor + Send + 'static + Clone,
  {
    let comm = self.comm_builder.build_with_transport(transport, executor).await;
    let handlers = DashMap::new();
    let objects = DashMap::new();
    Actor::from_builder(self.receiver, comm, handlers, objects, self.listening_addresses).await
  }

  pub async fn build_with_transport<TRA>(self, transport: TRA) -> Result<Actor>
  where
    TRA: Transport + Sized + Clone + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    let executor = |fut| {
      tokio::spawn(fut);
    };
    let comm = self.comm_builder.build_with_transport(transport, executor).await;
    let handlers = DashMap::new();
    let objects = DashMap::new();
    Actor::from_builder(self.receiver, comm, handlers, objects, self.listening_addresses).await
  }

  pub fn keys(mut self, keys: InitKeypair) -> Self {
    self.comm_builder = self.comm_builder.with_keys(keys);
    self
  }

  pub fn listen_on(mut self, address: Multiaddr) -> Self {
    self.listening_addresses.push(address);
    self
  }
}
