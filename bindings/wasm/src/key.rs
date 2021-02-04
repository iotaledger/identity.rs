// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::crypto::KeyPair as KeyPair_;
use identity::crypto::PublicKey;
use identity::crypto::SecretKey;
use wasm_bindgen::prelude::*;

use crate::utils::err;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(crate) enum Algorithm {
  #[serde(alias = "ed25519", alias = "ED25519")]
  Ed25519,
}

impl Algorithm {
  pub(crate) fn from_value(value: &JsValue) -> Result<Self, JsValue> {
    if value.is_falsy() {
      Ok(Self::Ed25519)
    } else {
      value.into_serde().map_err(err)
    }
  }
}

#[derive(Deserialize, Serialize)]
struct JsonData {
  algorithm: Algorithm,
  public: String,
  secret: String,
}

// =============================================================================
// =============================================================================

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug)]
pub struct KeyPair {
  pub(crate) alg: Algorithm,
  pub(crate) key: KeyPair_,
}

#[wasm_bindgen]
impl KeyPair {
  /// Generates a new `KeyPair` object.
  #[wasm_bindgen(constructor)]
  pub fn new(value: &JsValue) -> Result<KeyPair, JsValue> {
    let alg: Algorithm = Algorithm::from_value(value)?;

    let key: KeyPair_ = match alg {
      Algorithm::Ed25519 => KeyPair_::new_ed25519().map_err(err)?,
    };

    Ok(Self { alg, key })
  }

  /// Parses a `KeyPair` object from base58-encoded public/secret keys.
  #[wasm_bindgen(js_name = fromBase58)]
  pub fn from_base58(public_key: &str, secret_key: &str, value: &JsValue) -> Result<KeyPair, JsValue> {
    Self::__from_base58(public_key, secret_key, Algorithm::from_value(value)?)
  }

  /// Returns the public key as a base58-encoded string.
  #[wasm_bindgen(getter)]
  pub fn public(&self) -> String {
    encode_b58(self.key.public())
  }

  /// Returns the secret key as a base58-encoded string.
  #[wasm_bindgen(getter)]
  pub fn secret(&self) -> String {
    encode_b58(self.key.secret())
  }

  /// Serializes a `KeyPair` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    let data: JsonData = JsonData {
      algorithm: self.alg,
      public: self.public(),
      secret: self.secret(),
    };

    JsValue::from_serde(&data).map_err(err)
  }

  /// Deserializes a `KeyPair` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<KeyPair, JsValue> {
    let data: JsonData = json.into_serde().map_err(err)?;

    Self::__from_base58(&data.public, &data.secret, data.algorithm)
  }

  fn __from_base58(public: &str, secret: &str, alg: Algorithm) -> Result<KeyPair, JsValue> {
    let public: PublicKey = decode_b58(public).map_err(err)?.into();
    let secret: SecretKey = decode_b58(secret).map_err(err)?.into();

    Ok(Self {
      alg,
      key: KeyPair_::new(public, secret),
    })
  }
}
