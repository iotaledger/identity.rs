// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Url;
use wasm_bindgen::prelude::*;

use crate::utils::err;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct WasmUrl(pub(crate) Url);

#[wasm_bindgen]
impl WasmUrl {
  #[wasm_bindgen(constructor)]
  pub fn new(data: String) -> Result<WasmUrl, JsValue> {
    Url::parse(data).map_err(err).map(Self)
  }

  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl From<Url> for WasmUrl {
  fn from(other: Url) -> Self {
    Self(other)
  }
}

impl From<WasmUrl> for Url {
  fn from(other: WasmUrl) -> Self {
    other.0
  }
}
