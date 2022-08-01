// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::CoreDID;
use wasm_bindgen::prelude::*;

use crate::did::WasmIotaDID;
use crate::error::Result;
use crate::error::WasmResult;

/// @typicalname did
#[wasm_bindgen(js_name = CoreDID, inspectable)]
pub struct WasmCoreDID(pub(crate) CoreDID);

#[wasm_bindgen(js_class = CoreDID)]
impl WasmCoreDID {
  /// Parses a [`CoreDID`] from the given `input`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`CoreDID`].
  pub fn parse(input: &str) -> Result<WasmCoreDID> {
    CoreDID::parse(input).wasm_result().map(Self)
  }

  /// Returns the `CoreDID` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl_wasm_json!(WasmCoreDID, CoreDID);
impl_wasm_clone!(WasmCoreDID, CoreDID);

impl From<CoreDID> for WasmCoreDID {
  fn from(did: CoreDID) -> Self {
    WasmCoreDID(did)
  }
}

impl From<WasmIotaDID> for WasmCoreDID {
  fn from(wasm_did: WasmIotaDID) -> Self {
    let core_did: CoreDID = wasm_did.0.into();
    core_did.into()
  }
}
