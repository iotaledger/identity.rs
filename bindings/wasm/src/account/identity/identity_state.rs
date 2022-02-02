// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::IdentityState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = IdentityState, inspectable)]
pub struct WasmIdentityState(pub(crate) IdentityState);

impl From<IdentityState> for WasmIdentityState {
  fn from(identity_state: IdentityState) -> Self {
    WasmIdentityState(identity_state)
  }
}
