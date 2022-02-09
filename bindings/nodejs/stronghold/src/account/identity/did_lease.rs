// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::DIDLease;

#[napi]
pub struct JsDIDLease(pub(crate) DIDLease);

impl From<DIDLease> for JsDIDLease {
  fn from(did_lease: DIDLease) -> Self {
    JsDIDLease(did_lease)
  }
}
