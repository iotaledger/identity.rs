// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::DIDLease;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = DIDLease, inspectable)]
pub struct WasmDIDLease(pub(crate) DIDLease);

#[wasm_bindgen(js_class = DIDLease)]
impl WasmDIDLease {
  #[wasm_bindgen(constructor)]
  pub fn new() -> WasmDIDLease {
    WasmDIDLease(DIDLease::new())
  }

  #[wasm_bindgen]
  pub fn store(&self, value: bool) {
    self.0.store(value);
  }

  #[wasm_bindgen]
  pub fn load(&self) -> bool {
    self.0.load()
  }
}

impl Default for WasmDIDLease {
  fn default() -> Self {
    Self::new()
  }
}

impl From<DIDLease> for WasmDIDLease {
  fn from(did_lease: DIDLease) -> Self {
    WasmDIDLease(did_lease)
  }
}

impl From<WasmDIDLease> for DIDLease {
  fn from(wasm_did_lease: WasmDIDLease) -> Self {
    wasm_did_lease.0
  }
}
