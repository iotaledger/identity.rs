// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<void>")]
  pub type PromiseVoid;

  #[wasm_bindgen(typescript_type = "Array<string>")]
  pub type ArrayString;

  #[wasm_bindgen(typescript_type = "Map<string, any>")]
  pub type MapStringAny;
}
