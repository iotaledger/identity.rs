// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::crypto::KeyPair as KeyPair_;
use identity::crypto::KeyType;
use identity::crypto::PublicKey;
use identity::crypto::SecretKey;
use wasm_bindgen::prelude::*;

use crate::utils::err;

#[derive(Deserialize, Serialize)]
struct JsonData {
  #[serde(rename = "type")]
  type_: KeyType,
  public: String,
  secret: String,
}

// =============================================================================
// =============================================================================

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug)]
pub struct KeyPair(pub(crate) KeyPair_);

#[wasm_bindgen]
impl KeyPair {
  pub(crate) fn parse_key_type(value: &str) -> Result<KeyType, JsValue> {
    if value.is_empty() {
      Ok(KeyType::Ed25519)
    } else {
      value.parse().map_err(err)
    }
  }

  /// Generates a new `KeyPair` object.
  #[wasm_bindgen(constructor)]
  pub fn new(value: &str) -> Result<KeyPair, JsValue> {
    Self::parse_key_type(value)
      .and_then(|type_| KeyPair_::new(type_).map_err(err))
      .map(Self)
  }

  /// Parses a `KeyPair` object from base58-encoded public/secret keys.
  #[wasm_bindgen(js_name = fromBase58)]
  pub fn from_base58(public_key: &str, secret_key: &str, value: &str) -> Result<KeyPair, JsValue> {
    Self::parse_key_type(value).and_then(|type_| Self::__from_base58(type_, public_key, secret_key))
  }

  /// Returns the public key as a base58-encoded string.
  #[wasm_bindgen(getter)]
  pub fn public(&self) -> String {
    encode_b58(self.0.public())
  }

  /// Returns the secret key as a base58-encoded string.
  #[wasm_bindgen(getter)]
  pub fn secret(&self) -> String {
    encode_b58(self.0.secret())
  }

  /// Serializes a `KeyPair` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    let data: JsonData = JsonData {
      type_: self.0.type_(),
      public: self.public(),
      secret: self.secret(),
    };

    JsValue::from_serde(&data).map_err(err)
  }

  /// Deserializes a `KeyPair` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<KeyPair, JsValue> {
    let data: JsonData = json.into_serde().map_err(err)?;

    Self::__from_base58(data.type_, &data.public, &data.secret)
  }

  fn __from_base58(type_: KeyType, public: &str, secret: &str) -> Result<KeyPair, JsValue> {
    let public: PublicKey = decode_b58(public).map_err(err)?.into();
    let secret: SecretKey = decode_b58(secret).map_err(err)?.into();

    Ok(Self((type_, public, secret).into()))
  }
}
