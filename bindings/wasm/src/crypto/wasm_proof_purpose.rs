// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::ProofPurpose;
use wasm_bindgen::prelude::*;

/// Associates a purpose with a {@link Proof}.
///
/// See https://w3c-ccg.github.io/security-vocab/#proofPurpose
#[wasm_bindgen(js_name = ProofPurpose, inspectable)]
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
}

impl_wasm_json!(WasmProofPurpose, ProofPurpose);
impl_wasm_clone!(WasmProofPurpose, ProofPurpose);

impl From<ProofPurpose> for WasmProofPurpose {
  fn from(purpose: ProofPurpose) -> Self {
    WasmProofPurpose(purpose)
  }
}
