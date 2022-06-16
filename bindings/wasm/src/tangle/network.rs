// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota_core::Network;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Network)]
pub struct WasmNetwork(pub(crate) Network);

#[wasm_bindgen(js_class = Network)]
impl WasmNetwork {
  /// Parses the provided string to a `Network`.
  ///
  /// Errors if the name is invalid.
  #[wasm_bindgen(js_name = tryFromName)]
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

  /// Returns a copy of the network name.
  #[wasm_bindgen]
  pub fn name(&self) -> String {
    self.0.name_str().to_owned()
  }

  /// Returns a copy of the node URL of the Tangle network.
  #[wasm_bindgen(js_name = defaultNodeURL)]
  pub fn default_node_url(&self) -> Option<String> {
    self.0.default_node_url().map(ToString::to_string)
  }

  #[allow(clippy::inherent_to_string, clippy::wrong_self_convention)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.name_str().to_owned()
  }

  /// Serializes a `Network` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Network` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmNetwork> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmNetwork, Network);

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
