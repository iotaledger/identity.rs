// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::Network as IotaNetwork;
use wasm_bindgen::prelude::*;

use crate::error::{Result, WasmResult};

#[wasm_bindgen(js_name = Network)]
#[derive(Clone, Debug)]
pub struct WasmNetwork(IotaNetwork);

#[wasm_bindgen(js_class = Network)]
impl WasmNetwork {
  /// Parses the provided string to a [`WasmNetwork`].
  #[wasm_bindgen]
  pub fn try_from_name(name: String) -> Result<WasmNetwork> {
    IotaNetwork::try_from_name(name).map(Self).wasm_result()
  }

  #[wasm_bindgen]
  pub fn mainnet() -> WasmNetwork {
    Self(IotaNetwork::Mainnet)
  }

  #[wasm_bindgen]
  pub fn devnet() -> WasmNetwork {
    Self(IotaNetwork::Devnet)
  }

  /// Returns the node URL of the Tangle network.
  #[wasm_bindgen(getter = defaultNodeURL)]
  pub fn default_node_url(&self) -> Option<String> {
    self.0.default_node_url().map(ToString::to_string)
  }

  /// Returns the web explorer URL of the Tangle network.
  #[wasm_bindgen(getter = explorerURL)]
  pub fn explorer_url(&self) -> Option<String> {
    self.0.explorer_url().map(ToString::to_string)
  }

  /// Returns the web explorer URL of the given `message`.
  #[wasm_bindgen(js_name = messageURL)]
  pub fn message_url(&self, message_id: &str) -> Result<String> {
    self.0.message_url(message_id).map(|url| url.to_string()).wasm_result()
  }

  #[allow(clippy::inherent_to_string, clippy::wrong_self_convention)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.name_str().to_owned()
  }
}

impl Default for WasmNetwork {
  fn default() -> Self {
    IotaNetwork::default().into()
  }
}

impl From<WasmNetwork> for IotaNetwork {
  fn from(other: WasmNetwork) -> Self {
    other.0
  }
}

impl From<IotaNetwork> for WasmNetwork {
  fn from(other: IotaNetwork) -> Self {
    Self(other)
  }
}
