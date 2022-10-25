// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::MethodType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MethodType | undefined")]
  pub type OptionMethodType;
}

/// Supported verification method types.
#[wasm_bindgen(js_name = MethodType, inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmMethodType(pub(crate) MethodType);

#[wasm_bindgen(js_class = MethodType)]
impl WasmMethodType {
  #[wasm_bindgen(js_name = ed25519VerificationKey2018)]
  pub fn ed25519_verification_key_2018() -> WasmMethodType {
    WasmMethodType(MethodType::ED25519_VERIFICATION_KEY_2018)
  }

  #[wasm_bindgen(js_name = x25519KeyAgreementKey2019)]
  pub fn x25519_key_agreement_key_2019() -> WasmMethodType {
    WasmMethodType(MethodType::X25519_KEY_AGREEMENT_KEY_2019)
  }

  #[wasm_bindgen(js_name = fromString)]
  pub fn from_string(string: String) -> WasmMethodType {
    WasmMethodType(MethodType::from(string))
  }

  /// Returns the `MethodType` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    // Prevents the automatically derived toString which adds quotation marks due to using toJSON.
    self.0.to_string()
  }
}

impl_wasm_json!(WasmMethodType, MethodType);
impl_wasm_clone!(WasmMethodType, MethodType);

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
