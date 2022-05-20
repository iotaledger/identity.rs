// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account_storage::CekAlgorithm;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::account::types::WasmAgreementInfo;
use crate::error::Result;
use crate::error::WasmResult;

/// Supported CEK algorithms
#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen(js_name = CekAlgorithm, inspectable)]
pub struct WasmCekAlgorithm(CekAlgorithm);

#[wasm_bindgen(js_class = CekAlgorithm)]
impl WasmCekAlgorithm {
  /// ECDH-ES will be used as the content encryption key.
  #[wasm_bindgen(js_name = EcdhEs)]
  pub fn ecdh_es(agreement: &WasmAgreementInfo) -> WasmCekAlgorithm {
    Self(CekAlgorithm::ECDH_ES {
      agreement: agreement.clone().into(),
    })
  }

  /// Serializes `CEKAlgorithm` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `CEKAlgorithm` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmCekAlgorithm> {
    json_value.into_serde().map(Self).wasm_result()
  }
}

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
