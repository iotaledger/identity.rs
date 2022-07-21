// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_iota::client::ResolvedIotaDocument;
use identity_iota::iota_core::MessageId;
use wasm_bindgen::prelude::*;

use crate::did::WasmDiffMessage;
use crate::did::WasmDocument;
use crate::error::Result;
use crate::error::WasmResult;

/// An IOTA DID document resolved from the Tangle. Represents an integration chain message possibly
/// merged with one or more `DiffMessages`.
#[wasm_bindgen(js_name = ResolvedDocument, inspectable)]
#[derive(Debug)]
pub struct WasmResolvedDocument(pub(crate) ResolvedIotaDocument);

#[wasm_bindgen]
extern "C" {
  // Workaround for Typescript type annotations on async function returns.
  #[wasm_bindgen(typescript_type = "Promise<ResolvedDocument>")]
  pub type PromiseResolvedDocument;

  #[wasm_bindgen(typescript_type = "Promise<Array<ResolvedDocument>>")]
  pub type PromiseArrayResolvedDocument;

  // Workaround for (current) lack of array support in wasm-bindgen
  #[wasm_bindgen(typescript_type = "Array<ResolvedDocument>")]
  pub type ArrayResolvedDocument;

  // Workaround for (current) lack of generics in wasm-bindgen
  #[wasm_bindgen(typescript_type = "Document | ResolvedDocument")]
  pub type DocumentOrResolvedDocument;

  #[wasm_bindgen(typescript_type = "Array<Document | ResolvedDocument>")]
  pub type ArrayDocumentOrResolvedDocument;
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
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen(js_name = "mergeDiffMessage")]
  pub fn merge_diff_message(&mut self, diff_message: &WasmDiffMessage) -> Result<()> {
    self.0.merge_diff_message(&diff_message.0).wasm_result()?;
    Ok(())
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns a copy of the inner DID document.
  ///
  /// NOTE: If the `ResolvedDocument` is no longer needed after calling this method
  /// then consider using `intoDocument()` for efficiency.
  #[wasm_bindgen]
  pub fn document(&self) -> WasmDocument {
    WasmDocument::from(self.0.document.clone())
  }

  /// Consumes this object and returns the inner DID document.
  ///
  /// NOTE: trying to use the `ResolvedDocument` after calling this will throw an error.
  #[wasm_bindgen(js_name = intoDocument)]
  pub fn into_document(self) -> WasmDocument {
    WasmDocument::from(self.0.document)
  }

  /// Returns a copy of the diff chain message id.
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen(js_name = diffMessageId)]
  pub fn diff_message_id(&self) -> String {
    self.0.diff_message_id.to_string()
  }

  /// Sets the diff chain message id.
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen(js_name = setDiffMessageId)]
  pub fn set_diff_message_id(&mut self, value: &str) -> Result<()> {
    let message_id: MessageId = MessageId::from_str(value)
      .map_err(identity_iota::iota_core::Error::InvalidMessage)
      .wasm_result()?;
    self.0.diff_message_id = message_id;
    Ok(())
  }

  /// Returns a copy of the integration chain message id.
  #[wasm_bindgen(js_name = integrationMessageId)]
  pub fn integration_message_id(&self) -> String {
    self.0.integration_message_id.to_string()
  }

  /// Sets the integration chain message id.
  #[wasm_bindgen(js_name = setIntegrationMessageId)]
  pub fn set_integration_message_id(&mut self, value: &str) -> Result<()> {
    let message_id: MessageId = MessageId::from_str(value)
      .map_err(identity_iota::iota_core::Error::InvalidMessage)
      .wasm_result()?;
    self.0.integration_message_id = message_id;
    Ok(())
  }
}

impl_wasm_json!(WasmResolvedDocument, ResolvedDocument);
impl_wasm_clone!(WasmResolvedDocument, ResolvedDocument);

impl From<ResolvedIotaDocument> for WasmResolvedDocument {
  fn from(document: ResolvedIotaDocument) -> Self {
    Self(document)
  }
}

impl From<WasmResolvedDocument> for ResolvedIotaDocument {
  fn from(wasm_document: WasmResolvedDocument) -> Self {
    wasm_document.0
  }
}
