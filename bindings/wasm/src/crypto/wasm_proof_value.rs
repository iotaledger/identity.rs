// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::ProofValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ProofValue, inspectable)]
#[derive(Serialize, Deserialize)]
pub struct WasmProofValue(pub(crate) ProofValue);

#[wasm_bindgen(js_class = ProofValue)]
impl WasmProofValue {
  #[wasm_bindgen(js_name = None)]
  pub fn none() -> WasmProofValue {
    Self(ProofValue::None)
  }

  #[wasm_bindgen(js_name = Jws)]
  pub fn jws(signature: String) -> WasmProofValue {
    Self(ProofValue::Jws(signature))
  }

  #[wasm_bindgen(js_name = Proof)]
  pub fn proof(signature: String) -> WasmProofValue {
    Self(ProofValue::Proof(signature))
  }

  #[wasm_bindgen(js_name = Signature)]
  pub fn signature(signature: String) -> WasmProofValue {
    Self(ProofValue::Signature(signature))
  }
}

impl From<ProofValue> for WasmProofValue {
  fn from(proof_value: ProofValue) -> Self {
    WasmProofValue(proof_value)
  }
}

impl From<WasmProofValue> for ProofValue {
  fn from(proof_value: WasmProofValue) -> Self {
    proof_value.0
  }
}

impl_wasm_json!(WasmProofValue, ProofValue);

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<ProofValue>")]
  pub type PromiseProofValue;
}
