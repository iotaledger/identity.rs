// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::crypto::KeyPair;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use wasm_bindgen::prelude::*;

use crate::crypto::KeyType;
use crate::error::Result;
use crate::error::WasmResult;

#[derive(Deserialize, Serialize)]
struct JsonData {
  #[serde(rename = "type")]
  type_: KeyType,
  public: String,
  private: String,
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

  /// Parses a `KeyPair` object from base58-encoded public/private keys.
  #[wasm_bindgen(js_name = fromBase58)]
  pub fn from_base58(type_: KeyType, public_key: &str, private_key: &str) -> Result<WasmKeyPair> {
    let public: PublicKey = decode_b58(public_key).wasm_result()?.into();
    let private: PrivateKey = decode_b58(private_key).wasm_result()?.into();

    Ok(Self((type_.into(), public, private).into()))
  }

  /// Returns the private key as a base58-encoded string.
  #[wasm_bindgen(getter = type)]
  pub fn type_(&self) -> KeyType {
    KeyType::from(self.0.type_())
  }

  /// Returns the public key as a base58-encoded string.
  #[wasm_bindgen(getter)]
  pub fn public(&self) -> String {
    encode_b58(self.0.public())
  }

  /// Returns the private key as a base58-encoded string.
  #[wasm_bindgen(getter)]
  pub fn private(&self) -> String {
    encode_b58(self.0.private())
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

    Self::from_base58(data.type_, &data.public, &data.private)
  }
}

impl_wasm_clone!(WasmKeyPair, KeyPair);
