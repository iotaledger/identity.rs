// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm;
use identity::core::Timestamp;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Timing(pub(crate) comm::Timing);

#[wasm_bindgen]
impl Timing {
  #[wasm_bindgen(getter = outTime)]
  pub fn out_time(&self) -> Option<String> {
    self.0.out_time().map(ToString::to_string)
  }

  #[wasm_bindgen(setter = outTime)]
  pub fn set_out_time(&mut self, value: &str) {
    self.0.set_out_time(Timestamp::parse(value).ok());
  }

  #[wasm_bindgen(getter = inTime)]
  pub fn in_time(&self) -> Option<String> {
    self.0.in_time().map(ToString::to_string)
  }

  #[wasm_bindgen(setter = inTime)]
  pub fn set_in_time(&mut self, value: &str) {
    self.0.set_in_time(Timestamp::parse(value).ok());
  }

  #[wasm_bindgen(getter = staleTime)]
  pub fn stale_time(&self) -> Option<String> {
    self.0.stale_time().map(ToString::to_string)
  }

  #[wasm_bindgen(setter = staleTime)]
  pub fn set_stale_time(&mut self, value: &str) {
    self.0.set_stale_time(Timestamp::parse(value).ok());
  }

  #[wasm_bindgen(getter = expiresTime)]
  pub fn expires_time(&self) -> Option<String> {
    self.0.expires_time().map(ToString::to_string)
  }

  #[wasm_bindgen(setter = expiresTime)]
  pub fn set_expires_time(&mut self, value: &str) {
    self.0.set_expires_time(Timestamp::parse(value).ok());
  }

  #[wasm_bindgen(getter = waitUntilTime)]
  pub fn wait_until_time(&self) -> Option<String> {
    self.0.wait_until_time().map(ToString::to_string)
  }

  #[wasm_bindgen(setter = waitUntilTime)]
  pub fn set_wait_until_time(&mut self, value: &str) {
    self.0.set_wait_until_time(Timestamp::parse(value).ok());
  }

  #[wasm_bindgen(getter = delayMilli)]
  pub fn delay_milli(&self) -> Option<u32> {
    self.0.delay_milli()
  }

  #[wasm_bindgen(setter = delayMilli)]
  pub fn set_delay_milli(&mut self, value: u32) {
    self.0.set_delay_milli(value);
  }

  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(wasm_error)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<Timing, JsValue> {
    value.into_serde().map_err(wasm_error).map(Self)
  }
}
