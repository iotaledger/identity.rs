// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::Result as AccountResult;
use identity::core::Result as CoreResult;
use napi::bindgen_prelude::Error;
use napi::Result;
use serde_json::Result as SerdeResult;

/// Convenience trait to simplify `result.map_err(account_error)` to `result.napi_result()`
pub trait NapiResult<T> {
  fn napi_result(self) -> Result<T>;
}

//impl<T> NapiResult<T> for impl std::error::Error {
//  fn napi_result(self) -> Result<T> {
//    self.map_err(|account_error| Error::from_reason(account_error.to_string()))
//  }
//}

impl<T> NapiResult<T> for AccountResult<T> {
  fn napi_result(self) -> Result<T> {
    self.map_err(|account_error| Error::from_reason(account_error.to_string()))
  }
}

impl<T> NapiResult<T> for CoreResult<T> {
  fn napi_result(self) -> Result<T> {
    self.map_err(|core_error| Error::from_reason(core_error.to_string()))
  }
}

impl<T> NapiResult<T> for SerdeResult<T> {
  fn napi_result(self) -> Result<T> {
    self.map_err(|serde_error| Error::from_reason(serde_error.to_string()))
  }
}
