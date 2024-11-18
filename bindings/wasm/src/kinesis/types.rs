// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;

use crate::error::{wasm_error, Result};

pub use super::ts_client_sdk::wasm_types::*;

pub fn into_sdk_type<T: DeserializeOwned, W: Into<JsValue>>(wasm_type_instance: W) -> Result<T> {
  let js_value: JsValue = wasm_type_instance.into();
  //serde_wasm_bindgen::from_value::<T>(js_value).map_err(wasm_error)
  match serde_wasm_bindgen::from_value::<T>(js_value.clone()) {
    Ok(ret_val) => Ok(ret_val),
    Err(e) => {
      // TODO: Replace all console_log! usages by proper Error management and Result types. 
      // Use console_log! only for debug purposes
      console_log!(
        "[identity_wasm::kinesis::types - fn into_sdk_type]\n   js_value: {:?}\n   Error: {:?}",
        js_value,
        e
      );
      Err(wasm_error(e))
    }
  }
}

pub(crate) type WasmIotaAddress = String;
pub(crate) type WasmObjectID = String;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Balance>")]
  pub type PromiseBalance;
}
