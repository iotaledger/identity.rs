// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::Network;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Network)]
#[derive(Clone, Debug)]
pub struct WasmNetwork(Network);

#[wasm_bindgen(js_class = Network)]
impl WasmNetwork {
  /// Parses the provided string to a `Network`.
  #[wasm_bindgen]
  pub fn try_from_name(name: String) -> Result<WasmNetwork> {
    Network::try_from_name(name).map(Self).wasm_result()
  }

  #[wasm_bindgen]
  pub fn mainnet() -> WasmNetwork {
    Self(Network::Mainnet)
  }

  #[wasm_bindgen]
  pub fn devnet() -> WasmNetwork {
    Self(Network::Devnet)
  }

  /// Returns the node URL of the Tangle network.
  #[wasm_bindgen(getter = defaultNodeURL)]
  pub fn default_node_url(&self) -> Option<String> {
    self.0.default_node_url().map(ToString::to_string)
  }

  #[allow(clippy::inherent_to_string, clippy::wrong_self_convention)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.name_str().to_owned()
  }
}

impl Default for WasmNetwork {
  fn default() -> Self {
    Network::default().into()
  }
}

impl From<WasmNetwork> for Network {
  fn from(other: WasmNetwork) -> Self {
    other.0
  }
}

impl From<Network> for WasmNetwork {
  fn from(other: Network) -> Self {
    Self(other)
  }
}
