// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::types::DIDType;
use napi::bindgen_prelude::ToNapiValue;
use napi_derive::napi;

#[napi]
pub enum NapiDIDType {
  IotaDID,
}

impl From<NapiDIDType> for DIDType {
  fn from(other: NapiDIDType) -> Self {
    match other {
      NapiDIDType::IotaDID => DIDType::IotaDID,
    }
  }
}
