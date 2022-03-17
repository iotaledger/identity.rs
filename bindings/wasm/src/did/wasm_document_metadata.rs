// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota_core::IotaDocumentMetadata;
use wasm_bindgen::prelude::*;

use crate::common::WasmTimestamp;
use crate::error::Result;
use crate::error::WasmResult;

// =============================================================================
// =============================================================================

/// Additional attributes related to an IOTA DID Document.
#[wasm_bindgen(js_name = DocumentMetadata, inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct WasmDocumentMetadata(pub(crate) IotaDocumentMetadata);

// NOTE: these properties are read-only (no setters) to prevent bugs where a clone of the metadata
//       is updated instead of the actual instance in the document.
#[wasm_bindgen(js_class = DocumentMetadata)]
impl WasmDocumentMetadata {
  /// Returns a copy of the timestamp of when the DID document was created.
  #[wasm_bindgen]
  pub fn created(&self) -> WasmTimestamp {
    WasmTimestamp::from(self.0.created)
  }

  /// Returns a copy of the timestamp of the last DID document update.
  #[wasm_bindgen]
  pub fn updated(&self) -> WasmTimestamp {
    WasmTimestamp::from(self.0.updated)
  }

  #[wasm_bindgen(getter = previousMessageId)]
  pub fn previous_message_id(&self) -> String {
    self.0.previous_message_id.to_string()
  }
}

impl_wasm_clone!(WasmDocumentMetadata, DocumentMetadata);

impl From<IotaDocumentMetadata> for WasmDocumentMetadata {
  fn from(metadata: IotaDocumentMetadata) -> Self {
    Self(metadata)
  }
}
