// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::crypto::KeyPair;
use identity::crypto::KeyType;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use wasm_bindgen::prelude::*;

use crate::crypto::WasmKeyType;
use crate::error::Result;
use crate::error::WasmResult;

#[derive(Deserialize, Serialize)]
struct JsonData {
  #[serde(rename = "type")]
  type_: WasmKeyType,
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
  pub fn new(type_: WasmKeyType) -> Result<WasmKeyPair> {
    KeyPair::new(type_.into()).map(Self).wasm_result()
  }

  /// Parses a `KeyPair` object from base58-encoded public/private keys.
  #[wasm_bindgen(js_name = fromBase58)]
  pub fn from_base58(type_: WasmKeyType, public_key: &str, private_key: &str) -> Result<WasmKeyPair> {
    let public: PublicKey = decode_b58(public_key).wasm_result()?.into();
    let private: PrivateKey = decode_b58(private_key).wasm_result()?.into();

    Ok(Self((type_.into(), public, private).into()))
  }

  /// Reconstructs a `KeyPair` from the bytes of a private key.
  ///
  /// The private key for `Ed25519` must be a 32-byte seed in compliance
  /// with [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-3.2).
  /// Other implementations often use another format. See [this blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) for further explanation.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = tryFromPrivateKeyBytes)]
  pub fn try_from_private_key_bytes(keyType: WasmKeyType, privateKeyBytes: &[u8]) -> Result<WasmKeyPair> {
    KeyPair::try_from_private_key_bytes(KeyType::from(keyType), privateKeyBytes)
      .map(Self)
      .wasm_result()
  }

  /// Returns a copy of the private key as a base58-encoded string.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> WasmKeyType {
    WasmKeyType::from(self.0.type_())
  }

  /// Returns a copy of the public key as a base58-encoded string.
  #[wasm_bindgen]
  pub fn public(&self) -> String {
    encode_b58(self.0.public())
  }

  /// Returns a copy of the private key as a base58-encoded string.
  #[wasm_bindgen]
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

impl From<KeyPair> for WasmKeyPair {
  fn from(key_pair: KeyPair) -> Self {
    WasmKeyPair(key_pair)
  }
}

impl_wasm_clone!(WasmKeyPair, KeyPair);
