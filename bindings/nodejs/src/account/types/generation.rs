// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::Generation;
use napi::JsBuffer;
use napi::Result;

use crate::error::NapiResult;

#[napi]
#[derive(Deserialize, Serialize)]
pub struct NapiGeneration(pub(crate) Generation);

#[napi]
impl NapiGeneration {
  #[napi]
  pub fn from_json_value(json_value: serde_json::Value) -> Result<NapiGeneration> {
    serde_json::from_value(json_value).napi_result()
  }

  #[napi]
  pub fn from_buffer(buffer: JsBuffer) -> Result<NapiGeneration> {
    let bytes: &[u8] = &buffer.into_value()?;
    bincode::deserialize(bytes).napi_result()
  }

  #[napi]
  pub fn as_json(&self) -> Result<serde_json::Value> {
    serde_json::to_value(&self).napi_result()
  }

  #[napi]
  pub fn as_bytes(&self) -> Result<Vec<u32>> {
    let bytes: Vec<u8> = bincode::serialize(&self).napi_result()?;
    Ok(bytes.into_iter().map(|v| v as u32).collect())
  }
}

impl From<Generation> for NapiGeneration {
  fn from(generation: Generation) -> Self {
    NapiGeneration(generation)
  }
}
