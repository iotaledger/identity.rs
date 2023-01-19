// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::types::KeyLocation;
use identity_did::CoreDID;
use napi::Result;
use napi_derive::napi;

use crate::error::NapiResult;
use crate::types::NapiCoreDid;
use crate::types::NapiKeyLocation;

/// Workaround for lack of tuple support in Napi.
#[derive(serde::Serialize, serde::Deserialize)]
struct DIDLocation {
  did: CoreDID,
  location: KeyLocation,
}

#[napi]
pub struct NapiDidLocation(DIDLocation);

#[napi]
impl NapiDidLocation {
  #[napi]
  pub fn did(&self) -> NapiCoreDid {
    NapiCoreDid(self.0.did.clone())
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

impl From<(CoreDID, KeyLocation)> for NapiDidLocation {
  fn from(tuple: (CoreDID, KeyLocation)) -> Self {
    Self(DIDLocation {
      did: tuple.0,
      location: tuple.1,
    })
  }
}
