// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_storage::SigningAlgorithm;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = SigningAlgorithm, inspectable)]
pub struct WasmSigningAlgorithm(pub(crate) SigningAlgorithm);

#[wasm_bindgen(js_class = WasmSigningAlgorithm)]
impl WasmSigningAlgorithm {
  #[wasm_bindgen(js_name = EcdhEs)]
  pub fn ecdh_es(agreement: &WasmAgreementInfo) -> WasmCekAlgorithm {
    Self(SigningAlgorithm::Ed25519 )
  }

  #[wasm_bindgen(js_name = EcdhEsA256Kw)]
  pub fn ecdh_es_a256kw(agreement: &WasmAgreementInfo) -> WasmCekAlgorithm {
    Self(CekAlgorithm::ECDH_ES_A256KW(agreement.0.clone()))
  }
}

// impl_wasm_json!(WasmCekAlgorithm, CekAlgorithm);

impl From<WasmCekAlgorithm> for CekAlgorithm {
  fn from(wasm_cek_algorithm: WasmCekAlgorithm) -> Self {
    wasm_cek_algorithm.0
  }
}

impl From<CekAlgorithm> for WasmCekAlgorithm {
  fn from(cek_algorithm: CekAlgorithm) -> Self {
    WasmCekAlgorithm(cek_algorithm)
  }
}
