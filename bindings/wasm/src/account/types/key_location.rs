// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::KeyLocation;
use wasm_bindgen::prelude::*;

use crate::crypto::WasmKeyType;
use crate::did::WasmVerificationMethod;
use crate::error::Result;
use crate::error::WasmResult;

// TODO: Documentation.
#[derive(Debug)]
#[wasm_bindgen(js_name = KeyLocation, inspectable)]
pub struct WasmKeyLocation(pub(crate) KeyLocation);

#[wasm_bindgen(js_class = KeyLocation)]
impl WasmKeyLocation {
  #[wasm_bindgen(constructor)]
  pub fn new(key_type: WasmKeyType, fragment: String, public_key: Vec<u8>) -> WasmKeyLocation {
    WasmKeyLocation(KeyLocation::new(key_type.into(), fragment, public_key.as_ref()))
  }

  #[wasm_bindgen(js_name = fromVerificationMethod)]
  pub fn from_verification_method(method: &WasmVerificationMethod) -> Result<WasmKeyLocation> {
    Ok(WasmKeyLocation(
      KeyLocation::from_verification_method(&method.0).wasm_result()?,
    ))
  }

  #[wasm_bindgen(js_name = canonicalRepr)]
  pub fn canonical_repr(&self) -> String {
    self.0.canonical_repr()
  }

  /// Returns a copy of the key type of the key location.
  #[wasm_bindgen(js_name = keyType)]
  pub fn key_type(&self) -> WasmKeyType {
    self.0.key_type.into()
  }

  /// Serializes `KeyLocation` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a JSON object into a `KeyLocation`.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmKeyLocation> {
    json_value.into_serde().map(Self).wasm_result()
  }

  #[wasm_bindgen(js_name = toString)]
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl From<WasmKeyLocation> for KeyLocation {
  fn from(wasm_key_location: WasmKeyLocation) -> Self {
    wasm_key_location.0
  }
}

impl From<KeyLocation> for WasmKeyLocation {
  fn from(wasm_key_location: KeyLocation) -> Self {
    WasmKeyLocation(wasm_key_location)
  }
}
