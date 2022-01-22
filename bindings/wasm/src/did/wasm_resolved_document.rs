// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity::iota::IotaDocument;
use identity::iota::MessageId;
use identity::iota::ResolvedIotaDocument;
use identity::iota::TangleRef;
use wasm_bindgen::prelude::*;

use crate::did::WasmDiffMessage;
use crate::did::WasmDocument;
use crate::error::Result;
use crate::error::WasmResult;

/// An IOTA DID document resolved from the Tangle. Represents an integration chain message possibly
/// merged with one or more `DiffMessages`.
#[wasm_bindgen(js_name = ResolvedDocument, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmResolvedDocument {
  // Redefines fields manually to avoid having to clone the document.
  document: WasmDocument,
  integration_message_id: MessageId,
  diff_message_id: MessageId,
}

// Workaround for Typescript type annotations on async function returns.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<ResolvedDocument>")]
  pub type PromiseResolvedDocument;
}

#[wasm_bindgen(js_class = ResolvedDocument)]
impl WasmResolvedDocument {
  /// Attempts to merge changes from a `DiffMessage` into this document and
  /// updates the `ResolvedDocument::diffMessageId`.
  ///
  /// If merging fails the document remains unmodified, otherwise this represents
  /// the merged document state.
  ///
  /// See `Document::mergeDiff`.
  ///
  /// # Errors
  ///
  /// Fails if the merge operation or signature verification on the diff fails.
  #[wasm_bindgen(js_name = "mergeDiffMessage")]
  pub fn merge_diff_message(&mut self, diff_message: &WasmDiffMessage) -> Result<()> {
    self.document.merge_diff(diff_message)?;
    self.diff_message_id = *diff_message.0.message_id();
    Ok(())
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the inner DID document.
  #[wasm_bindgen(getter)]
  pub fn document(&self) -> WasmDocument {
    self.document.clone()
  }

  /// Returns the diff chain message id.
  #[wasm_bindgen(getter = diffMessageId)]
  pub fn diff_message_id(&self) -> String {
    self.diff_message_id.to_string()
  }

  /// Sets the diff chain message id.
  #[wasm_bindgen(setter = diffMessageId)]
  pub fn set_diff_message_id(&mut self, value: &str) -> Result<()> {
    let message_id: MessageId = MessageId::from_str(value).wasm_result()?;
    self.diff_message_id = message_id;
    Ok(())
  }

  /// Returns the integration chain message id.
  #[wasm_bindgen(getter = integrationMessageId)]
  pub fn integration_message_id(&self) -> String {
    self.integration_message_id.to_string()
  }

  /// Sets the integration chain message id.
  #[wasm_bindgen(setter = integrationMessageId)]
  pub fn set_integration_message_id(&mut self, value: &str) -> Result<()> {
    let message_id: MessageId = MessageId::from_str(value).wasm_result()?;
    self.integration_message_id = message_id;
    Ok(())
  }

  // ===========================================================================
  // JSON
  // ===========================================================================

  /// Serializes a `Document` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&ResolvedIotaDocument::from(self.clone())).wasm_result()
  }

  /// Deserializes a `Document` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmResolvedDocument> {
    json
      .into_serde::<ResolvedIotaDocument>()
      .map(WasmResolvedDocument::from)
      .wasm_result()
  }
}

impl From<ResolvedIotaDocument> for WasmResolvedDocument {
  fn from(
    ResolvedIotaDocument {
      document,
      integration_message_id,
      diff_message_id,
    }: ResolvedIotaDocument,
  ) -> Self {
    Self {
      document: WasmDocument::from(document),
      integration_message_id,
      diff_message_id,
    }
  }
}

impl From<WasmResolvedDocument> for ResolvedIotaDocument {
  fn from(
    WasmResolvedDocument {
      document,
      diff_message_id,
      integration_message_id,
    }: WasmResolvedDocument,
  ) -> Self {
    Self {
      document: IotaDocument::from(document),
      integration_message_id,
      diff_message_id,
    }
  }
}
