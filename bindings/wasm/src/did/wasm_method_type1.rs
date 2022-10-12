// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_storage::MethodType1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MethodType1 | undefined")]
  pub type OptionMethodType1;
}

/// Supported verification method types.
#[wasm_bindgen(js_name = MethodType1, inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmMethodType1(pub(crate) MethodType1);

#[wasm_bindgen(js_class = MethodType1)]
impl WasmMethodType1 {
  #[wasm_bindgen(js_name = ed25519VerificationKey2018)]
  pub fn ed25519_verification_key_2018() -> WasmMethodType1 {
    WasmMethodType1(MethodType1::ed25519_verification_key_2018())
  }

  #[wasm_bindgen(js_name = x25519KeyAgreementKey2019)]
  pub fn x25519_key_agreement_key_2019() -> WasmMethodType1 {
    WasmMethodType1(MethodType1::x25519_verification_key_2018())
  }

  #[wasm_bindgen(js_name = fromString)]
  pub fn from_string(string: String) -> WasmMethodType1 {
    WasmMethodType1(MethodType1::from(string))
  }

  /// Returns the `MethodType` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    // Prevents the automatically derived toString which adds quotation marks due to using toJSON.
    self.0.to_string()
  }
}

impl_wasm_json!(WasmMethodType1, MethodType1);
impl_wasm_clone!(WasmMethodType1, MethodType1);

impl From<WasmMethodType1> for MethodType1 {
  fn from(wasm_method_type: WasmMethodType1) -> Self {
    wasm_method_type.0
  }
}

impl From<MethodType1> for WasmMethodType1 {
  fn from(method_type: MethodType1) -> Self {
    WasmMethodType1(method_type)
  }
}
