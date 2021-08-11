// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::MessageHistory;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::chain::{WasmDiffChain, WasmIntegrationChain};
use crate::error::wasm_error;

#[wasm_bindgen(js_name = MessageHistory, inspectable)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WasmMessageHistory(MessageHistory);

#[wasm_bindgen(js_class = MessageHistory)]
impl WasmMessageHistory {
  /// Returns the integration chain.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(getter = intChainData)]
  pub fn int_chain_data(&self) -> WasmIntegrationChain {
    WasmIntegrationChain::from(self.0.int_chain_data.clone())
  }

  /// Returns a [`js_sys::Array`] of message id strings for the spam messages on the same index
  /// as the integration chain but do not map to a valid DID document.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(getter = intChainSpam)]
  pub fn int_chain_spam(&self) -> js_sys::Array {
    self
      .0
      .int_chain_spam
      .iter()
      .cloned()
      .map(|mid| mid.to_string())
      .map(JsValue::from)
      .collect()
  }

  /// Returns the diff chain.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(getter = diffChainData)]
  pub fn diff_chain_data(&self) -> WasmDiffChain {
    WasmDiffChain::from(self.0.diff_chain_data.clone())
  }

  /// Returns a [`js_sys::Array`] of message id strings for the spam messages on the same index
  /// as the integration chain but do not map to a valid DID document.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(getter = diffChainSpam)]
  pub fn diff_chain_spam(&self) -> js_sys::Array {
    self
      .0
      .diff_chain_spam
      .iter()
      .cloned()
      .map(|mid| mid.to_string())
      .map(JsValue::from)
      .collect()
  }

  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(wasm_error)
  }

  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmMessageHistory, JsValue> {
    value.into_serde().map_err(wasm_error).map(Self)
  }
}

impl From<MessageHistory> for WasmMessageHistory {
  fn from(message_history: MessageHistory) -> Self {
    Self(message_history)
  }
}
