// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::EncryptionKey;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

// Workaround for having to deserialize `WasmEncryptionKey` from a Typescript interface
// TODO: remove when https://github.com/rustwasm/wasm-bindgen/pull/2677 is merged.
#[derive(Clone, Serialize, Deserialize)]
enum WasmEncryptionKeyInner {
  Ed25519,
  X25519(Vec<u8>),
}

#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen(js_name = EncryptionKey, inspectable)]
pub struct WasmEncryptionKey(WasmEncryptionKeyInner);

#[wasm_bindgen(js_class = EncryptionKey)]
impl WasmEncryptionKey {
  /// Generates an Ed25519 `EncryptionKey`.
  #[wasm_bindgen(js_name = ed25519)]
  pub fn ed25519() -> WasmEncryptionKey {
    Self(WasmEncryptionKeyInner::Ed25519)
  }

  /// Generates an X25519 `EncryptionKey`.
  #[wasm_bindgen(js_name = x25519)]
  pub fn x25519(public_key: Vec<u8>) -> WasmEncryptionKey {
    Self(WasmEncryptionKeyInner::X25519(public_key))
  }

  /// Serializes `EncryptionKey` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `EncryptionKey` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmEncryptionKey> {
    json_value
      .into_serde::<WasmEncryptionKeyInner>()
      .map(Self)
      .wasm_result()
  }
}

impl From<WasmEncryptionKey> for EncryptionKey {
  fn from(wasm_encryption_key: WasmEncryptionKey) -> Self {
    match wasm_encryption_key.0 {
      WasmEncryptionKeyInner::Ed25519 => EncryptionKey::Ed25519,
      WasmEncryptionKeyInner::X25519(public_key) => EncryptionKey::X25519(public_key.into()),
    }
  }
}

impl From<EncryptionKey> for WasmEncryptionKey {
  fn from(encryption_key: EncryptionKey) -> Self {
    match encryption_key {
      EncryptionKey::Ed25519 => WasmEncryptionKey(WasmEncryptionKeyInner::Ed25519),
      EncryptionKey::X25519(public_key) => {
        WasmEncryptionKey(WasmEncryptionKeyInner::X25519(public_key.as_ref().to_vec()))
      }
    }
  }
}
