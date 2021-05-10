// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm;
use wasm_bindgen::prelude::*;

use crate::utils::err;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct FeaturesRequest(pub(crate) comm::FeaturesRequest);

#[wasm_bindgen]
impl FeaturesRequest {
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<FeaturesRequest, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct FeaturesResponse(pub(crate) comm::FeaturesResponse);

#[wasm_bindgen]
impl FeaturesResponse {
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<FeaturesResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
