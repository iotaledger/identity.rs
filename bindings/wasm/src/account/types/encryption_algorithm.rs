// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::account_storage::EncryptionAlgorithm;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Supported content encryption algorithms.
#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen(js_name = EncryptionAlgorithm, inspectable)]
pub struct WasmEncryptionAlgorithm(EncryptionAlgorithm);

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

  /// Serializes `EncryptionAlgorithm` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `EncryptionAlgorithm` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmEncryptionAlgorithm> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

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
