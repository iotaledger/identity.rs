// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::AutoSave;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "AutoSave | undefined")]
  pub type OptionAutoSave;
}

#[wasm_bindgen(js_name = AutoSave, inspectable)]
pub struct WasmAutoSave(pub(crate) AutoSave);

/// Available auto-save behaviours.
#[wasm_bindgen(js_class = AutoSave)]
impl WasmAutoSave {
  /// Never save.
  #[wasm_bindgen]
  pub fn never() -> WasmAutoSave {
    Self(AutoSave::Never)
  }

  /// Save after every action.
  #[wasm_bindgen]
  pub fn every() -> WasmAutoSave {
    Self(AutoSave::Every)
  }

  /// Save after every N actions.
  #[wasm_bindgen]
  pub fn batch(number_of_actions: usize) -> WasmAutoSave {
    Self(AutoSave::Batch(number_of_actions))
  }

  /// Serializes `AutoSave` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `AutoSave` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmAutoSave> {
    json_value.into_serde().map(Self).wasm_result()
  }
}
