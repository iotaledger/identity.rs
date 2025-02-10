// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::result::Result as StdResult;

use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use wasm_bindgen::JsValue;

use crate::common::into_sdk_type;
use identity_iota_interaction::types::execution_status::CommandArgumentError;
use identity_iota_interaction::types::execution_status::ExecutionFailureStatus;
use identity_iota_interaction::types::execution_status::PackageUpgradeError;
use identity_iota_interaction::types::execution_status::TypeArgumentError;
use thiserror::Error as ThisError;

/// Convenience wrapper for `Result<T, JsValue>`.
///
/// All exported errors must be converted to [`JsValue`] when using wasm_bindgen.
/// See: https://rustwasm.github.io/docs/wasm-bindgen/reference/types/result.html
pub type Result<T> = core::result::Result<T, JsValue>;

/// Convert an error into an idiomatic [js_sys::Error].
pub fn wasm_error<'a, E>(error: E) -> JsValue
where
  E: Into<WasmError<'a>>,
{
  let wasm_err: WasmError<'_> = error.into();
  JsValue::from(wasm_err)
}

/// Convenience trait to simplify `result.map_err(wasm_error)` to `result.wasm_result()`
pub trait WasmResult<T> {
  fn wasm_result(self) -> Result<T>;
}

impl<'a, T, E> WasmResult<T> for core::result::Result<T, E>
where
  E: Into<WasmError<'a>>,
{
  fn wasm_result(self) -> Result<T> {
    self.map_err(wasm_error)
  }
}

/// Convenience struct to convert internal errors to [js_sys::Error]. Uses [std::borrow::Cow]
/// internally to avoid unnecessary clones.
///
/// This is a workaround for orphan rules so we can implement [core::convert::From] on errors from
/// dependencies.
#[derive(Debug, Clone)]
pub struct WasmError<'a> {
  pub name: Cow<'a, str>,
  pub message: Cow<'a, str>,
}

impl<'a> WasmError<'a> {
  pub fn new(name: Cow<'a, str>, message: Cow<'a, str>) -> Self {
    Self { name, message }
  }
}

/// Convert [WasmError] into [js_sys::Error] for idiomatic error handling.
impl From<WasmError<'_>> for js_sys::Error {
  fn from(error: WasmError<'_>) -> Self {
    let js_error = js_sys::Error::new(&error.message);
    js_error.set_name(&error.name);
    js_error
  }
}

/// Convert [WasmError] into [wasm_bindgen::JsValue].
impl From<WasmError<'_>> for JsValue {
  fn from(error: WasmError<'_>) -> Self {
    JsValue::from(js_sys::Error::from(error))
  }
}

/// Implement WasmError for each type individually rather than a trait due to Rust's orphan rules.
/// Each type must implement `Into<&'static str> + Display`. The `Into<&'static str>` trait can be
/// derived using `strum::IntoStaticStr`.
#[macro_export]
macro_rules! impl_wasm_error_from {
  ( $($t:ty),* ) => {
  $(impl From<$t> for WasmError<'_> {
    fn from(error: $t) -> Self {
      Self {
        message: Cow::Owned(ErrorMessage(&error).to_string()),
        name: Cow::Borrowed(error.into()),
      }
    }
  })*
  }
}

// Similar to `impl_wasm_error_from`, but uses the types name instead of requiring/calling Into &'static str
#[macro_export]
macro_rules! impl_wasm_error_from_with_struct_name {
  ( $($t:ty),* ) => {
  $(impl From<$t> for WasmError<'_> {
    fn from(error: $t) -> Self {
      Self {
        message: Cow::Owned(error.to_string()),
        name: Cow::Borrowed(stringify!($t)),
      }
    }
  })*
  }
}

impl From<JsValue> for WasmError<'_> {
  fn from(error: JsValue) -> Self {
    let js_err = js_sys::Error::from(error);
    let name: String = js_err.name().into();
    let message: String = js_err.message().into();
    WasmError::new(name.into(), message.into())
  }
}

// identity_iota::iota now has some errors where the error message does not include the source error's error message.
// This is in compliance with the Rust error handling project group's recommendation:
// * An error type with a source error should either return that error via source or include that source's error message
//   in its own Display output, but never both. *
// See https://blog.rust-lang.org/inside-rust/2021/07/01/What-the-error-handling-project-group-is-working-towards.html#guidelines-for-implementing-displayfmt-and-errorsource.
//
// However in WasmError we want the display message of the entire error chain. We introduce a workaround here that let's
// us display the entire display chain for new variants that don't include the error message of the source error in its
// own display.

// the following function is inspired by https://www.lpalmieri.com/posts/error-handling-rust/#error-source
fn error_chain_fmt(e: &impl std::error::Error, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  write!(f, "{e}. ")?;
  let mut current = e.source();
  while let Some(cause) = current {
    write!(f, "Caused by: {cause}. ")?;
    current = cause.source();
  }
  Ok(())
}

struct ErrorMessage<'a, E: std::error::Error>(&'a E);

impl<'a, E: std::error::Error> Display for ErrorMessage<'a, E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    error_chain_fmt(self.0, f)
  }
}

impl From<serde_json::Error> for WasmError<'_> {
  fn from(error: serde_json::Error) -> Self {
    Self {
      name: Cow::Borrowed("serde_json::Error"), // the exact error code is embedded in the message
      message: Cow::Owned(error.to_string()),
    }
  }
}

impl From<serde_wasm_bindgen::Error> for WasmError<'_> {
  fn from(error: serde_wasm_bindgen::Error) -> Self {
    Self {
      name: Cow::Borrowed("serde_wasm_bindgen::Error"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

/// Consumes the struct and returns a Result<_, String>, leaving an `Ok` value untouched.
pub fn stringify_js_error<T>(result: Result<T>) -> StdResult<T, String> {
  result.map_err(|js_value| {
    let error_string: String = match wasm_bindgen::JsCast::dyn_into::<js_sys::Error>(js_value) {
      Ok(js_err) => ToString::to_string(&js_err.to_string()),
      Err(js_val) => {
        // Fall back to debug formatting if this is not a proper JS Error instance.
        format!("{js_val:?}")
      }
    };
    error_string
  })
}

#[derive(ThisError, Debug)]
pub enum TsSdkError {
  #[error("[TsSdkError] PackageUpgradeError: {0}")]
  PackageUpgradeError(#[from] PackageUpgradeError),
  #[error("[TsSdkError] CommandArgumentError: {0}")]
  CommandArgumentError(#[from] CommandArgumentError),
  #[error("[TsSdkError] ExecutionFailureStatus: {0}")]
  ExecutionFailureStatus(#[from] ExecutionFailureStatus),
  #[error("[TsSdkError] TypeArgumentError: {0}")]
  TypeArgumentError(#[from] TypeArgumentError),
  #[error("[TsSdkError] WasmError:{{\n   name: {0},\n   message: {1}\n}}")]
  WasmError(String, String),
  #[error("[TsSdkError] JsSysError: {0}")]
  JsSysError(String),
  #[error("[TsSdkError] TransactionSerializationError: {0}")]
  TransactionSerializationError(String),
}

pub type TsSdkResult<T> = core::result::Result<T, TsSdkError>;

impl From<WasmError<'_>> for TsSdkError {
  fn from(err: WasmError<'_>) -> Self {
    TsSdkError::WasmError(err.name.to_string(), err.message.to_string())
  }
}

pub fn into_ts_sdk_result<T: DeserializeOwned>(result: Result<JsValue>) -> TsSdkResult<T> {
  let result_str = stringify_js_error(result);
  let js_value = result_str.map_err(|e| TsSdkError::JsSysError(e))?;
  let ret_val: T = into_sdk_type(js_value)?;
  Ok(ret_val)
}
