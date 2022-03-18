// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::crypto::KeyPair;
use wasm_bindgen::prelude::*;

use crate::crypto::KeyType;
use crate::error::Result;
use crate::error::WasmResult;

#[derive(Deserialize, Serialize)]
struct JsonData {
  #[serde(rename = "type")]
  type_: KeyType,
  public: Vec<u8>,
  private: Vec<u8>,
}

// =============================================================================
// =============================================================================

#[wasm_bindgen(inspectable, js_name = KeyPair)]
#[derive(Clone, Debug)]
pub struct WasmKeyPair(pub(crate) KeyPair);

#[wasm_bindgen(js_class = KeyPair)]
impl WasmKeyPair {
  /// Generates a new `KeyPair` object.
  #[wasm_bindgen(constructor)]
  pub fn new(type_: KeyType) -> Result<WasmKeyPair> {
    KeyPair::new(type_.into()).map(Self).wasm_result()
  }

  /// Parses a `KeyPair` object from the public/private keys.
  #[wasm_bindgen(js_name = fromBase58)]
  pub fn from_keys(type_: KeyType, public_key: Vec<u8>, private_key: Vec<u8>) -> Result<WasmKeyPair> {
    Ok(Self((type_.into(), public_key.into(), private_key.into()).into()))
  }

  /// Returns the `KeyType` of the `KeyPair` object.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> KeyType {
    KeyType::from(self.0.type_())
  }

  /// Returns a copy of the public key as a `UInt8Array`.
  #[wasm_bindgen]
  pub fn public(&self) -> Vec<u8> {
    self.0.public().into()
  }

  /// Returns a copy of the private key as a `UInt8Array`.
  #[wasm_bindgen]
  pub fn private(&self) -> Vec<u8> {
    self.0.private().into()
  }

  /// Serializes a `KeyPair` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    let data: JsonData = JsonData {
      type_: self.0.type_().into(),
      public: self.public(),
      private: self.private(),
    };

    JsValue::from_serde(&data).wasm_result()
  }

  /// Deserializes a `KeyPair` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmKeyPair> {
    let data: JsonData = json.into_serde().wasm_result()?;
    Ok(data.into())
  }
}

impl From<JsonData> for WasmKeyPair {
  fn from(json_data: JsonData) -> Self {
    Self(
      (
        json_data.type_.into(),
        json_data.public.into(),
        json_data.private.into(),
      )
        .into(),
    )
  }
}

impl From<KeyPair> for WasmKeyPair {
  fn from(key_pair: KeyPair) -> Self {
    WasmKeyPair(key_pair)
  }
}

impl_wasm_clone!(WasmKeyPair, KeyPair);
