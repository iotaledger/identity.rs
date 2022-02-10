// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::KeyLocation;
use napi::Error;
use napi::JsBuffer;
use napi::Result;

#[napi]
#[derive(Deserialize)]
pub struct JsKeyLocation(pub(crate) KeyLocation);

#[napi]
impl JsKeyLocation {
  #[napi]
  pub fn from_json_value(json_value: serde_json::Value) -> Result<JsKeyLocation> {
    serde_json::from_value(json_value).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn from_buffer(buffer: JsBuffer) -> Result<JsKeyLocation> {
    let bytes: &[u8] = &buffer.into_value()?;
    bincode::deserialize(bytes).map_err(|e| Error::from_reason(e.to_string()))
  }
}
