// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::identity::IdentityState;
use napi::Result;
use napi_derive::napi;

use crate::error::NapiResult;

#[napi]
pub struct NapiIdentityState(pub(crate) IdentityState);

#[napi]
impl NapiIdentityState {
  #[napi(js_name = fromJSON)]
  pub fn from_json(json_value: serde_json::Value) -> Result<NapiIdentityState> {
    serde_json::from_value(json_value).map(Self).napi_result()
  }

  #[napi(js_name = toJSON)]
  pub fn to_json(&self) -> Result<serde_json::Value> {
    serde_json::to_value(&self.0).napi_result()
  }
}

impl From<IdentityState> for NapiIdentityState {
  fn from(identity_state: IdentityState) -> Self {
    NapiIdentityState(identity_state)
  }
}
