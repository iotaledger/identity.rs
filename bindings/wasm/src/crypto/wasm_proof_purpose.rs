// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::crypto::ProofPurpose;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Associates a purpose with a {@link Signature}.
///
/// See https://w3c-ccg.github.io/security-vocab/#proofPurpose
#[wasm_bindgen(js_name = ProofPurpose, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmProofPurpose(pub(crate) ProofPurpose);

#[wasm_bindgen(js_class = ProofPurpose)]
impl WasmProofPurpose {
  /// Purpose is to assert a claim.
  /// See https://www.w3.org/TR/did-core/#assertion
  #[wasm_bindgen(js_name = assertionMethod)]
  pub fn assertion_method() -> WasmProofPurpose {
    WasmProofPurpose(ProofPurpose::AssertionMethod)
  }

  /// Purpose is to authenticate the signer.
  /// See https://www.w3.org/TR/did-core/#authentication
  #[wasm_bindgen(js_name = authentication)]
  pub fn authentication() -> WasmProofPurpose {
    WasmProofPurpose(ProofPurpose::Authentication)
  }

  /// Serializes a `ProofPurpose` to a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `ProofPurpose` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmProofPurpose> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmProofPurpose, ProofPurpose);

impl From<ProofPurpose> for WasmProofPurpose {
  fn from(purpose: ProofPurpose) -> Self {
    WasmProofPurpose(purpose)
  }
}
