// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::credential::Jwt;
use wasm_bindgen::prelude::*;

/// A wrapper around a JSON Web Token (JWK).
#[wasm_bindgen(js_name = Jwt)]
pub struct WasmJwt(pub(crate) Jwt);

#[wasm_bindgen(js_class = Jwt)]
impl WasmJwt {
  /// Creates a new `Jwt`.
  #[wasm_bindgen(constructor)]
  pub fn new(jwt_string: String) -> Self {
    Self(Jwt::new(jwt_string))
  }

  /// Returns a clone of the JWT string.
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}
