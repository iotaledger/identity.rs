// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::did::MethodData;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Supported verification method types.
#[wasm_bindgen(js_name = MethodData, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmMethodData(pub(crate) MethodData);

#[wasm_bindgen(js_class = MethodData)]
impl WasmMethodData {
  /// Creates a new `MethodData` variant with Base58-BTC encoded content.
  #[wasm_bindgen(js_name = newBase58)]
  pub fn new_base58(data: Vec<u8>) -> Self {
    Self(MethodData::new_base58(data))
  }

  /// Creates a new `MethodData` variant with Multibase-encoded content.
  #[wasm_bindgen(js_name = newMultibase)]
  pub fn new_multibase(data: Vec<u8>) -> Self {
    Self(MethodData::new_multibase(data))
  }

  /// Returns a `UInt8Array` containing the decoded bytes of the `MethodData`.
  ///
  /// This is generally a public key identified by a `MethodData` value.
  ///
  /// ### Errors
  /// Decoding can fail if `MethodData` has invalid content or cannot be
  /// represented as a vector of bytes.
  #[wasm_bindgen(js_name = tryDecode)]
  pub fn try_decode(&self) -> Result<Vec<u8>> {
    self.0.try_decode().wasm_result()
  }

  /// Serializes a `MethodData` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `MethodData` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmMethodData> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl From<WasmMethodData> for MethodData {
  fn from(data: WasmMethodData) -> Self {
    data.0
  }
}

impl From<MethodData> for WasmMethodData {
  fn from(data: MethodData) -> Self {
    WasmMethodData(data)
  }
}

impl_wasm_clone!(WasmMethodData, MethodData);
