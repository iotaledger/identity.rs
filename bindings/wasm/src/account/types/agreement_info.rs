// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::AgreementInfo;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen(js_name = AgreementInfo, inspectable)]
pub struct WasmAgreementInfo(AgreementInfo);

#[wasm_bindgen(js_class = AgreementInfo)]
impl WasmAgreementInfo {
  /// Creates an `AgreementInfo` Object
  #[wasm_bindgen(js_name = new)]
  pub fn new(apu: Vec<u8>, apv: Vec<u8>, pub_info: Vec<u8>, priv_info: Vec<u8>) -> WasmAgreementInfo {
    WasmAgreementInfo(AgreementInfo::new(apu, apv, pub_info, priv_info))
  }

  /// Returns a copy of `apu'
  #[wasm_bindgen(js_name = apu)]
  pub fn apu(&self) -> Vec<u8> {
    self.0.apu().to_vec()
  }

  /// Returns a copy of `apv'
  #[wasm_bindgen(js_name = apv)]
  pub fn apv(&self) -> Vec<u8> {
    self.0.apv().to_vec()
  }

  /// Returns a copy of `pubInfo'
  #[wasm_bindgen(js_name = pubInfo)]
  pub fn pub_info(&self) -> Vec<u8> {
    self.0.pub_info().to_vec()
  }

  /// Returns a copy of `privInfo'
  #[wasm_bindgen(js_name = privInfo)]
  pub fn priv_info(&self) -> Vec<u8> {
    self.0.priv_info().to_vec()
  }

  /// Serializes `AgreementInfo` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `AgreementInfo` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmAgreementInfo> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

impl From<WasmAgreementInfo> for AgreementInfo {
  fn from(wasm_agreement_info: WasmAgreementInfo) -> Self {
    wasm_agreement_info.0
  }
}

impl From<AgreementInfo> for WasmAgreementInfo {
  fn from(agreement_info: AgreementInfo) -> Self {
    WasmAgreementInfo(agreement_info)
  }
}
