// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

use identity::account::derive_encryption_key as derive_encryption_key_;
use identity::account::EncryptionKey as EncryptionKey_;

#[wasm_bindgen(js_name = EncryptionKey, inspectable)]
pub struct WasmEncryptionKey(EncryptionKey_);

#[wasm_bindgen(js_class = EncryptionKey)]
impl WasmEncryptionKey {
  #[wasm_bindgen(js_name = deriveEncryptionKey)]
  pub fn derive_encryption_key(password: &str) -> WasmEncryptionKey {
    WasmEncryptionKey(derive_encryption_key_(password))
  }
}

impl From<EncryptionKey_> for WasmEncryptionKey {
  fn from(encryption_key: EncryptionKey_) -> WasmEncryptionKey {
    WasmEncryptionKey(encryption_key)
  }
}
