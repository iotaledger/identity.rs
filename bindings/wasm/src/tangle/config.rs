// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::ClientBuilder;
use std::time::Duration;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;
use crate::tangle::WasmDIDMessageEncoding;
use crate::tangle::WasmNetwork;

fn to_duration(seconds: u32) -> Duration {
  Duration::from_secs(u64::from(seconds))
}

fn to_basic_auth<'a>(username: &'a Option<String>, password: &'a Option<String>) -> Option<(&'a str, &'a str)> {
  username.as_deref().zip(password.as_deref())
}

/// Options to configure a new {@link Client}.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Config {
  pub(crate) builder: Option<ClientBuilder>,
}

#[wasm_bindgen]
impl Config {
  /// Creates a new `Config`.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      builder: Some(ClientBuilder::new()),
    }
  }

  /// Creates a new `Config` for the given IOTA Tangle network.
  #[wasm_bindgen(js_name = fromNetwork)]
  pub fn from_network(network: &WasmNetwork) -> Result<Config, JsValue> {
    let mut this: Self = Self::new();
    this.set_network(network)?;
    Ok(this)
  }

  /// Sets the IOTA Tangle network.
  #[wasm_bindgen(js_name = setNetwork)]
  pub fn set_network(&mut self, network: &WasmNetwork) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.network(network.0.clone()))
  }

  /// Sets the DID message encoding used when publishing to the Tangle.
  #[wasm_bindgen(js_name = setEncoding)]
  pub fn set_encoding(&mut self, encoding: WasmDIDMessageEncoding) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.encoding(encoding.into()))
  }

  /// Adds an IOTA node by its URL.
  #[wasm_bindgen(js_name = setNode)]
  pub fn set_node(&mut self, url: &str) -> Result<(), JsValue> {
    self.try_with_mut(|builder| builder.node(url).map_err(wasm_error))
  }

  /// Adds an IOTA node by its URL to be used as primary node.
  #[wasm_bindgen(js_name = setPrimaryNode)]
  pub fn set_primary_node(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<(), JsValue> {
    self.try_with_mut(|builder| {
      builder
        .primary_node(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(wasm_error)
    })
  }

  /// Adds an IOTA node by its URL to be used as primary PoW node (for remote PoW).
  #[wasm_bindgen(js_name = setPrimaryPoWNode)]
  pub fn set_primary_pow_node(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<(), JsValue> {
    self.try_with_mut(|builder| {
      builder
        .primary_pow_node(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(wasm_error)
    })
  }

  /// Adds a permanode by its URL.
  #[wasm_bindgen(js_name = setPermanode)]
  pub fn set_permanode(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<(), JsValue> {
    self.try_with_mut(|builder| {
      builder
        .permanode(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(wasm_error)
    })
  }

  /// Adds an IOTA node by its URL.
  #[wasm_bindgen(js_name = setNodeAuth)]
  pub fn set_node_auth(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<(), JsValue> {
    self.try_with_mut(|builder| {
      builder
        .node_auth(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(wasm_error)
    })
  }

  /// Sets the node sync interval.
  #[wasm_bindgen(js_name = setNodeSyncInterval)]
  pub fn set_node_sync_interval(&mut self, value: u32) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.node_sync_interval(to_duration(value)))
  }

  /// Disables the node sync process.
  #[wasm_bindgen(js_name = setNodeSyncDisabled)]
  pub fn set_node_sync_disabled(&mut self) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.node_sync_disabled())
  }

  /// Enables/disables quorum.
  #[wasm_bindgen(js_name = setQuorum)]
  pub fn set_quorum(&mut self, value: bool) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.quorum(value))
  }

  /// Sets the number of nodes used for quorum.
  #[wasm_bindgen(js_name = setQuorumSize)]
  pub fn set_quorum_size(&mut self, value: usize) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.quorum_size(value))
  }

  /// Sets the quorum threshold.
  #[wasm_bindgen(js_name = setQuorumThreshold)]
  pub fn set_quorum_threshold(&mut self, value: usize) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.quorum_threshold(value))
  }

  /// Sets whether proof-of-work (PoW) is performed locally or remotely.
  ///
  /// Default: false.
  #[wasm_bindgen(js_name = setLocalPoW)]
  pub fn set_local_pow(&mut self, value: bool) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.local_pow(value))
  }

  /// Sets whether the PoW should be done locally in case a node doesn't support remote PoW.
  ///
  /// Default: true.
  #[wasm_bindgen(js_name = setFallbackToLocalPoW)]
  pub fn set_fallback_to_local_pow(&mut self, value: bool) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.fallback_to_local_pow(value))
  }

  /// Sets the number of seconds that new tips will be requested during PoW.
  #[wasm_bindgen(js_name = setTipsInterval)]
  pub fn set_tips_interval(&mut self, value: u32) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.tips_interval(u64::from(value)))
  }

  /// Sets the default request timeout.
  #[wasm_bindgen(js_name = setRequestTimeout)]
  pub fn set_request_timeout(&mut self, value: u32) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.request_timeout(to_duration(value)))
  }

  pub(crate) fn take_builder(&mut self) -> Result<ClientBuilder, JsValue> {
    self.builder.take().ok_or_else(|| "Client Builder Consumed".into())
  }

  fn with_mut(&mut self, f: impl Fn(ClientBuilder) -> ClientBuilder) -> Result<(), JsValue> {
    self.builder = Some(f(self.take_builder()?));
    Ok(())
  }

  fn try_with_mut(&mut self, f: impl Fn(ClientBuilder) -> Result<ClientBuilder, JsValue>) -> Result<(), JsValue> {
    self.builder = Some(f(self.take_builder()?)?);
    Ok(())
  }
}

impl Default for Config {
  fn default() -> Self {
    Self::new()
  }
}
