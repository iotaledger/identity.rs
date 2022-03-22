// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::crypto::KeyType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = KeyType)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum WasmKeyType {
  #[serde(rename = "ed25519")]
  Ed25519 = 1,
  #[serde(rename = "x25519")]
  X25519 = 2,
}

impl From<WasmKeyType> for KeyType {
  fn from(other: WasmKeyType) -> Self {
    match other {
      WasmKeyType::Ed25519 => KeyType::Ed25519,
      WasmKeyType::X25519 => KeyType::X25519,
    }
  }
}

impl From<KeyType> for WasmKeyType {
  fn from(other: KeyType) -> Self {
    match other {
      KeyType::Ed25519 => WasmKeyType::Ed25519,
      KeyType::X25519 => WasmKeyType::X25519,
    }
  }
}
