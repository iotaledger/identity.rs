// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::KeyLocation;
use napi::Result;

use crate::error::NapiResult;

#[napi]
pub struct NapiKeyLocation(pub(crate) KeyLocation);

#[napi]
impl NapiKeyLocation {
  #[napi(js_name = fromJSON)]
  pub fn from_json(json_value: serde_json::Value) -> Result<NapiKeyLocation> {
    serde_json::from_value(json_value).map(Self).napi_result()
  }
}
