// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::credential::Jws;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Jws)]
pub struct WasmJws(pub(crate) Jws);

#[wasm_bindgen(js_class = Jws)]
impl WasmJws {
  #[wasm_bindgen(constructor)]
  pub fn new(jws_string: String) -> Self {
    Self(Jws::new(jws_string))
  }

  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}
