// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod deactivate_did;
mod update_did;

pub use deactivate_did::*;
pub use update_did::*;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Set<string>")]
  pub type StringSet;
}