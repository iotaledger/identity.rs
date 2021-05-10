// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm;
use wasm_bindgen::prelude::*;

use crate::utils::err;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct AuthenticationRequest(pub(crate) comm::AuthenticationRequest);

#[wasm_bindgen]
impl AuthenticationRequest {
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<AuthenticationRequest, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct AuthenticationResponse(pub(crate) comm::AuthenticationResponse);

#[wasm_bindgen]
impl AuthenticationResponse {
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<AuthenticationResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
