// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::EncryptedData;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// The structure returned after encrypting data
#[wasm_bindgen(js_name = EncryptedData, inspectable)]
pub struct WasmEncryptedData(pub(crate) EncryptedData);

#[wasm_bindgen(js_class = EncryptedData)]
impl WasmEncryptedData {
  /// Returns a copy of the nonce
  #[wasm_bindgen(js_name = nonce)]
  pub fn nonce(&self) -> Vec<u8> {
    self.0.nonce().to_vec()
  }

  /// Returns a copy of the associated data
  #[wasm_bindgen(js_name = associatedData)]
  pub fn associated_data(&self) -> Vec<u8> {
    self.0.associated_data().to_vec()
  }

  /// Returns a copy of the ciphertext
  #[wasm_bindgen(js_name = ciphertext)]
  pub fn ciphertext(&self) -> Vec<u8> {
    self.0.ciphertext().to_vec()
  }

  /// Returns a copy of the tag
  #[wasm_bindgen(js_name = tag)]
  pub fn tag(&self) -> Vec<u8> {
    self.0.tag().to_vec()
  }

  /// Serializes `EncryptedData` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `EncryptedData` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmEncryptedData> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl From<WasmEncryptedData> for EncryptedData {
  fn from(wasm_encrypted_data: WasmEncryptedData) -> Self {
    wasm_encrypted_data.0
  }
}

impl From<EncryptedData> for WasmEncryptedData {
  fn from(encrypted_data: EncryptedData) -> Self {
    WasmEncryptedData(encrypted_data)
  }
}
