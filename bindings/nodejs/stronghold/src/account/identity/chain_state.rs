// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::ChainState;
use napi::Error;
use napi::JsBuffer;
use napi::Result;

#[napi]
#[derive(Deserialize, Serialize)]
pub struct JsChainState(pub(crate) ChainState);

#[napi]
impl JsChainState {
  #[napi]
  pub fn from_json_value(json_value: serde_json::Value) -> Result<JsChainState> {
    serde_json::from_value(json_value).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn from_buffer(buffer: JsBuffer) -> Result<JsChainState> {
    let bytes: &[u8] = &buffer.into_value()?;
    bincode::deserialize(bytes).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn as_json(&self) -> Result<serde_json::Value> {
    serde_json::to_value(&self).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn as_bytes(&self) -> Result<Vec<u32>> {
    let bytes: Vec<u8> =
      bincode::serialize(&self).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(bytes.into_iter().map(|v| v as u32).collect())
  }
}

impl From<ChainState> for JsChainState {
  fn from(chain_state: ChainState) -> Self {
    JsChainState(chain_state)
  }
}
