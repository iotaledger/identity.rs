// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::CEKAlgorithm;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Supported CEK algorithms
#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen(js_name = CEKAlgorithm, inspectable)]
pub struct WasmCEKAlgorithm(CEKAlgorithm);

#[wasm_bindgen(js_class = CEKAlgorithm)]
impl WasmCEKAlgorithm {
  /// ECDH-ES will be used as the content encryption key.
  #[wasm_bindgen(js_name = ecdhES)]
  pub fn ecdh_es() -> WasmCEKAlgorithm {
    Self(CEKAlgorithm::ECDH_ES)
  }

  /// Serializes `CEKAlgorithm` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `CEKAlgorithm` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmCEKAlgorithm> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl From<WasmCEKAlgorithm> for CEKAlgorithm {
  fn from(wasm_cek_algorithm: WasmCEKAlgorithm) -> Self {
    wasm_cek_algorithm.0
  }
}

impl From<CEKAlgorithm> for WasmCEKAlgorithm {
  fn from(cek_algorithm: CEKAlgorithm) -> Self {
    WasmCEKAlgorithm(cek_algorithm)
  }
}
