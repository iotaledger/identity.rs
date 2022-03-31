// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::types::KeyLocation;
use identity_iota_core::did::IotaDID;
use napi::Result;
use napi_derive::napi;

use crate::account::NapiKeyLocation;
use crate::did::NapiDID;
use crate::error::NapiResult;

#[derive(serde::Serialize, serde::Deserialize)]
struct DIDLocation {
  did: IotaDID,
  location: KeyLocation,
}

#[napi]
pub struct NapiDidLocation(DIDLocation);

#[napi]
impl NapiDidLocation {
  #[napi]
  pub fn did(&self) -> NapiDID {
    NapiDID(self.0.did.clone())
  }

  #[napi(js_name = keyLocation)]
  pub fn key_location(&self) -> NapiKeyLocation {
    NapiKeyLocation(self.0.location.clone())
  }

  #[napi(js_name = fromJSON)]
  pub fn from_json(json_value: serde_json::Value) -> Result<NapiDidLocation> {
    serde_json::from_value::<DIDLocation>(json_value)
      .map(Self)
      .napi_result()
  }

  #[napi(js_name = toJSON)]
  pub fn to_json(&self) -> Result<serde_json::Value> {
    serde_json::to_value(&self.0).napi_result()
  }
}

impl From<(IotaDID, KeyLocation)> for NapiDidLocation {
  fn from(tuple: (IotaDID, KeyLocation)) -> Self {
    Self(DIDLocation {
      did: tuple.0,
      location: tuple.1,
    })
  }
}
