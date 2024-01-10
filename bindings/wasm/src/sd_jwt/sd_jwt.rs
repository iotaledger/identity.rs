// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::{common::ArrayString, error::WasmResult};
use identity_iota::sd_jwt_payload::SdJwt;
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = SdJwt, inspectable)]
pub struct WasmSdJwt(pub(crate) SdJwt);

#[wasm_bindgen(js_class = SdJwt)]
impl WasmSdJwt {
  #[wasm_bindgen(constructor)]
  pub fn new(jwt: String, disclosures: ArrayString, key_binding_jwt: Option<String>) -> Result<WasmSdJwt> {
    let disclosures: Vec<String> = disclosures
      .dyn_into::<Array>()?
      .iter()
      .map(|item| item.dyn_into::<JsString>().map(String::from))
      .collect::<Result<Vec<String>>>()?;
    let sd_jwt = SdJwt::new(jwt, disclosures, key_binding_jwt);
    Ok(WasmSdJwt(sd_jwt))
  }

  #[wasm_bindgen]
  pub fn presentation(&self) -> String {
    self.0.presentation()
  }

  #[wasm_bindgen]
  pub fn parse(sd_jwt: String) -> Result<WasmSdJwt> {
    let sd_jwt = SdJwt::parse(&sd_jwt).wasm_result()?;
    Ok(WasmSdJwt(sd_jwt))
  }

  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.presentation()
  }

  #[wasm_bindgen]
  pub fn jwt(&self) -> String {
    self.0.jwt.clone()
  }

  #[wasm_bindgen]
  pub fn disclosuress(&self) -> ArrayString {
    self
      .0
      .disclosures
      .clone()
      .iter()
      .map(|url| url.to_string())
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }

  #[wasm_bindgen(js_name = keyBindingJwt)]
  pub fn key_binding_jwt(&self) -> Option<String> {
    self.0.key_binding_jwt.clone()
  }
}

impl_wasm_json!(WasmSdJwt, SdJwt);
impl_wasm_clone!(WasmSdJwt, SdJwt);
