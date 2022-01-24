// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

use identity::core::Fragment;

#[wasm_bindgen(js_name = Fragment, inspectable)]
pub struct WasmFragment(pub(crate) Fragment);

#[wasm_bindgen(js_class = Fragment)]
impl WasmFragment {
  #[wasm_bindgen(constructor)]
  pub fn new(value: String) -> WasmFragment {
    WasmFragment(Fragment::new(value))
  }

  /// Returns the complete fragment identifier.
  #[wasm_bindgen(getter)]
  pub fn identifier(&self) -> String {
    self.0.identifier().to_string()
  }

  /// Returns the fragment name.
  #[wasm_bindgen(getter)]
  pub fn name(&self) -> String {
    self.0.name().to_string()
  }
}

impl From<Fragment> for WasmFragment {
  fn from(fragment: Fragment) -> Self {
    WasmFragment(fragment)
  }
}

impl From<WasmFragment> for Fragment {
  fn from(wasm_fragment: WasmFragment) -> Self {
    wasm_fragment.0
  }
}
