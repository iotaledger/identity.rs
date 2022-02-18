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
  #[napi(js_name = toJSON)]
  pub fn to_json(&self) -> Result<serde_json::Value> {
    serde_json::to_value(&self.0).napi_result()
  }
}
