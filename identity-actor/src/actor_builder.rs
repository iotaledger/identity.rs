// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{types::NamedMessage, Communicator};
use communication_refactored::firewall::FirewallConfiguration;
use communication_refactored::InitKeypair;
use communication_refactored::{ReceiveRequest, ShCommunicationBuilder};
use futures::{channel::mpsc, AsyncRead, AsyncWrite};
use libp2p::Transport;

pub struct CommunicatorBuilder {
  receiver: mpsc::Receiver<ReceiveRequest<NamedMessage, NamedMessage>>,
  comm_builder: ShCommunicationBuilder<NamedMessage, NamedMessage, NamedMessage>,
}

impl CommunicatorBuilder {
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::channel(512);
    let (firewall_sender, _) = mpsc::channel(512);
    let comm_builder = ShCommunicationBuilder::new(firewall_sender, sender, None)
      .with_firewall_config(FirewallConfiguration::allow_all());
    Self { receiver, comm_builder }
  }

  pub async fn build(self) -> Communicator {
    let comm = self.comm_builder.build().await;
    Communicator::from_builder(self.receiver, comm)
  }

  pub async fn build_with_transport<TRA>(self, transport: TRA) -> Communicator
  where
    TRA: Transport + Sized + Clone + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    let comm = self.comm_builder.build_with_transport(transport).await;
    Communicator::from_builder(self.receiver, comm)
  }

  pub fn keys(mut self, keys: InitKeypair) -> Self {
    self.comm_builder = self.comm_builder.with_keys(keys);
    self
  }
}
