// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

pub use super::wasm_types::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Balance>")]
  pub type PromiseBalance;
}
