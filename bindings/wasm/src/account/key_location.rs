// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

use identity::account::KeyLocation as KeyLocation_;

use crate::account::WasmGeneration;
use crate::core::WasmFragment;
use crate::did::WasmMethodType;

#[wasm_bindgen(js_name = KeyLocation, inspectable)]
#[derive(Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct WasmKeyLocation(pub(crate) KeyLocation_);

#[wasm_bindgen(js_class = KeyLocation)]
impl WasmKeyLocation {
  #[wasm_bindgen(constructor)]
  pub fn new(method: WasmMethodType, fragment: String, generation: WasmGeneration) -> WasmKeyLocation {
    WasmKeyLocation(KeyLocation_::new(method.into(), fragment, generation.into()))
  }

  /// Returns the method type of the key location.
  #[wasm_bindgen(getter)]
  pub fn method(&self) -> WasmMethodType {
    self.0.method().into()
  }

  /// Returns the fragment name of the key location.
  #[wasm_bindgen(getter)]
  pub fn fragment(&self) -> WasmFragment {
    self.0.fragment().clone().into()
  }

  /// Returns the fragment name of the key location.
  #[wasm_bindgen(getter = fragmentName)]
  pub fn fragment_name(&self) -> String {
    self.0.fragment_name().to_string()
  }

  /// Returns the integration generation when this key was created.
  #[wasm_bindgen(getter)]
  pub fn generation(&self) -> WasmGeneration {
    self.0.generation().into()
  }
}

impl From<WasmKeyLocation> for KeyLocation_ {
  fn from(wasm_key_location: WasmKeyLocation) -> Self {
    wasm_key_location.0
  }
}

impl From<KeyLocation_> for WasmKeyLocation {
  fn from(wasm_key_location: KeyLocation_) -> Self {
    WasmKeyLocation(wasm_key_location)
  }
}
