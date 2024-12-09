// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;

use crate::error::{wasm_error, WasmError};

pub fn into_sdk_type<'a, T: DeserializeOwned, W: Into<JsValue>>(wasm_type_instance: W) -> core::result::Result<T, WasmError<'a>> {
  let js_value: JsValue = wasm_type_instance.into();
  match serde_wasm_bindgen::from_value::<T>(js_value.clone()) {
    Ok(ret_val) => Ok(ret_val),
    Err(e) => {
      // TODO: Replace all console_log! usages by proper Error management and Result types. 
      // Use console_log! only for debug purposes
      console_log!(
        "[iota_interaction_ts::common::utils - fn into_sdk_type]\n   js_value: {:?}\n   Error: {:?}",
        js_value,
        e
      );
      Err(e.into())
    }
  }
}