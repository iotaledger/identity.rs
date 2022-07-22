// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::account_storage::EncryptionAlgorithm;
use wasm_bindgen::prelude::*;

/// Supported content encryption algorithms.
#[wasm_bindgen(js_name = EncryptionAlgorithm, inspectable)]
pub struct WasmEncryptionAlgorithm(pub(crate) EncryptionAlgorithm);

#[wasm_bindgen(js_class = EncryptionAlgorithm)]
impl WasmEncryptionAlgorithm {
  /// AES GCM using 256-bit key.
  #[wasm_bindgen(js_name = A256GCM)]
  pub fn aes256gcm() -> WasmEncryptionAlgorithm {
    Self(EncryptionAlgorithm::AES256GCM)
  }

  /// Returns the length of the cipher's key.
  #[wasm_bindgen(js_name = keyLength)]
  pub fn key_length(&self) -> usize {
    self.0.key_length()
  }
}

impl_wasm_json!(WasmEncryptionAlgorithm, EncryptionAlgorithm);

impl From<WasmEncryptionAlgorithm> for EncryptionAlgorithm {
  fn from(wasm_encryption_algorithm: WasmEncryptionAlgorithm) -> Self {
    wasm_encryption_algorithm.0
  }
}

impl From<EncryptionAlgorithm> for WasmEncryptionAlgorithm {
  fn from(encryption_algorithm: EncryptionAlgorithm) -> Self {
    WasmEncryptionAlgorithm(encryption_algorithm)
  }
}
