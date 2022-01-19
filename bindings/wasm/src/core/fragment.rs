// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

use identity::core::Fragment as Fragment_;

#[wasm_bindgen(js_name = Fragment, inspectable)]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct WasmFragment(pub(crate) Fragment_);

#[wasm_bindgen(js_class = Fragment)]
impl WasmFragment {
  #[wasm_bindgen(constructor)]
  pub fn new(value: String) -> WasmFragment {
    WasmFragment(Fragment_::new(value))
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

impl From<Fragment_> for WasmFragment {
  fn from(fragment: Fragment_) -> Self {
    WasmFragment(fragment)
  }
}

impl From<WasmFragment> for Fragment_ {
  fn from(wasm_fragment: WasmFragment) -> Self {
    wasm_fragment.0
  }
}
