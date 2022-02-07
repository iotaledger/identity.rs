// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::error::Result as AccountResult;
use napi::bindgen_prelude::Error;
use napi::Result;

/// Convenience trait to simplify `result.map_err(account_error)` to `result.napi_result()`
pub trait NapiResult<T> {
  fn napi_result(self) -> Result<T>;
}

impl<T> NapiResult<T> for AccountResult<T> {
  fn napi_result(self) -> Result<T> {
    self.map_err(|account_error| Error::from_reason(account_error.to_string()))
  }
}
