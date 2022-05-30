// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota_core::EmbeddedRevocationEndpoint;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// A parsed data url.
#[wasm_bindgen(js_name = EmbeddedRevocationEndpoint, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmEmbeddedRevocationEndpoint(pub(crate) EmbeddedRevocationEndpoint);

#[wasm_bindgen(js_class = EmbeddedRevocationEndpoint)]
impl WasmEmbeddedRevocationEndpoint {
  /// Parses an [`EmbeddedRevocationEndpoint`] from the given input String.
  #[wasm_bindgen]
  pub fn parse(input: String) -> Result<WasmEmbeddedRevocationEndpoint> {
    Ok(WasmEmbeddedRevocationEndpoint(
      EmbeddedRevocationEndpoint::parse(&input).wasm_result()?,
    ))
  }

  /// Returns the `EmbeddedRevocationEndpoint` as a String.
  #[wasm_bindgen]
  pub fn into_string(&self) -> String {
    self.0.clone().into_string()
  }

  /// Returns the data from the [`EmbeddedRevocationEndpoint`].
  #[wasm_bindgen]
  pub fn data(&self) -> String {
    self.0.data().to_owned()
  }

  /// Serializes a `EmbeddedRevocationEndpoint` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `EmbeddedRevocationEndpoint` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmEmbeddedRevocationEndpoint> {
    value.into_serde().map(Self).wasm_result()
  }
}

impl From<EmbeddedRevocationEndpoint> for WasmEmbeddedRevocationEndpoint {
  fn from(endpoint: EmbeddedRevocationEndpoint) -> Self {
    WasmEmbeddedRevocationEndpoint(endpoint)
  }
}

impl From<WasmEmbeddedRevocationEndpoint> for EmbeddedRevocationEndpoint {
  fn from(endpoint: WasmEmbeddedRevocationEndpoint) -> Self {
    endpoint.0
  }
}
