// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm;
use wasm_bindgen::prelude::*;

use crate::wasm_did::WasmDID;
use crate::wasm_url::WasmUrl;
use crate::wasm_uuid::WasmUuid;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialRevocation(pub(crate) comm::CredentialRevocation);

impl_wasm_accessors!(CredentialRevocation, {
  context => String,
  thread => WasmUuid,
  credential_id => String,
  callback_url => Option<WasmUrl>,
  response_requested => Option<bool>,
  id => Option<WasmDID>,
  comment => Option<String>,
});
