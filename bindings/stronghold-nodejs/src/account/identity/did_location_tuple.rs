// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::types::KeyLocation;
use identity_iota_core::did::IotaDID;
use napi::Result;
use napi_derive::napi;

use crate::error::NapiResult;

#[napi]
pub struct NapiDIDLocation(DIDLocation);

#[derive(serde::Serialize, serde::Deserialize)]
struct DIDLocation {
  did: IotaDID,
  location: KeyLocation,
}

#[napi]
impl NapiDIDLocation {
  #[napi(js_name = fromJSON)]
  pub fn from_json(json_value: serde_json::Value) -> Result<NapiDIDLocation> {
    serde_json::from_value::<DIDLocation>(json_value)
      .map(Self)
      .napi_result()
  }

  #[napi(js_name = toJSON)]
  pub fn to_json(&self) -> Result<serde_json::Value> {
    serde_json::to_value(&self.0).napi_result()
  }
}

impl From<(IotaDID, KeyLocation)> for NapiDIDLocation {
  fn from(tuple: (IotaDID, KeyLocation)) -> Self {
    Self(DIDLocation {
      did: tuple.0,
      location: tuple.1,
    })
  }
}
