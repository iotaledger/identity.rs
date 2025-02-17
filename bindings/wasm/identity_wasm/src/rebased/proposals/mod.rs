// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod deactivate_did;
mod update_did;
mod send;

pub use deactivate_did::*;
pub use update_did::*;
pub use send::*;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast as _;
use js_sys::Reflect;
use js_sys::JsString;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Set<string>")]
  pub type StringSet;
  #[wasm_bindgen(typescript_type = "[string, string]")]
  pub type StringCouple;
}

impl From<StringCouple> for (String, String) {
  fn from(value: StringCouple) -> Self {
    let first = Reflect::get_u32(&value, 0)
      .expect("[string, string] has property 0")
      .unchecked_into::<JsString>()
      .into();
    let second = Reflect::get_u32(&value, 1)
      .expect("[string, string] has property 1")
      .unchecked_into::<JsString>()
      .into();

    (first, second)
  }
}

impl From<(String, String)> for StringCouple {
  fn from(value: (String, String)) -> Self {
    serde_wasm_bindgen::to_value(&value).expect("a string couple can be serialized to JS").unchecked_into()
  }
}