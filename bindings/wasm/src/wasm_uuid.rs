// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm::Uuid;
use wasm_bindgen::prelude::*;

use crate::utils::err;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct WasmUuid(pub(crate) Uuid);

#[wasm_bindgen]
impl WasmUuid {
  #[wasm_bindgen(constructor)]
  pub fn new(data: String) -> Result<WasmUuid, JsValue> {
    Uuid::parse_str(&data).map_err(err).map(Self)
  }

  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl From<Uuid> for WasmUuid {
  fn from(other: Uuid) -> Self {
    Self(other)
  }
}

impl From<WasmUuid> for Uuid {
  fn from(other: WasmUuid) -> Self {
    other.0
  }
}
