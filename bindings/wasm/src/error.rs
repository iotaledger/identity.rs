// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use wasm_bindgen::JsValue;

/// Convert an error into an idiomatic [js_sys::Error].
pub fn wasm_error<'a, T>(error: T) -> JsValue
where
  T: Into<WasmError<'a>>,
{
  let wasm_err: WasmError = error.into();
  JsValue::from(wasm_err)
}

/// Convenience struct to convert internal errors to [js_sys::Error]. Uses [std::borrow::Cow]
/// internally to avoid unnecessary clones.
///
/// This is a workaround for orphan rules so we can implement [core::convert::From] on errors from
/// dependencies.
pub struct WasmError<'a> {
  name: Cow<'a, str>,
  message: Cow<'a, str>,
}

/// Convert [WasmError] into [wasm_bindgen::JsValue], represented as a [js_sys::Error] for idiomatic
/// error handling.
impl From<WasmError<'_>> for JsValue {
  fn from(error: WasmError<'_>) -> Self {
    let js_error = js_sys::Error::new(&error.message);
    js_error.set_name(&error.name);
    JsValue::from(js_error)
  }
}

/// Implement WasmError for each type individually rather than a trait due to Rust's orphan rules.
/// Each type must implement `Into<&'static str> + Display`. The `Into<&'static str>` trait can be
/// derived using `strum::IntoStaticStr`.
macro_rules! impl_wasm_error_from {
  ( $($t:ty),* ) => {
  $(impl From<$t> for WasmError<'_> {
    fn from(error: $t) -> Self {
      Self {
        message: Cow::Owned(error.to_string()),
        name: Cow::Borrowed(error.into()),
      }
    }
  })*
  }
}

impl_wasm_error_from!(
  identity::comm::Error,
  identity::core::Error,
  identity::credential::Error,
  identity::did::Error,
  identity::iota::Error
);

impl From<serde_json::Error> for WasmError<'_> {
  fn from(error: serde_json::Error) -> Self {
    Self {
      name: Cow::Borrowed("serde_json::Error"), // the exact error code is embedded in the message
      message: Cow::Owned(error.to_string()),
    }
  }
}

impl From<identity::iota::BeeMessageError> for WasmError<'_> {
  fn from(error: identity::iota::BeeMessageError) -> Self {
    Self {
      name: Cow::Borrowed("bee_message::Error"),
      message: Cow::Owned(error.to_string()),
    }
  }
}
