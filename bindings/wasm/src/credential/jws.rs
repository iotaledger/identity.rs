// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::credential::Jws;
use wasm_bindgen::prelude::*;

/// A wrapper around a JSON Web Signature (JWS).
#[wasm_bindgen(js_name = Jws)]
pub struct WasmJws(pub(crate) Jws);

#[wasm_bindgen(js_class = Jws)]
impl WasmJws {
  /// Creates a new `Jws`.
  #[wasm_bindgen(constructor)]
  pub fn new(jws_string: String) -> Self {
    Self(Jws::new(jws_string))
  }

  /// Returns a clone of the JWS string.
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}
