// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::Result as AccountStorageResult;
use identity_core::Result as CoreResult;
use identity_iota_core_legacy::Result as IotaCoreResult;
use napi::bindgen_prelude::Error;
use napi::Result;
use serde_json::Result as SerdeResult;

/// Convenience trait to simplify `result.map_err(account_error)` to `result.napi_result()`
pub trait NapiResult<T> {
  fn napi_result(self) -> Result<T>;
}

impl<T> NapiResult<T> for AccountStorageResult<T> {
  fn napi_result(self) -> Result<T> {
    self.map_err(|account_storage_error| Error::from_reason(error_chain_fmt(&account_storage_error)))
  }
}

impl<T> NapiResult<T> for CoreResult<T> {
  fn napi_result(self) -> Result<T> {
    self.map_err(|core_error| Error::from_reason(error_chain_fmt(&core_error)))
  }
}

impl<T> NapiResult<T> for IotaCoreResult<T> {
  fn napi_result(self) -> Result<T> {
    self.map_err(|iota_core_error| Error::from_reason(error_chain_fmt(&iota_core_error)))
  }
}

impl<T> NapiResult<T> for SerdeResult<T> {
  fn napi_result(self) -> Result<T> {
    self.map_err(|serde_error| Error::from_reason(error_chain_fmt(&serde_error)))
  }
}

// the following function is inspired by https://www.lpalmieri.com/posts/error-handling-rust/#error-source
fn error_chain_fmt(err: &impl std::error::Error) -> String {
  let mut error = err.to_string() + ".";

  let mut current = err.source();
  while let Some(cause) = current {
    error = format!("{error} Caused by: {}.", cause);
    current = cause.source();
  }

  error
}
