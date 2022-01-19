// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

use identity::account::IdentityState as IdentityState_;

use crate::account::WasmGeneration;
use crate::account::WasmKeyLocation;
use crate::core::WasmFragment;
use crate::did::WasmDocument;
use crate::did::WasmMethodType;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = IdentityState, inspectable)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WasmIdentityState(pub(crate) IdentityState_);

#[wasm_bindgen(js_class = IdentityState)]
impl WasmIdentityState {
  #[wasm_bindgen(constructor)]
  pub fn new(document: WasmDocument) -> WasmIdentityState {
    WasmIdentityState(IdentityState_::new(document.into()))
  }

  // ===========================================================================
  // Internal State
  // ===========================================================================

  /// Returns the current generation of the identity integration chain.
  #[wasm_bindgen(getter)]
  pub fn generation(&self) -> WasmGeneration {
    self.0.generation().into()
  }

  /// Increments the generation of the identity diff chain.
  #[wasm_bindgen(js_name = incrementGeneration)]
  pub fn increment_generation(&mut self) -> Result<()> {
    self.0.increment_generation().wasm_result()
  }

  /// Stores the generations at which the method was inserted.
  #[wasm_bindgen(js_name = storeMethodGenerations)]
  pub fn store_method_generations(&mut self, fragment: WasmFragment) {
    self.0.store_method_generations(fragment.into())
  }

  /// Return the `KeyLocation` of the given method.
  #[wasm_bindgen(getter = methodLocation)]
  pub fn method_location(&self, method_type: WasmMethodType, fragment: String) -> Result<WasmKeyLocation> {
    self
      .0
      .method_location(method_type.into(), fragment)
      .map(|key_location| key_location.into())
      .wasm_result()
  }

  // ===========================================================================
  // Document State
  // ===========================================================================

  #[wasm_bindgen(getter)]
  pub fn document(&self) -> WasmDocument {
    self.0.document().clone().into()
  }

  /// Returns a key location suitable for the specified `fragment`.
  #[wasm_bindgen(js_name = keyLocation)]
  pub fn key_location(&self, method: WasmMethodType, fragment: String) -> Result<WasmKeyLocation> {
    self
      .0
      .key_location(method.into(), fragment)
      .map(|key_location| key_location.into())
      .wasm_result()
  }
}

impl From<IdentityState_> for WasmIdentityState {
  fn from(identity_state: IdentityState_) -> Self {
    WasmIdentityState(identity_state)
  }
}
