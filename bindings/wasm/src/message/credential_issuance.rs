// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm;
use wasm_bindgen::prelude::*;

use crate::utils::err;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialSelection(pub(crate) comm::CredentialSelection);

#[wasm_bindgen]
impl CredentialSelection {
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<CredentialSelection, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialIssuance(pub(crate) comm::CredentialIssuance);

#[wasm_bindgen]
impl CredentialIssuance {
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<CredentialIssuance, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
