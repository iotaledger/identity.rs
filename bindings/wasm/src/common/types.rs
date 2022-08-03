// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::core::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::error::WasmResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<void>")]
  pub type PromiseVoid;

  #[wasm_bindgen(typescript_type = "Promise<boolean>")]
  pub type PromiseBool;

  #[wasm_bindgen(typescript_type = "Array<string>")]
  pub type ArrayString;

  #[wasm_bindgen(typescript_type = "Map<string, any>")]
  pub type MapStringAny;

  #[wasm_bindgen(typescript_type = "number | number[]")]
  pub type UOneOrManyNumber;

  #[wasm_bindgen(typescript_type = "string | string[] | null")]
  pub type OptionOneOrManyString;
}

impl TryFrom<Object> for MapStringAny {
  type Error = JsValue;

  fn try_from(properties: Object) -> Result<Self, Self::Error> {
    MapStringAny::try_from(&properties)
  }
}

impl TryFrom<&Object> for MapStringAny {
  type Error = JsValue;

  fn try_from(properties: &Object) -> Result<Self, Self::Error> {
    let map: js_sys::Map = js_sys::Map::new();
    for (key, value) in properties.iter() {
      map.set(
        &JsValue::from_str(key.as_str()),
        &JsValue::from_serde(&value).wasm_result()?,
      );
    }
    Ok(map.unchecked_into::<MapStringAny>())
  }
}
