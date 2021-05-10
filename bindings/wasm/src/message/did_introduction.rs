// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm;
use wasm_bindgen::prelude::*;

use crate::utils::err;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct IntroductionProposal(pub(crate) comm::IntroductionProposal);

#[wasm_bindgen]
impl IntroductionProposal {
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<IntroductionProposal, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct IntroductionResponse(pub(crate) comm::IntroductionResponse);

#[wasm_bindgen]
impl IntroductionResponse {
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<IntroductionResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Introduction(pub(crate) comm::Introduction);

#[wasm_bindgen]
impl Introduction {
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<Introduction, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
