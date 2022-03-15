// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::Signature;
use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::crypto::PublicKey;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Signature, inspectable)]
pub struct WasmSignature(pub(crate) Signature);

#[wasm_bindgen(js_class = Signature)]
impl WasmSignature {
  #[wasm_bindgen(constructor)]
  /// Creates a new `Signature`.
  pub fn new(pkey: &str, data: Vec<u8>) -> Result<WasmSignature> {
    let public_key: PublicKey = decode_b58(pkey).map_err(wasm_error)?.into();
    Ok(WasmSignature(Signature::new(public_key, data)))
  }

  #[wasm_bindgen(getter)]
  /// Returns the public key, encoded as a base58 string, used to verify this signature.
  pub fn pkey(&self) -> String {
    encode_b58(self.0.pkey())
  }

  #[wasm_bindgen(getter)]
  /// Returns the signature data as a vec of bytes.
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
