// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::IotaDID;
use napi::JsBuffer;
use napi::Result;

use crate::error::NapiResult;

#[napi(js_name = NapiDID)]
#[derive(Deserialize)]
pub struct NapiDID(pub(crate) IotaDID);

#[napi(js_name = NapiDID)]
impl NapiDID {
  #[napi]
  pub fn from_json_value(json_value: serde_json::Value) -> Result<NapiDID> {
    serde_json::from_value(json_value).napi_result()
  }

  #[napi]
  pub fn from_buffer(buffer: JsBuffer) -> Result<NapiDID> {
    let bytes: &[u8] = &buffer.into_value()?;
    bincode::deserialize(bytes).napi_result()
  }
}
