// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::utils::err;
use identity::did::Service as IotaService;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Service(pub(crate) IotaService);

#[wasm_bindgen]
impl Service {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Service` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<Service, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
