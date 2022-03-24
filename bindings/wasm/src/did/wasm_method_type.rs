// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::did::MethodType;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Supported verification method types.
#[wasm_bindgen(js_name = MethodType, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmMethodType(pub(crate) MethodType);

#[wasm_bindgen(js_class = MethodType)]
impl WasmMethodType {
  #[wasm_bindgen(js_name = Ed25519VerificationKey2018)]
  pub fn ed25519_verification_key_2018() -> WasmMethodType {
    WasmMethodType(MethodType::Ed25519VerificationKey2018)
  }

  #[wasm_bindgen(js_name = X25519KeyAgreementKey2019)]
  pub fn x25519_key_agreement_key_2019() -> WasmMethodType {
    WasmMethodType(MethodType::X25519KeyAgreementKey2019)
  }

  /// Serializes a `MethodType` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `MethodType` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmMethodType> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl From<WasmMethodType> for MethodType {
  fn from(wasm_method_type: WasmMethodType) -> Self {
    wasm_method_type.0
  }
}

impl From<MethodType> for WasmMethodType {
  fn from(method_type: MethodType) -> Self {
    WasmMethodType(method_type)
  }
}

impl_wasm_clone!(WasmMethodType, MethodType);
