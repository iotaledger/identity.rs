// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::Signature;
use napi::Error;
use napi::Result;

#[napi]
#[derive(Serialize)]
pub struct JsSignature(pub(crate) Signature);

impl From<Signature> for JsSignature {
  fn from(signature: Signature) -> Self {
    JsSignature(signature)
  }
}

#[napi]
impl JsSignature {
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
