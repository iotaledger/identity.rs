// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_storage::KeyAlias;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = KeyAlias)]
#[derive(Debug, Clone)]
pub struct WasmKeyAlias(KeyAlias);

#[wasm_bindgen(js_class = KeyAlias)]
impl WasmKeyAlias {
  #[wasm_bindgen(constructor)]
  pub fn new(alias: String) -> WasmKeyAlias {
    WasmKeyAlias(KeyAlias::new(alias))
  }

  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl_wasm_json!(WasmKeyAlias, KeyAlias);

impl From<WasmKeyAlias> for KeyAlias {
  fn from(alias: WasmKeyAlias) -> Self {
    alias.0
  }
}

impl From<KeyAlias> for WasmKeyAlias {
  fn from(alias: KeyAlias) -> Self {
    WasmKeyAlias(alias)
  }
}
