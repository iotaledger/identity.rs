// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use identity::iota::IntegrationChain;
use wasm_bindgen::prelude::*;

use crate::did::WasmDocument;
use crate::error::{Result, WasmResult};

#[wasm_bindgen(js_name = IntegrationChain, inspectable)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WasmIntegrationChain(IntegrationChain);

#[wasm_bindgen(js_class = IntegrationChain)]
impl WasmIntegrationChain {
  /// Returns the latest [`WasmDocument`].
  ///
  /// NOTE: this clones the data.
  #[wasm_bindgen]
  pub fn current(&self) -> WasmDocument {
    WasmDocument::from(self.0.current().clone())
  }

  /// Returns the message id of the latest [`WasmDocument`].
  #[wasm_bindgen(js_name = currentMessageId)]
  pub fn current_message_id(&self) -> String {
    self.0.current_message_id().to_string()
  }

  /// Returns an array of documents in the integration chain (excluding the current document).
  ///
  /// NOTE: this clones the data.
  #[wasm_bindgen]
  pub fn history(&self) -> js_sys::Array {
    self
      .0
      .history()
      .map(|documents| {
        documents
          .iter()
          .cloned()
          .map(WasmDocument::from)
          .map(JsValue::from)
          .collect::<Vec<_>>()
      })
      .unwrap_or_default()
      .into_iter()
      .collect()
  }

  /// Tries to append the document to this integration chain.
  #[wasm_bindgen(js_name = tryPush)]
  pub fn try_push(&mut self, document: WasmDocument) -> Result<()> {
    self.0.try_push(document.0).wasm_result()
  }

  /// Returns whether the document may be appended to this integration chain.
  #[wasm_bindgen(js_name = isValidAddition)]
  pub fn is_valid_addition(&self, document: &WasmDocument) -> bool {
    self.0.is_valid_addition(&document.0)
  }

  /// Checks whether the document may be appended to this integration chain.
  #[wasm_bindgen(js_name = checkValidAddition)]
  pub fn check_valid_addition(&self, document: &WasmDocument) -> Result<()> {
    self.0.check_valid_addition(&document.0).wasm_result()
  }

  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmIntegrationChain> {
    value.into_serde().map(Self).wasm_result()
  }
}

impl From<IntegrationChain> for WasmIntegrationChain {
  fn from(integration_chain: IntegrationChain) -> Self {
    Self(integration_chain)
  }
}

impl Deref for WasmIntegrationChain {
  type Target = IntegrationChain;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
