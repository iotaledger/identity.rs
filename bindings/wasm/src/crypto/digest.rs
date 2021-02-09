// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::JsValue;

use crate::utils::err;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Digest {
  #[serde(rename = "sha256")]
  Sha256,
}

impl Digest {
  pub fn from_value(value: &JsValue) -> Result<Self, JsValue> {
    if value.is_falsy() {
      Ok(Self::Sha256)
    } else {
      value.into_serde().map_err(err)
    }
  }
}

impl From<&str> for Digest {
  fn from(_: &str) -> Self {
    Self::Sha256
  }
}
