// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::ChainState;
use napi::JsBuffer;
use napi::Result;

use crate::error::NapiResult;

#[napi]
#[derive(Deserialize, Serialize)]
pub struct NapiChainState(pub(crate) ChainState);

#[napi]
impl NapiChainState {
  #[napi]
  pub fn from_json_value(json_value: serde_json::Value) -> Result<NapiChainState> {
    serde_json::from_value(json_value).napi_result()
  }

  #[napi]
  pub fn from_buffer(buffer: JsBuffer) -> Result<NapiChainState> {
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

impl From<ChainState> for NapiChainState {
  fn from(chain_state: ChainState) -> Self {
    NapiChainState(chain_state)
  }
}
