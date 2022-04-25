// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use std::time::Duration;

use identity_iota_core::tangle::Network;

use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::DIDMessageEncoding;

const DEFAULT_LOCAL_POW: bool = false;
const DEFAULT_RETRY_UNTIL_INCLUDED: bool = true;

/// A [`ClientBuilder`] is used to generated a customized [`Client`].
pub struct ClientBuilder {
  pub(super) nodeset: bool,
  pub(super) network: Network,
  pub(super) builder: iota_client::ClientBuilder,
  pub(super) encoding: DIDMessageEncoding,
  pub(crate) retry_until_included: bool,
}

impl ClientBuilder {
  /// Creates a new `ClientBuilder`.
  pub fn new() -> Self {
    Self {
      nodeset: false,
      network: Default::default(),
      builder: iota_client::ClientBuilder::new().with_local_pow(DEFAULT_LOCAL_POW),
      encoding: DIDMessageEncoding::JsonBrotli,
      retry_until_included: DEFAULT_RETRY_UNTIL_INCLUDED,
    }
  }

  /// Sets the IOTA Tangle network.
  #[must_use]
  pub fn network(mut self, network: Network) -> Self {
    self.builder = self.builder.with_network(network.name_str());
    self.network = network;
    self
  }

  /// Sets the DID message encoding used when publishing to the Tangle.
  #[must_use]
  pub fn encoding(mut self, encoding: DIDMessageEncoding) -> Self {
    self.encoding = encoding;
    self
  }

  
  /// When publishing to the Tangle, sets whether to retry until the message is 
  /// confirmed by a milestone.
  /// Default: true.
  pub fn retry_until_included(mut self, value: bool) -> Self {
    self.retry_until_included = value;
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
  #[must_use]
  pub fn node_sync_interval(mut self, value: Duration) -> Self {
    self.builder = self.builder.with_node_sync_interval(value);
    self
  }

  /// Disables the node sync process.
  #[must_use]
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
  #[must_use]
  pub fn quorum(mut self, value: bool) -> Self {
    self.builder = self.builder.with_quorum(value);
    self
  }

  /// Sets the number of nodes used for quorum.
  #[must_use]
  pub fn quorum_size(mut self, value: usize) -> Self {
    self.builder = self.builder.with_quorum_size(value);
    self
  }

  /// Sets the quorum threshold.
  #[must_use]
  pub fn quorum_threshold(mut self, value: usize) -> Self {
    self.builder = self.builder.with_quorum_threshold(value);
    self
  }

  /// Sets whether proof-of-work (PoW) is performed locally or remotely.
  ///
  /// Default: false.
  #[must_use]
  pub fn local_pow(mut self, value: bool) -> Self {
    self.builder = self.builder.with_local_pow(value);
    self
  }

  /// Sets whether the PoW should be done locally in case a node doesn't support remote PoW.
  ///
  /// Default: true.
  #[must_use]
  pub fn fallback_to_local_pow(mut self, value: bool) -> Self {
    self.builder = self.builder.with_fallback_to_local_pow(value);
    self
  }

  /// Sets the number of seconds that new tips will be requested during PoW.
  #[must_use]
  pub fn tips_interval(mut self, value: u64) -> Self {
    self.builder = self.builder.with_tips_interval(value);
    self
  }

  /// Sets the default request timeout.
  #[must_use]
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
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("ClientBuilder").field("network", &self.network).finish()
  }
}

impl Default for ClientBuilder {
  fn default() -> Self {
    Self::new()
  }
}
