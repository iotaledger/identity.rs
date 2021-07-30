// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::ClientBuilder;
use std::time::Duration;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;
use crate::tangle::WasmNetwork;

fn to_duration(seconds: u32) -> Duration {
  Duration::from_secs(u64::from(seconds))
}

fn to_basic_auth<'a>(username: &'a Option<String>, password: &'a Option<String>) -> Option<(&'a str, &'a str)> {
  username.as_deref().zip(password.as_deref())
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Config {
  pub(crate) builder: Option<ClientBuilder>,
}

#[wasm_bindgen]
impl Config {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      builder: Some(ClientBuilder::new()),
    }
  }

  #[wasm_bindgen(js_name = fromNetwork)]
  pub fn from_network(network: &WasmNetwork) -> Result<Config, JsValue> {
    let mut this: Self = Self::new();
    this.set_network(network)?;
    Ok(this)
  }

  #[wasm_bindgen(js_name = setNetwork)]
  pub fn set_network(&mut self, network: &WasmNetwork) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.network((*network).into()))
  }

  #[wasm_bindgen(js_name = setNode)]
  pub fn set_node(&mut self, url: &str) -> Result<(), JsValue> {
    self.try_with_mut(|builder| builder.node(url).map_err(wasm_error))
  }

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

  #[wasm_bindgen(js_name = setNodeSyncInterval)]
  pub fn set_node_sync_interval(&mut self, value: u32) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.node_sync_interval(to_duration(value)))
  }

  #[wasm_bindgen(js_name = setNodeSyncDisabled)]
  pub fn set_node_sync_disabled(&mut self) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.node_sync_disabled())
  }

  #[wasm_bindgen(js_name = setQuorum)]
  pub fn set_quorum(&mut self, value: bool) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.quorum(value))
  }

  #[wasm_bindgen(js_name = setQuorumSize)]
  pub fn set_quorum_size(&mut self, value: usize) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.quorum_size(value))
  }

  #[wasm_bindgen(js_name = setQuorumThreshold)]
  pub fn set_quorum_threshold(&mut self, value: usize) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.quorum_threshold(value))
  }

  #[wasm_bindgen(js_name = setLocalPoW)]
  pub fn set_local_pow(&mut self, value: bool) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.local_pow(value))
  }

  #[wasm_bindgen(js_name = setTipsInterval)]
  pub fn set_tips_interval(&mut self, value: u32) -> Result<(), JsValue> {
    self.with_mut(|builder| builder.tips_interval(u64::from(value)))
  }

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
