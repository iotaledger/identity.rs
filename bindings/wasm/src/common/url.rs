// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::error::WasmResult;
use identity::core::Url;
use wasm_bindgen::prelude::*;

/// A parsed URL.
#[wasm_bindgen(js_name = Url, inspectable)]
pub struct WasmUrl(pub(crate) Url);

#[wasm_bindgen(js_class = Url)]
impl WasmUrl {
  /// Parses an absolute `Url` from the given input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmUrl> {
    Ok(WasmUrl(Url::parse(input).wasm_result()?))
  }

  /// Parses the given input string as a `Url`, with `self` as the base Url.
  #[wasm_bindgen]
  pub fn join(&self, input: &str) -> Result<WasmUrl> {
    Ok(WasmUrl(self.0.join(input).wasm_result()?))
  }

  /// Serializes a `Url` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Url` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmUrl> {
    json.into_serde().map(Self).wasm_result()
  }
}
