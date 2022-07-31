// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_stardust::StardustDocumentMetadata;
use wasm_bindgen::prelude::*;

use crate::common::MapStringAny;
use crate::common::WasmTimestamp;
use crate::error::Result;

/// Additional attributes related to an IOTA DID Document.
#[wasm_bindgen(js_name = StardustDocumentMetadata, inspectable)]
pub struct WasmStardustDocumentMetadata(pub(crate) StardustDocumentMetadata);

// NOTE: these properties are read-only (no setters) to prevent bugs where a clone of the metadata
//       is updated instead of the actual instance in the document.
#[wasm_bindgen(js_class = StardustDocumentMetadata)]
impl WasmStardustDocumentMetadata {
  /// Returns a copy of the timestamp of when the DID document was created.
  #[wasm_bindgen]
  pub fn created(&self) -> Option<WasmTimestamp> {
    self.0.created.map(WasmTimestamp::from)
  }

  /// Returns a copy of the timestamp of the last DID document update.
  #[wasm_bindgen]
  pub fn updated(&self) -> Option<WasmTimestamp> {
    self.0.updated.map(WasmTimestamp::from)
  }

  /// Returns a copy of the custom metadata properties.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(&self.0.properties)
  }
}

impl_wasm_json!(WasmStardustDocumentMetadata, StardustDocumentMetadata);
impl_wasm_clone!(WasmStardustDocumentMetadata, StardustDocumentMetadata);

impl From<StardustDocumentMetadata> for WasmStardustDocumentMetadata {
  fn from(metadata: StardustDocumentMetadata) -> Self {
    Self(metadata)
  }
}
