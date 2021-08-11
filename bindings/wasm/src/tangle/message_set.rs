// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::DiffSet;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::did::WasmDocumentDiff;
use crate::error::{Result, WasmResult};

/// List of [`DocumentDiff`] messages forming a diff chain.
///
/// Retains a list of "spam" messages that are valid but do not form part of the resulting chain.
#[wasm_bindgen(js_name = DiffSet, inspectable)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WasmDiffSet(DiffSet);

#[wasm_bindgen(js_class = DiffSet)]
impl WasmDiffSet {
  /// Returns a [`js_sys::Array`] of [`WasmDocumentDiffs`](WasmDocumentDiff) forming a diff chain.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(getter = data)]
  pub fn data(&self) -> Array {
    self
      .0
      .data()
      .iter()
      .cloned()
      .map(WasmDocumentDiff::from)
      .map(JsValue::from)
      .collect()
  }

  /// Returns a [`js_sys::Array`] of spam message ids on the same index but not forming part of the
  /// diff chain.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(getter = spam)]
  pub fn spam(&self) -> Array {
    self
      .0
      .spam()
      .iter()
      .cloned()
      .map(|mid| mid.to_string())
      .map(JsValue::from)
      .collect()
  }

  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmDiffSet> {
    value.into_serde().map(Self).wasm_result()
  }
}

impl From<DiffSet> for WasmDiffSet {
  fn from(diff_set: DiffSet) -> Self {
    Self(diff_set)
  }
}
