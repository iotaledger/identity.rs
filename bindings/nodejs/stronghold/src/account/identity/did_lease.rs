// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::DIDLease;

#[napi(js_name = NapiDIDLease)]
pub struct NapiDIDLease(pub(crate) DIDLease);

#[napi(js_name = NapiDIDLease)]
impl NapiDIDLease {
  #[napi]
  pub fn load(&self) -> bool {
    self.0.load()
  }
}

impl From<DIDLease> for NapiDIDLease {
  fn from(did_lease: DIDLease) -> Self {
    NapiDIDLease(did_lease)
  }
}
