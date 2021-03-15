// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
  client::{Client, Network},
  error::Result,
};

/// A `ClientBuilder` is used to generated a customized `Client`.
pub struct ClientBuilder {
  pub(crate) network: Network,
  pub(crate) nodes: Vec<String>,
  pub(crate) node_sync_enabled: bool,
}

impl ClientBuilder {
  /// Creates a new `ClientBuilder`.
  pub fn new() -> Self {
    Self {
      network: Network::default(),
      nodes: Vec::new(),
      node_sync_enabled: true,
    }
  }

  /// Sets the network of the generated `Client`.
  #[must_use]
  pub fn network(mut self, network: Network) -> Self {
    self.network = network;
    self
  }

  /// Adds an IOTA node to the generated `Client`.
  #[must_use]
  pub fn node(mut self, node: impl Into<String>) -> Self {
    self.nodes.push(node.into());
    self
  }

  /// Adds an iterator of IOTA nodes to the generated `Client`.
  pub fn nodes(mut self, nodes: impl IntoIterator<Item = impl Into<String>>) -> Self {
    self.nodes.extend(nodes.into_iter().map(Into::into));
    self
  }

  pub fn node_sync_disabled(mut self) -> Self {
    self.node_sync_enabled = false;
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
