// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::Client;
use crate::client::Network;
use crate::error::Result;

/// A `ClientBuilder` is used to generated a customized [Client].
#[derive(Clone, Debug)]
pub struct ClientBuilder {
  pub(crate) network: Network,
  pub(crate) nodes: Vec<String>,
}

impl ClientBuilder {
  /// Creates a new `ClientBuilder`.
  pub const fn new() -> Self {
    Self {
      network: Network::Mainnet,
      nodes: Vec::new(),
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

  /// Creates a new `Client` based on the `ClientBuilder` configuration.
  pub fn build(self) -> Result<Client> {
    Client::from_builder(self)
  }
}

impl Default for ClientBuilder {
  fn default() -> Self {
    Self::new()
  }
}
