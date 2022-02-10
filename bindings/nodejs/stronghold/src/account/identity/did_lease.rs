// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::DIDLease;
use napi::Error;
use napi::Result;

#[napi]
#[derive(Serialize)]
pub struct JsDIDLease(pub(crate) DIDLease);

#[napi]
impl JsDIDLease {
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

impl From<DIDLease> for JsDIDLease {
  fn from(did_lease: DIDLease) -> Self {
    JsDIDLease(did_lease)
  }
}
