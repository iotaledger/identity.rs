// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::Signature;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Signature, inspectable)]
pub struct WasmSignature(pub(crate) Signature);

#[wasm_bindgen(js_class = Signature)]
impl WasmSignature {
  #[wasm_bindgen(constructor)]
  /// Creates a new `Signature`.
  pub fn new(pkey: Vec<u8>, data: Vec<u8>) -> Result<WasmSignature> {
    Ok(WasmSignature(Signature::new(pkey.into(), data)))
  }

  #[wasm_bindgen]
  /// Returns a copy of the public key used to verify this signature.
  pub fn pkey(&self) -> Vec<u8> {
    self.0.pkey().as_ref().to_vec()
  }

  #[wasm_bindgen]
  /// Returns a copy of the signature data as a vec of bytes.
  pub fn data(&self) -> Vec<u8> {
    self.0.data().to_vec()
  }

  /// Serializes a `Signature` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a JSON object as `Signature`.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmSignature> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl From<WasmSignature> for Signature {
  fn from(wasm_signature: WasmSignature) -> Self {
    wasm_signature.0
  }
}
