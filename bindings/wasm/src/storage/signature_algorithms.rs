// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_storage::Ed25519SignatureAlgorithm;
use wasm_bindgen::prelude::*;

/// The Ed25519 Signature Algorithm interoperability type.
#[wasm_bindgen(js_name = Ed25519SignatureAlgorithm)]
pub struct WasmEd25519SignatureAlgorithm;

#[wasm_bindgen]
impl WasmEd25519SignatureAlgorithm {
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string() -> String {
    Ed25519SignatureAlgorithm.to_string()
  }
}
