// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use wasm_bindgen::prelude::*;

// Workaround for Typescript type annotations on async functions that don't return anything.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<void>")]
  pub type PromiseVoid;
}
