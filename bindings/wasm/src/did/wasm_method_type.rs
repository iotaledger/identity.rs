// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::did::MethodType;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MethodType | undefined")]
  pub type OptionMethodType;
}

/// Supported verification method types.
#[wasm_bindgen(js_name = MethodType, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmMethodType(pub(crate) MethodType);

#[wasm_bindgen(js_class = MethodType)]
impl WasmMethodType {
  #[wasm_bindgen(js_name = Ed25519VerificationKey2018)]
  pub fn ed25519_verification_key_2018() -> WasmMethodType {
    WasmMethodType(MethodType::Ed25519VerificationKey2018)
  }

  #[wasm_bindgen(js_name = MerkleKeyCollection2021)]
  pub fn merkle_key_collection_2021() -> WasmMethodType {
    WasmMethodType(MethodType::MerkleKeyCollection2021)
  }

  /// Serializes a `MethodType` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `MethodType` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmMethodType> {
    json.into_serde().map(Self).wasm_result()
  }

  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl From<WasmMethodType> for MethodType {
  fn from(wasm_method_type: WasmMethodType) -> Self {
    wasm_method_type.0
  }
}

impl From<MethodType> for WasmMethodType {
  fn from(method_type: MethodType) -> Self {
    WasmMethodType(method_type)
  }
}

impl_wasm_clone!(WasmMethodType, MethodType);
