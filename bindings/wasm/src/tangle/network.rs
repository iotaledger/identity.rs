// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::Network as IotaNetwork;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Network)]
#[derive(Clone, Copy, Debug)]
pub struct WasmNetwork(IotaNetwork);

#[wasm_bindgen(js_class = Network)]
impl WasmNetwork {
  #[wasm_bindgen]
  pub fn mainnet() -> WasmNetwork {
    Self(IotaNetwork::Mainnet)
  }

  #[wasm_bindgen]
  pub fn testnet() -> WasmNetwork {
    Self(IotaNetwork::Testnet)
  }

  #[wasm_bindgen(getter = defaultNodeURL)]
  pub fn default_node_url(&self) -> String {
    self.0.node_url().to_string()
  }

  #[wasm_bindgen(getter = explorerURL)]
  pub fn explorer_url(&self) -> String {
    self.0.explorer_url().to_string()
  }

  #[allow(clippy::inherent_to_string, clippy::wrong_self_convention)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.as_str().into()
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
