// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core::did::IotaDID;
use napi::Result;
use napi_derive::napi;

use crate::error::NapiResult;

#[napi(js_name = NapiDID)]
pub struct NapiDID(pub(crate) IotaDID);

#[napi(js_name = NapiDID)]
impl NapiDID {
  #[napi(js_name = fromJSON)]
  pub fn from_json(json_value: serde_json::Value) -> Result<NapiDID> {
    serde_json::from_value(json_value).map(Self).napi_result()
  }
}
