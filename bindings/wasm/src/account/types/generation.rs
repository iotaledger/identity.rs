// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::Generation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Generation)]
pub struct WasmGeneration(pub(crate) Generation);

#[allow(clippy::new_without_default)]
#[wasm_bindgen(js_class = Generation)]
impl WasmGeneration {
  /// Creates a new `WasmGeneration`.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    WasmGeneration(Generation::new())
  }

  /// Creates a new `WasmGeneration` from a 32-bit integer.
  #[wasm_bindgen(js_name = fromUnsignedInteger)]
  pub fn from_u32(value: u32) -> Self {
    WasmGeneration(Generation::from_u32(value))
  }

  /// Returns the `WasmGeneration` as a 32-bit integer.
  #[wasm_bindgen(js_name = toUnsignedInteger)]
  pub fn to_u32(self) -> u32 {
    self.0.to_u32()
  }
}

impl From<Generation> for WasmGeneration {
  fn from(generation: Generation) -> Self {
    Self(generation)
  }
}

impl From<WasmGeneration> for Generation {
  fn from(wasm_generation: WasmGeneration) -> Self {
    wasm_generation.0
  }
}
