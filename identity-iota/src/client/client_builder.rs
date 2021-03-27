// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::Client;
use crate::client::Network;
use crate::error::Result;

/// Sets the default node syncing process.
/// For Chrysalis network (Testnet) we need node_sync_enabled to be false (default).
const NODE_SYNC_ENABLED: bool = false;

/// A `ClientBuilder` is used to generated a customized `Client`.
#[derive(Clone, Debug)]
pub struct ClientBuilder {
  pub(crate) network: Network,
  pub(crate) nodes: Vec<String>,
  pub(crate) node_sync_enabled: bool,
}

impl ClientBuilder {
  /// Creates a new `ClientBuilder`.
  pub const fn new() -> Self {
    Self {
      network: Network::Mainnet,
      nodes: Vec::new(),
      node_sync_enabled: NODE_SYNC_ENABLED,
    }
  }

  /// Sets the network.
  #[must_use]
  pub fn network(mut self, network: Network) -> Self {
    self.network = network;
    self
  }

  /// Adds an IOTA node.
  #[must_use]
  pub fn node(mut self, node: impl Into<String>) -> Self {
    self.nodes.push(node.into());
    self
  }

  /// Adds an iterator of IOTA nodes.
  pub fn nodes(mut self, nodes: impl IntoIterator<Item = impl Into<String>>) -> Self {
    self.nodes.extend(nodes.into_iter().map(Into::into));
    self
  }

  pub fn node_sync_enabled(mut self, value: bool) -> Self {
    self.node_sync_enabled = value;
    self
  }

  /// Creates a new `Client` based on the `ClientBuilder` configuration.
  pub async fn build(self) -> Result<Client> {
    Client::from_builder(self).await
  }
}

impl Default for ClientBuilder {
  fn default() -> Self {
    Self::new()
  }
}
