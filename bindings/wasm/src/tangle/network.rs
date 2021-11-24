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

  /// Returns the web identity resolver URL for the given DID.
  #[wasm_bindgen(js_name = resolverURL)]
  pub fn resolver_url(&self, did: &str) -> Result<String> {
    self.0.resolver_url(did).map(|url| url.to_string()).wasm_result()
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
