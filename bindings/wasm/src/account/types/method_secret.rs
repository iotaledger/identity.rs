// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;
use identity::account::MethodSecret;
use identity::core::decode_b58;
use identity::crypto::PrivateKey;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::crypto::WasmKeyCollection;
use crate::crypto::WasmKeyCollectionData;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MethodSecret | undefined")]
  pub type OptionMethodSecret;
}

/// Workaround for being unable to use serde for private keys.
#[derive(Serialize, Deserialize)]
enum WasmMethodSecretInner {
  Ed25519(String),
  MerkleKeyCollection(WasmKeyCollectionData),
}

#[wasm_bindgen(js_name = MethodSecret)]
#[derive(Serialize, Deserialize)]
pub struct WasmMethodSecret(WasmMethodSecretInner);

#[wasm_bindgen(js_class = MethodSecret)]
impl WasmMethodSecret {
  /// Creates a {@link MethodSecret} object from a Base58-BTC encoded Ed25519 private key.
  #[wasm_bindgen(js_name = ed25519Base58)]
  pub fn ed25519_base58(private_key: String) -> WasmMethodSecret {
    Self(WasmMethodSecretInner::Ed25519(private_key))
  }

  /// Creates a {@link MethodSecret} object from {@link KeyCollection}.
  #[wasm_bindgen(js_name = merkleKeyCollection)]
  pub fn merkle_key_collection(collection: &WasmKeyCollection) -> WasmMethodSecret {
    Self(WasmMethodSecretInner::MerkleKeyCollection(WasmKeyCollectionData::from(collection)))
  }

  /// Serializes a `MethodSecret` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `MethodSecret` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmMethodSecret> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl TryFrom<WasmMethodSecret> for MethodSecret {
  type Error = JsValue;

  fn try_from(value: WasmMethodSecret) -> std::result::Result<Self, Self::Error> {
    match value.0 {
      WasmMethodSecretInner::Ed25519(encoded) => {
        let private: PrivateKey = decode_b58(&encoded).wasm_result()?.into();
        if private.as_ref().len() != ed25519::SECRET_KEY_LENGTH {
          return Err(identity::core::Error::InvalidKeyLength(private.as_ref().len(), ed25519::SECRET_KEY_LENGTH)).wasm_result();
        };
        Ok(MethodSecret::Ed25519(private))
      }
      WasmMethodSecretInner::MerkleKeyCollection(data) => {
        let collection: WasmKeyCollection = WasmKeyCollection::try_from(data)?;
        Ok(MethodSecret::MerkleKeyCollection(collection.0))
      }
    }
  }
}
