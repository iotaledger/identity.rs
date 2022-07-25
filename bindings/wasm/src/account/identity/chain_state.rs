// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::account_storage::ChainState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ChainState, inspectable)]
pub struct WasmChainState(pub(crate) ChainState);

impl_wasm_json!(WasmChainState, ChainState);
impl_wasm_clone!(WasmChainState, ChainState);

impl From<ChainState> for WasmChainState {
  fn from(chain_state: ChainState) -> Self {
    WasmChainState(chain_state)
  }
}

impl From<WasmChainState> for ChainState {
  fn from(wasm_chain_state: WasmChainState) -> Self {
    wasm_chain_state.0
  }
}
