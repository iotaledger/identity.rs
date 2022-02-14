// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::Signature;
use napi::Result;

use crate::error::NapiResult;

#[napi]
#[derive(Serialize)]
pub struct NapiSignature(pub(crate) Signature);

impl From<Signature> for NapiSignature {
  fn from(signature: Signature) -> Self {
    NapiSignature(signature)
  }
}

#[napi]
impl NapiSignature {
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
