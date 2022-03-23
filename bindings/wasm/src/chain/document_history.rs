// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::ChainHistory;
use identity::iota::DocumentHistory;
use identity::iota::ResolvedIotaDocument;
use identity::iota_core::DiffMessage;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::did::WasmDiffMessage;
use crate::did::WasmResolvedDocument;
use crate::error::Result;
use crate::error::WasmResult;

/// A DID Document's history and current state.
#[wasm_bindgen(js_name = DocumentHistory, inspectable)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WasmDocumentHistory(DocumentHistory);

// Workaround for Typescript type annotations on async function returns and arrays.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<DocumentHistory>")]
  pub type PromiseDocumentHistory;

  #[wasm_bindgen(typescript_type = "Promise<IntegrationChainHistory>")]
  pub type PromiseIntegrationChainHistory;

  #[wasm_bindgen(typescript_type = "Promise<DiffChainHistory>")]
  pub type PromiseDiffChainHistory;

  #[wasm_bindgen(typescript_type = "Array<string>")]
  pub type ArrayString;

  #[wasm_bindgen(typescript_type = "Array<ResolvedDocument>")]
  pub type ArrayResolvedDocument;

  #[wasm_bindgen(typescript_type = "Array<DiffMessage>")]
  pub type ArrayDiffMessage;
}

#[wasm_bindgen(js_class = DocumentHistory)]
impl WasmDocumentHistory {
  /// Returns an `Array` of integration chain `Documents`.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(js_name = integrationChainData)]
  pub fn integration_chain_data(&self) -> ArrayResolvedDocument {
    self
      .0
      .integration_chain_data
      .iter()
      .cloned()
      .map(WasmResolvedDocument::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayResolvedDocument>()
  }

  /// Returns an `Array` of message id strings for "spam" messages on the same index
  /// as the integration chain.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(js_name = integrationChainSpam)]
  pub fn integration_chain_spam(&self) -> ArrayString {
    self
      .0
      .integration_chain_spam
      .iter()
      .cloned()
      .map(|message_id| message_id.to_string())
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }

  /// Returns an `Array` of diff chain `DiffMessages`.
  ///
  /// NOTE: clones the data.
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen(js_name = diffChainData)]
  pub fn diff_chain_data(&self) -> ArrayDiffMessage {
    self
      .0
      .diff_chain_data
      .iter()
      .cloned()
      .map(WasmDiffMessage::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayDiffMessage>()
  }

  /// Returns an `Array` of message id strings for "spam" messages on the same index
  /// as the diff chain.
  ///
  /// NOTE: clones the data.
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen(js_name = diffChainSpam)]
  pub fn diff_chain_spam(&self) -> ArrayString {
    self
      .0
      .diff_chain_spam
      .iter()
      .cloned()
      .map(|message_id| message_id.to_string())
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }

  /// Serializes `DocumentHistory` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `DocumentHistory` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmDocumentHistory> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmDocumentHistory, DocumentHistory);

impl From<DocumentHistory> for WasmDocumentHistory {
  fn from(document_history: DocumentHistory) -> Self {
    Self(document_history)
  }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntegrationChainHistory(ChainHistory<ResolvedIotaDocument>);

/// @deprecated since 0.5.0, diff chain features are slated for removal.
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiffChainHistory(ChainHistory<DiffMessage>);

#[wasm_bindgen]
impl IntegrationChainHistory {
  /// Returns an `Array` of the integration chain `Documents`.
  ///
  /// NOTE: this clones the field.
  #[wasm_bindgen(js_name = chainData)]
  pub fn chain_data(&self) -> ArrayResolvedDocument {
    self
      .0
      .chain_data
      .iter()
      .cloned()
      .map(WasmResolvedDocument::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayResolvedDocument>()
  }
}

#[wasm_bindgen]
impl DiffChainHistory {
  /// Returns an `Array` of the diff chain `DiffMessages`.
  ///
  /// NOTE: this clones the field.
  #[wasm_bindgen(js_name = chainData)]
  pub fn chain_data(&self) -> ArrayDiffMessage {
    self
      .0
      .chain_data
      .iter()
      .cloned()
      .map(WasmDiffMessage::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayDiffMessage>()
  }
}

macro_rules! impl_wasm_chain_history {
  ($ident:ident, $ty:ty, $wasm_ty:ty) => {
    #[wasm_bindgen]
    impl $ident {
      /// Returns an `Array` of `MessageIds` as strings.
      ///
      /// NOTE: this clones the field.
      #[wasm_bindgen]
      pub fn spam(&self) -> ArrayString {
        self
          .0
          .spam
          .iter()
          .cloned()
          .map(|message_id| message_id.to_string())
          .map(JsValue::from)
          .collect::<js_sys::Array>()
          .unchecked_into::<ArrayString>()
      }

      /// Serializes as a JSON object.
      #[wasm_bindgen(js_name = toJSON)]
      pub fn to_json(&self) -> Result<JsValue> {
        JsValue::from_serde(&self.0).wasm_result()
      }

      /// Deserializes from a JSON object.
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

impl_wasm_chain_history!(IntegrationChainHistory, ResolvedIotaDocument, WasmResolvedDocument);
impl_wasm_chain_history!(DiffChainHistory, DiffMessage, WasmDiffMessage);
