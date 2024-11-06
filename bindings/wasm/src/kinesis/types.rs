// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::de::DeserializeOwned;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaObjectData;

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

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IotaObjectData")]
  pub type WasmIotaObjectData;
}

