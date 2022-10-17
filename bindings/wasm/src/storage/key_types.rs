// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_storage::Ed25519KeyType;
use identity_storage::X25519KeyType;
use wasm_bindgen::prelude::*;

/// The Ed25519 Key interoperability type.
#[wasm_bindgen(js_name = Ed25519KeyType)]
pub struct WasmEd25519KeyType;

#[wasm_bindgen]
impl WasmEd25519KeyType {
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string() -> String {
    Ed25519KeyType.to_string()
  }
}

/// The X25519 Key interoperability type.
#[wasm_bindgen(js_name = X25519KeyType)]
pub struct WasmX25519KeyType;

#[wasm_bindgen]
impl WasmX25519KeyType {
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string() -> String {
    X25519KeyType.to_string()
  }
}
