// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::{ChainHistory, DocumentDiff, DocumentHistory, IotaDocument};
use wasm_bindgen::prelude::*;

use crate::did::{WasmDocument, WasmDocumentDiff};
use crate::error::{Result, WasmResult};

/// A DID Document's history and current state.
#[wasm_bindgen(js_name = DocumentHistory, inspectable)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WasmDocumentHistory(DocumentHistory);

#[wasm_bindgen(js_class = DocumentHistory)]
impl WasmDocumentHistory {
  /// Returns a [`js_sys::Array`] of integration chain [`WasmDocuments`](WasmDocument).
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(js_name = integrationChainData)]
  pub fn integration_chain_data(&self) -> js_sys::Array {
    self
      .0
      .integration_chain_data
      .iter()
      .cloned()
      .map(WasmDocument::from)
      .map(JsValue::from)
      .collect()
  }

  /// Returns a [`js_sys::Array`] of message id strings for "spam" messages on the same index
  /// as the integration chain.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(js_name = integrationChainSpam)]
  pub fn integration_chain_spam(&self) -> js_sys::Array {
    self
      .0
      .integration_chain_spam
      .iter()
      .cloned()
      .map(|message_id| message_id.to_string())
      .map(JsValue::from)
      .collect()
  }

  /// Returns a [`js_sys::Array`] of diff chain [`WasmDocumentDiffs`](WasmDocumentDiff).
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(js_name = diffChainData)]
  pub fn diff_chain_data(&self) -> js_sys::Array {
    self
      .0
      .diff_chain_data
      .iter()
      .cloned()
      .map(WasmDocumentDiff::from)
      .map(JsValue::from)
      .collect()
  }

  /// Returns a [`js_sys::Array`] of message id strings for "spam" messages on the same index
  /// as the diff chain.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(js_name = diffChainSpam)]
  pub fn diff_chain_spam(&self) -> js_sys::Array {
    self
      .0
      .diff_chain_spam
      .iter()
      .cloned()
      .map(|message_id| message_id.to_string())
      .map(JsValue::from)
      .collect()
  }

  /// Serializes a [`WasmDocumentHistory`] object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a [`WasmDocumentHistory`] object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmDocumentHistory> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl From<DocumentHistory> for WasmDocumentHistory {
  fn from(document_history: DocumentHistory) -> Self {
    Self(document_history)
  }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntegrationChainHistory(ChainHistory<IotaDocument>);

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiffChainHistory(ChainHistory<DocumentDiff>);

macro_rules! impl_wasm_chain_history {
  ($ident:ident, $ty:ty, $wasm_ty:ty) => {
    #[wasm_bindgen]
    impl $ident {
      /// Returns a [`js_sys::Array`] of `$wasm_ty` as strings.
      ///
      /// NOTE: this clones the field.
      #[wasm_bindgen(js_name = chainData)]
      pub fn chain_data(&self) -> js_sys::Array {
        self
          .0
          .chain_data
          .iter()
          .cloned()
          .map(<$wasm_ty>::from)
          .map(JsValue::from)
          .collect()
      }

      /// Returns a [`js_sys::Array`] of [`MessageIds`][MessageId] as strings.
      ///
      /// NOTE: this clones the field.
      #[wasm_bindgen]
      pub fn spam(&self) -> js_sys::Array {
        self
          .0
          .spam
          .iter()
          .cloned()
          .map(|message_id| message_id.to_string())
          .map(JsValue::from)
          .collect()
      }

      /// Serializes a `$ident` object as a JSON object.
      #[wasm_bindgen(js_name = toJSON)]
      pub fn to_json(&self) -> Result<JsValue> {
        JsValue::from_serde(&self.0).wasm_result()
      }

      /// Deserializes a `$ident` object from a JSON object.
      #[wasm_bindgen(js_name = fromJSON)]
      pub fn from_json(json: &JsValue) -> Result<$ident> {
        json.into_serde().map(Self).wasm_result()
      }
    }

    impl From<ChainHistory<$ty>> for $ident {
      fn from(chain_history: ChainHistory<$ty>) -> Self {
        $ident(chain_history)
      }
    }
  };
}

impl_wasm_chain_history!(IntegrationChainHistory, IotaDocument, WasmDocument);
impl_wasm_chain_history!(DiffChainHistory, DocumentDiff, WasmDocumentDiff);
