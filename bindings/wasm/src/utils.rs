// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::JsValue;

/// Convert errors so they are readable in JS
pub fn err<T>(error: T) -> JsValue
where
  T: ToString,
{
  error.to_string().into()
}
