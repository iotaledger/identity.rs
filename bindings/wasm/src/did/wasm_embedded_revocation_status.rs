// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota_core::EmbeddedRevocationStatus;
use wasm_bindgen::prelude::*;

use crate::did::WasmDIDUrl;
use crate::error::Result;
use crate::error::WasmResult;

/// Information used to determine the current status of a `Credential`.
#[wasm_bindgen(js_name = EmbeddedRevocationStatus, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmEmbeddedRevocationStatus(pub(crate) EmbeddedRevocationStatus);

#[wasm_bindgen(js_class = EmbeddedRevocationStatus)]
impl WasmEmbeddedRevocationStatus {
  /// Creates a new `EmbeddedRevocationStatus`.
  #[wasm_bindgen(constructor)]
  pub fn new(id: &WasmDIDUrl, revocation_list_index: u32) -> WasmEmbeddedRevocationStatus {
    WasmEmbeddedRevocationStatus(EmbeddedRevocationStatus::new(id.clone().into(), revocation_list_index))
  }

  /// Serializes a `EmbeddedRevocationStatus` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `EmbeddedRevocationStatus` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmEmbeddedRevocationStatus> {
    value.into_serde().map(Self).wasm_result()
  }
}

impl From<EmbeddedRevocationStatus> for WasmEmbeddedRevocationStatus {
  fn from(status: EmbeddedRevocationStatus) -> Self {
    WasmEmbeddedRevocationStatus(status)
  }
}

impl From<WasmEmbeddedRevocationStatus> for EmbeddedRevocationStatus {
  fn from(status: WasmEmbeddedRevocationStatus) -> Self {
    status.0
  }
}
