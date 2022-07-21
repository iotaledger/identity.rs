// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::account_storage::CekAlgorithm;
use wasm_bindgen::prelude::*;

use crate::account::types::WasmAgreementInfo;

/// Supported algorithms used to determine and potentially encrypt the content encryption key (CEK).
#[wasm_bindgen(js_name = CekAlgorithm, inspectable)]
#[derive(Clone)]
pub struct WasmCekAlgorithm(CekAlgorithm);

#[wasm_bindgen(js_class = CekAlgorithm)]
impl WasmCekAlgorithm {
  /// Elliptic Curve Diffie-Hellman Ephemeral Static key agreement using Concat KDF.
  #[wasm_bindgen(js_name = EcdhEs)]
  pub fn ecdh_es(agreement: &WasmAgreementInfo) -> WasmCekAlgorithm {
    Self(CekAlgorithm::ECDH_ES(agreement.clone().into()))
  }

  /// Elliptic Curve Diffie-Hellman Ephemeral Static key agreement using Concat KDF.
  #[wasm_bindgen(js_name = EcdhEsA256Kw)]
  pub fn ecdh_es_a256kw(agreement: &WasmAgreementInfo) -> WasmCekAlgorithm {
    Self(CekAlgorithm::ECDH_ES_A256KW(agreement.clone().into()))
  }
}

impl_wasm_json!(WasmCekAlgorithm, CekAlgorithm);

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
