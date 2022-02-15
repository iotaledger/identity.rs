// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::KeyLocation;
use napi::JsBuffer;
use napi::Result;

use crate::error::NapiResult;

#[napi]
#[derive(Deserialize)]
pub struct NapiKeyLocation(pub(crate) KeyLocation);

#[napi]
impl NapiKeyLocation {
  #[napi]
  pub fn from_json_value(json_value: serde_json::Value) -> Result<NapiKeyLocation> {
    serde_json::from_value(json_value).napi_result()
  }

  #[napi]
  pub fn from_buffer(buffer: JsBuffer) -> Result<NapiKeyLocation> {
    let bytes: &[u8] = &buffer.into_value()?;
    bincode::deserialize(bytes).napi_result()
  }
}
