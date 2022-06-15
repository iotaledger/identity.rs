// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::account_storage::ChainState;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = ChainState, inspectable)]
pub struct WasmChainState(pub(crate) ChainState);

#[wasm_bindgen(js_class = ChainState)]
impl WasmChainState {
  /// Serializes a `ChainState` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a JSON object as `ChainState`.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmChainState> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl From<ChainState> for WasmChainState {
  fn from(chain_state: ChainState) -> Self {
    WasmChainState(chain_state)
  }
}

impl From<WasmChainState> for ChainState {
  fn from(wasm_chain_state: WasmChainState) -> Self {
    wasm_chain_state.0
  }
}
