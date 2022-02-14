// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::IdentityState;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = IdentityState, inspectable)]
#[derive(Serialize, Deserialize)]
pub struct WasmIdentityState(pub(crate) IdentityState);

#[wasm_bindgen(js_class = IdentityState)]
impl WasmIdentityState {
  /// Serializes a `IdentityState` as `Uint8Array`.
  #[wasm_bindgen(js_name = asBytes)]
  pub fn as_bytes(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).wasm_result()
  }

  /// Deserializes a `Uint8Array` as `IdentityState`.
  #[wasm_bindgen(js_name = fromBytes)]
  pub fn from_bytes(bytes: Vec<u8>) -> Result<WasmIdentityState> {
    bincode::deserialize(&bytes).wasm_result()
  }
}

impl From<IdentityState> for WasmIdentityState {
  fn from(identity_state: IdentityState) -> Self {
    WasmIdentityState(identity_state)
  }
}
