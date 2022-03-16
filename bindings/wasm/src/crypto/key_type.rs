// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::crypto::KeyType as KeyType_;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum KeyType {
  #[serde(rename = "ed25519")]
  Ed25519 = 1,
}

impl Default for KeyType {
  fn default() -> Self {
    Self::Ed25519
  }
}

impl From<KeyType> for KeyType_ {
  fn from(other: KeyType) -> Self {
    match other {
      KeyType::Ed25519 => KeyType_::Ed25519,
    }
  }
}

impl From<KeyType_> for KeyType {
  fn from(other: KeyType_) -> Self {
    match other {
      KeyType_::Ed25519 => KeyType::Ed25519,
    }
  }
}
