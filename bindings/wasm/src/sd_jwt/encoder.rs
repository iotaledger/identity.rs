// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::disclosure::WasmDisclosure;
use crate::{
  common::{ArrayString, RecordStringAny},
  error::{Result, WasmResult},
};
use identity_iota::sd_jwt_payload::{SdObjectEncoder, Sha256Hasher};
use js_sys::{Array, JsString};
use serde_json::Value;
use wasm_bindgen::prelude::*;

/// Transforms a JSON object into an SD-JWT object by substituting selected values
/// with their corresponding disclosure digests.
#[wasm_bindgen(js_name = SdObjectEncoder, inspectable)]
pub struct WasmSdObjectEncoder(pub(crate) SdObjectEncoder<Sha256Hasher>);

#[wasm_bindgen(js_class = SdObjectEncoder)]
impl WasmSdObjectEncoder {
  /// Creates a new `SdObjectEncoder` with `sha-256` hash function.
  #[wasm_bindgen(constructor)]
  pub fn new(object: &JsValue) -> Result<WasmSdObjectEncoder> {
    let object: Value = object.into_serde().wasm_result()?;
    Ok(Self(SdObjectEncoder::try_from_serializable(object).wasm_result()?))
  }

  /// Substitutes a value with the digest of its disclosure.
  /// If no salt is provided, the disclosure will be created with a random salt value.
  ///
  /// The value of the key specified in `path` will be concealed. E.g. for path
  /// `["claim", "subclaim"]` the value of `claim.subclaim` will be concealed.
  ///
  /// ## Error
  /// `InvalidPath` if path is invalid or the path slice is empty.
  /// `DataTypeMismatch` if existing SD format is invalid.
  ///
  /// ## Note
  /// Use `concealArrayEntry` for values in arrays.
  #[wasm_bindgen(js_name = conceal)]
  pub fn conceal(&mut self, path: ArrayString, salt: Option<String>) -> Result<WasmDisclosure> {
    let path: Vec<String> = path
      .dyn_into::<Array>()?
      .iter()
      .map(|item| item.dyn_into::<JsString>().map(String::from))
      .collect::<Result<Vec<String>>>()?;
    let path: Vec<&str> = path.iter().map(|s| &**s).collect();
    let disclosure = self.0.conceal(&path, salt).wasm_result()?;
    Ok(WasmDisclosure(disclosure))
  }

  /// Substitutes a value within an array with the digest of its disclosure.
  /// If no salt is provided, the disclosure will be created with random salt value.
  ///
  /// `path` is used to specify the array in the object, while `element_index` specifies
  /// the index of the element to be concealed (index start at 0).
  ///
  /// ## Error
  /// `InvalidPath` if path is invalid or the path slice is empty.
  /// `DataTypeMismatch` if existing SD format is invalid.
  /// `IndexOutofBounds` if `element_index` is out of bounds.
  #[wasm_bindgen(js_name = concealArrayEntry)]
  pub fn conceal_array_entry(
    &mut self,
    path: ArrayString,
    element_index: usize,
    salt: Option<String>,
  ) -> Result<WasmDisclosure> {
    let path: Vec<String> = path
      .dyn_into::<Array>()?
      .iter()
      .map(|item| item.dyn_into::<JsString>().map(String::from))
      .collect::<Result<Vec<String>>>()?;
    let path: Vec<&str> = path.iter().map(|s| &**s).collect();
    let disclosure = self.0.conceal_array_entry(&path, element_index, salt).wasm_result()?;
    Ok(WasmDisclosure(disclosure))
  }

  /// Adds the `_sd_alg` property to the top level of the object.
  /// The value is taken from the [`crate::Hasher::alg_name`] implementation.
  #[wasm_bindgen(js_name = addSdAlgProperty)]
  pub fn add_sd_alg_property(&mut self) {
    self.0.add_sd_alg_property();
  }

  /// Returns the modified object as a string.
  #[wasm_bindgen(js_name = encodeToString)]
  pub fn encoded_to_string(&self) -> Result<String> {
    self.0.try_to_string().wasm_result()
  }

  /// Returns the modified object as a string.
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> Result<String> {
    self.0.try_to_string().wasm_result()
  }

  /// Returns the modified object.
  #[wasm_bindgen(js_name = encodeToObject)]
  pub fn encode_to_object(&self) -> Result<RecordStringAny> {
    Ok(
      JsValue::from_serde(&self.0.object())
        .wasm_result()?
        .unchecked_into::<RecordStringAny>(),
    )
  }

  /// Returns the modified object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0.object()).wasm_result()
  }

  /// Adds a decoy digest to the specified path.
  /// If path is an empty slice, decoys will be added to the top level.
  #[wasm_bindgen(js_name = addDecoys)]
  pub fn add_decoys(&mut self, path: ArrayString, number_of_decoys: usize) -> Result<()> {
    let path: Vec<String> = path
      .dyn_into::<Array>()?
      .iter()
      .map(|item| item.dyn_into::<JsString>().map(String::from))
      .collect::<Result<Vec<String>>>()?;
    let path: Vec<&str> = path.iter().map(|s| &**s).collect();
    self.0.add_decoys(&path, number_of_decoys).wasm_result()?;
    Ok(())
  }
}
