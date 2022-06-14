// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::Proof;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::common::WasmTimestamp;
use crate::crypto::WasmProofPurpose;
use crate::error::Result;
use crate::error::WasmResult;

/// A digital signature.
///
/// For field definitions see: https://w3c-ccg.github.io/security-vocab/
#[wasm_bindgen(js_name = Proof, inspectable)]
pub struct WasmProof(pub(crate) Proof);

#[wasm_bindgen(js_class = Proof)]
impl WasmProof {
  /// Returns a copy of the proof type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> String {
    self.0.type_().to_owned()
  }

  /// Returns a copy of the proof value string.
  #[wasm_bindgen]
  pub fn value(&self) -> String {
    self.0.value().as_str().to_owned()
  }

  /// Returns a copy of the identifier of the DID method used to create this proof.
  #[wasm_bindgen(js_name = verificationMethod)]
  pub fn verification_method(&self) -> String {
    self.0.verification_method().to_owned()
  }

  /// When the proof was generated.
  #[wasm_bindgen]
  pub fn created(&self) -> Option<WasmTimestamp> {
    self.0.created.map(WasmTimestamp::from)
  }

  /// When the proof expires.
  #[wasm_bindgen]
  pub fn expires(&self) -> Option<WasmTimestamp> {
    self.0.expires.map(WasmTimestamp::from)
  }

  /// Challenge from a proof requester to mitigate replay attacks.
  #[wasm_bindgen]
  pub fn challenge(&self) -> Option<String> {
    self.0.challenge.clone()
  }

  /// Domain for which a proof is valid to mitigate replay attacks.
  #[wasm_bindgen]
  pub fn domain(&self) -> Option<String> {
    self.0.domain.clone()
  }

  /// Purpose for which the proof was generated.
  #[wasm_bindgen]
  pub fn purpose(&self) -> Option<WasmProofPurpose> {
    self.0.purpose.map(WasmProofPurpose::from)
  }

  /// Serializes a `Proof` to a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Proof` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmProof> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmProof, Proof);

impl From<Proof> for WasmProof {
  fn from(proof: Proof) -> Self {
    WasmProof(proof)
  }
}

impl From<WasmProof> for Proof {
  fn from(proof: WasmProof) -> Self {
    proof.0
  }
}
