// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::KeyLocation;
use wasm_bindgen::prelude::*;

use crate::account::types::WasmGeneration;
use crate::did::WasmMethodType;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = KeyLocation, inspectable)]
pub struct WasmKeyLocation(pub(crate) KeyLocation);

#[wasm_bindgen(js_class = KeyLocation)]
impl WasmKeyLocation {
  #[wasm_bindgen(constructor)]
  pub fn new(method: &WasmMethodType, fragment: String, generation: &WasmGeneration) -> WasmKeyLocation {
    WasmKeyLocation(KeyLocation::new(method.0, fragment, generation.0))
  }

  /// Returns a copy of the method type of the key location.
  #[wasm_bindgen]
  pub fn method(&self) -> WasmMethodType {
    self.0.method().into()
  }

  /// Returns a copy of the fragment name of the key location.
  #[wasm_bindgen]
  pub fn fragment(&self) -> String {
    self.0.fragment().clone().into()
  }

  /// Returns a copy of the fragment name of the key location.
  #[wasm_bindgen(js_name = fragmentName)]
  pub fn fragment_name(&self) -> String {
    self.0.fragment_name().to_string()
  }

  /// Returns a copy of the integration generation when this key was created.
  #[wasm_bindgen]
  pub fn generation(&self) -> WasmGeneration {
    self.0.generation().into()
  }

  /// Serializes `Signature` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a JSON object as `KeyLocation`.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmKeyLocation> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl From<WasmKeyLocation> for KeyLocation {
  fn from(wasm_key_location: WasmKeyLocation) -> Self {
    wasm_key_location.0
  }
}

impl From<KeyLocation> for WasmKeyLocation {
  fn from(wasm_key_location: KeyLocation) -> Self {
    WasmKeyLocation(wasm_key_location)
  }
}
