// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::IdentityState;

#[napi(js_name = IdentityState)]
pub struct JsIdentityState(pub(crate) IdentityState);

impl From<IdentityState> for JsIdentityState {
  fn from(identity_state: IdentityState) -> Self {
    JsIdentityState(identity_state)
  }
}
