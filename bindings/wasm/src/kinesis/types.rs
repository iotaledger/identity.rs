// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;

pub use super::ts_client_sdk::wasm_types::*;

pub fn into_sdk_type<T: DeserializeOwned, W: Into<JsValue>>(wasm_type_instance: W) -> T {
  let js_value: JsValue = wasm_type_instance.into();
  serde_wasm_bindgen::from_value::<T>(js_value).unwrap()
}

pub(crate) type WasmIotaAddress = String;
pub(crate) type WasmObjectID = String;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Balance>")]
  pub type PromiseBalance;
}
