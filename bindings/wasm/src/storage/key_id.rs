// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_storage::KeyId;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = KeyId)]
#[derive(Debug, Clone)]
pub struct WasmKeyId(KeyId);

#[wasm_bindgen(js_class = KeyId)]
impl WasmKeyId {
  #[wasm_bindgen(constructor)]
  pub fn new(alias: String) -> WasmKeyId {
    WasmKeyId(KeyId::new(alias))
  }

  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl_wasm_json!(WasmKeyId, KeyId);

impl From<WasmKeyId> for KeyId {
  fn from(alias: WasmKeyId) -> Self {
    alias.0
  }
}

impl From<KeyId> for WasmKeyId {
  fn from(alias: KeyId) -> Self {
    WasmKeyId(alias)
  }
}
