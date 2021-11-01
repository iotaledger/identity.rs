// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::time::Duration;

use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::Network;

const DEFAULT_LOCAL_POW: bool = false;

/// A [`ClientBuilder`] is used to generated a customized [`Client`].
pub struct ClientBuilder {
  pub(super) nodeset: bool,
  pub(super) network: Network,
  pub(super) builder: iota_client::ClientBuilder,
}

impl ClientBuilder {
  /// Creates a new `ClientBuilder`.
  pub fn new() -> Self {
    Self {
      nodeset: false,
      network: Default::default(),
      builder: iota_client::ClientBuilder::new().with_local_pow(DEFAULT_LOCAL_POW),
    }
  }

  /// Sets the IOTA Tangle network.
  pub fn network(mut self, network: Network) -> Self {
    self.builder = self.builder.with_network(network.name_str());
    self.network = network;
    self
  }

  /// Adds an IOTA node by its URL.
  pub fn node(mut self, url: &str) -> Result<Self> {
    self.builder = self.builder.with_node(url)?;
    self.nodeset = true;
    Ok(self)
  }

  /// Adds an IOTA node by its URL to be used as primary node.
  pub fn primary_node(mut self, url: &str, jwt: Option<String>, basic_auth: Option<(&str, &str)>) -> Result<Self> {
    self.builder = self.builder.with_primary_node(url, jwt, basic_auth)?;
    self.nodeset = true;
    Ok(self)
  }

  /// Adds an IOTA node by its URL to be used as primary PoW node (for remote PoW).
  pub fn primary_pow_node(mut self, url: &str, jwt: Option<String>, basic_auth: Option<(&str, &str)>) -> Result<Self> {
    self.builder = self.builder.with_primary_pow_node(url, jwt, basic_auth)?;
    Ok(self)
  }

  /// Adds a permanode by its URL.
  pub fn permanode(mut self, url: &str, jwt: Option<String>, basic_auth: Option<(&str, &str)>) -> Result<Self> {
    self.builder = self.builder.with_permanode(url, jwt, basic_auth)?;
    Ok(self)
  }

  /// Adds an IOTA node by its URL.
  pub fn node_auth(mut self, url: &str, jwt: Option<String>, basic_auth: Option<(&str, &str)>) -> Result<Self> {
    self.builder = self.builder.with_node_auth(url, jwt, basic_auth)?;
    self.nodeset = true;
    Ok(self)
  }

  /// Adds a list of IOTA nodes by their URLs.
  pub fn nodes(mut self, urls: &[&str]) -> Result<Self> {
    self.builder = self.builder.with_nodes(urls)?;
    self.nodeset = !urls.is_empty();
    Ok(self)
  }

  /// Sets the node sync interval.
  pub fn node_sync_interval(mut self, value: Duration) -> Self {
    self.builder = self.builder.with_node_sync_interval(value);
    self
  }

  /// Disables the node sync process.
  pub fn node_sync_disabled(mut self) -> Self {
    self.builder = self.builder.with_node_sync_disabled();
    self
  }

  /// Get node list from the `urls`.
  pub async fn node_pool_urls(mut self, urls: &[String]) -> Result<Self> {
    self.builder = self.builder.with_node_pool_urls(urls).await?;
    self.nodeset = !urls.is_empty();
    Ok(self)
  }

  /// Enables/disables quorum.
  pub fn quorum(mut self, value: bool) -> Self {
    self.builder = self.builder.with_quorum(value);
    self
  }

  /// Sets the number of nodes used for quorum.
  pub fn quorum_size(mut self, value: usize) -> Self {
    self.builder = self.builder.with_quorum_size(value);
    self
  }

  /// Sets the quorum threshold.
  pub fn quorum_threshold(mut self, value: usize) -> Self {
    self.builder = self.builder.with_quorum_threshold(value);
    self
  }

  /// Sets whether PoW is performed locally or remotely.
  pub fn local_pow(mut self, value: bool) -> Self {
    self.builder = self.builder.with_local_pow(value);
    self
  }

  /// Sets the number of seconds that new tips will be requested during PoW.
  pub fn tips_interval(mut self, value: u64) -> Self {
    self.builder = self.builder.with_tips_interval(value);
    self
  }

  /// Sets the default request timeout.
  pub fn request_timeout(mut self, value: Duration) -> Self {
    self.builder = self.builder.with_request_timeout(value);
    self
  }

  /// Creates a new `Client` based on the `ClientBuilder` configuration.
  pub async fn build(self) -> Result<Client> {
    Client::from_builder(self).await
  }
}

impl Debug for ClientBuilder {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("ClientBuilder").field("network", &self.network).finish()
  }
}

impl Default for ClientBuilder {
  fn default() -> Self {
    Self::new()
  }
}
