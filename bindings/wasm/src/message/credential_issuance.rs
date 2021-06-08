// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm;
use wasm_bindgen::prelude::*;

use crate::wasm_did::WasmDID;
use crate::wasm_url::WasmUrl;
use crate::wasm_uuid::WasmUuid;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialSelection(pub(crate) comm::CredentialSelection);

impl_wasm_accessors!(CredentialSelection, {
  context => String,
  thread => WasmUuid,
  callback_url => WasmUrl,
  response_requested => Option<bool>,
  id => Option<WasmDID>,
});

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialIssuance(pub(crate) comm::CredentialIssuance);

impl_wasm_accessors!(CredentialIssuance, {
  context => String,
  thread => WasmUuid,
  callback_url => Option<WasmUrl>,
  response_requested => Option<bool>,
  id => Option<WasmDID>,
});
