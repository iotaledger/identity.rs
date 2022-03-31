// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::KeyLocation;
use wasm_bindgen::prelude::*;

use crate::crypto::WasmKeyType;
use crate::did::WasmVerificationMethod;
use crate::error::Result;
use crate::error::WasmResult;

/// The storage location of a verification method key.
///
/// A key is uniquely identified by the fragment and a hash of its public key.
/// Importantly, the fragment alone is insufficient to represent the storage location.
/// For example, when rotating a key, there will be two keys in storage for the
/// same identity with the same fragment. The `key_hash` disambiguates the keys in
/// situations like these.
///
/// The string representation of that location can be obtained via `canonicalRepr`.
#[derive(Debug)]
#[wasm_bindgen(js_name = KeyLocation, inspectable)]
pub struct WasmKeyLocation(pub(crate) KeyLocation);

#[wasm_bindgen(js_class = KeyLocation)]
impl WasmKeyLocation {
  /// Create a location from a `KeyType`, the fragment of a verification method
  /// and the bytes of a public key.
  #[allow(non_snake_case)]
  pub fn new(keyType: WasmKeyType, fragment: String, publicKey: Vec<u8>) -> WasmKeyLocation {
    WasmKeyLocation(KeyLocation::new(keyType.into(), fragment, publicKey.as_ref()))
  }

  /// Obtain the location of a verification method's key in storage.
  #[wasm_bindgen(js_name = fromVerificationMethod)]
  pub fn from_verification_method(method: &WasmVerificationMethod) -> Result<WasmKeyLocation> {
    KeyLocation::from_verification_method(&method.0).map(WasmKeyLocation).wasm_result()
  }

  /// Returns the canonical string representation of the location.
  ///
  /// This should be used as the representation for storage keys.
  #[wasm_bindgen]
  pub fn canonical(&self) -> String {
    self.0.canonical()
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
