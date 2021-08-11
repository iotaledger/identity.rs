// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use identity::iota::{DiffChain, DocumentDiff};
use wasm_bindgen::prelude::*;

use crate::chain::WasmIntegrationChain;
use crate::did::WasmDocumentDiff;
use crate::error::{Result, WasmResult};

#[wasm_bindgen(js_name = DiffChain, inspectable)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WasmDiffChain(DiffChain);

#[wasm_bindgen(js_class = DiffChain)]
impl WasmDiffChain {
  /// Returns the total number of diffs.
  #[wasm_bindgen]
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Returns `true` if the [`DiffChain`] is empty.
  #[wasm_bindgen(js_name = isEmpty)]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Empties the [`DiffChain`], removing all diffs.
  #[wasm_bindgen]
  pub fn clear(&mut self) {
    self.0.clear();
  }

  /// Returns the [`MessageId`] of the latest diff in the chain, if any.
  #[wasm_bindgen(js_name = currentMessageId)]
  pub fn current_message_id(&self) -> Option<String> {
    self.0.current_message_id().map(ToString::to_string)
  }

  /// Adds a new diff to the [`DiffChain`].
  ///
  /// # Errors
  ///
  /// Fails if the diff signature is invalid or the Tangle message
  /// references within the diff are invalid.
  #[wasm_bindgen(js_name = tryPush)]
  pub fn try_push(&mut self, integration_chain: &WasmIntegrationChain, diff: WasmDocumentDiff) -> Result<()> {
    self.0.try_push(integration_chain.deref(), diff.0).wasm_result()
  }

  /// Returns `true` if the [`DocumentDiff`] can be added to the [`DiffChain`].
  #[wasm_bindgen(js_name = isValidAddition)]
  pub fn is_valid_addition(&self, integration_chain: &WasmIntegrationChain, diff: &WasmDocumentDiff) -> bool {
    self.0.is_valid_addition(integration_chain, diff.deref())
  }

  /// Checks if the [`DocumentDiff`] can be added to the [`DiffChain`].
  ///
  /// # Errors
  ///
  /// Fails if the [`DocumentDiff`] is not a valid addition.
  #[wasm_bindgen(js_name = checkValidAddition)]
  pub fn check_valid_addition(&self, integration_chain: &WasmIntegrationChain, diff: &WasmDocumentDiff) -> Result<()> {
    self
      .0
      .check_valid_addition(integration_chain, diff.deref())
      .wasm_result()
  }

  /// Converts the chain into a [`js_sys::Array`] of [`WasmDocumentDiffs`](WasmDocumentDiff).
  #[wasm_bindgen(js_name = intoArray)]
  pub fn into_array(self) -> js_sys::Array {
    let inner: Vec<DocumentDiff> = Vec::from(self.0);
    inner
      .into_iter()
      .map(WasmDocumentDiff::from)
      .map(JsValue::from)
      .collect()
  }

  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmDiffChain> {
    value.into_serde().map(Self).wasm_result()
  }
}

impl From<DiffChain> for WasmDiffChain {
  fn from(diff_chain: DiffChain) -> Self {
    Self(diff_chain)
  }
}

impl Deref for WasmDiffChain {
  type Target = DiffChain;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
