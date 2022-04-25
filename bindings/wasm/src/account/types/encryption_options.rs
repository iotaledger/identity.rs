// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::EncryptionOptions;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::account::types::WasmCEKAlgorithm;
use crate::account::types::WasmEncryptionAlgorithm;
use crate::error::Result;
use crate::error::WasmResult;

/// Object used for defining the algorithms used for encrypting/decrypting data
#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen(js_name = EncryptionOptions, inspectable)]
pub struct WasmEncryptionOptions(EncryptionOptions);

#[wasm_bindgen(js_class = EncryptionOptions)]
impl WasmEncryptionOptions {
  /// Creates an `EncryptionOptions` object.
  #[wasm_bindgen(js_name = new)]
  pub fn new(
    encryption_algorithm: &WasmEncryptionAlgorithm,
    cek_algorithm: &WasmCEKAlgorithm,
  ) -> WasmEncryptionOptions {
    WasmEncryptionOptions(EncryptionOptions::new(
      encryption_algorithm.clone().into(),
      cek_algorithm.clone().into(),
    ))
  }

  /// Serializes `EncryptionOptions` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `EncryptionOptions` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmEncryptionOptions> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl From<WasmEncryptionOptions> for EncryptionOptions {
  fn from(wasm_encryption_options: WasmEncryptionOptions) -> Self {
    wasm_encryption_options.0
  }
}

impl From<EncryptionOptions> for WasmEncryptionOptions {
  fn from(encryption_options: EncryptionOptions) -> Self {
    WasmEncryptionOptions(encryption_options)
  }
}
