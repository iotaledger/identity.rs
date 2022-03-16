// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::IdentityState;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = IdentityState, inspectable)]
pub struct WasmIdentityState(pub(crate) IdentityState);

#[wasm_bindgen(js_class = IdentityState)]
impl WasmIdentityState {
  /// Serializes a `IdentityState` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a JSON object as `IdentityState`.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmIdentityState> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl From<IdentityState> for WasmIdentityState {
  fn from(identity_state: IdentityState) -> Self {
    WasmIdentityState(identity_state)
  }
}

impl From<WasmIdentityState> for IdentityState {
  fn from(wasm_identity_state: WasmIdentityState) -> Self {
    wasm_identity_state.0
  }
}
