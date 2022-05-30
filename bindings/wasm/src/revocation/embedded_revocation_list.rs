// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota_core::EmbeddedRevocationList;
use wasm_bindgen::prelude::*;

use crate::did::WasmEmbeddedRevocationEndpoint;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = EmbeddedRevocationList, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmEmbeddedRevocationList(pub(crate) EmbeddedRevocationList);

#[wasm_bindgen(js_class = EmbeddedRevocationList)]
impl WasmEmbeddedRevocationList {
  /// Returns the name of the revocation method.
  #[wasm_bindgen]
  pub fn name() -> String {
    EmbeddedRevocationList::name().to_owned()
  }

  // Returns the name of the property that contains the index of the credential to be checked.
  #[wasm_bindgen]
  pub fn credential_list_index_property() -> String {
    EmbeddedRevocationList::credential_list_index_property().to_owned()
  }

  /// Creates a new `EmbeddedRevocationList` revocation method.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    WasmEmbeddedRevocationList(EmbeddedRevocationList::new())
  }

  /// Returns `true` if the credential at the given `index` is revoked.
  #[wasm_bindgen(js_name = isRevoked)]
  pub fn is_revoked(&self, index: u32) -> bool {
    self.0.is_revoked(index)
  }

  /// Revokes the credential at the given `index`.
  #[wasm_bindgen]
  pub fn revoke(&mut self, index: u32) -> bool {
    self.0.revoke(index)
  }

  /// Given the index of multiple credentials, revoke all.
  #[wasm_bindgen(js_name = revokeMultiple)]
  pub fn revoke_multiple(&mut self, indexes: Vec<u32>) {
    self.0.revoke_multiple(&indexes)
  }

  /// The credential at the given `index` will be set to valid.
  #[wasm_bindgen(js_name = undoRevocation)]
  pub fn undo_revocation(&mut self, index: u32) -> bool {
    self.0.undo_revocation(index)
  }

  /// Deserializes a compressed [`EmbeddedRevocationList`] base64-encoded `data`.
  #[wasm_bindgen(js_name = deserializeCompressedB64)]
  pub fn deserialize_compressed_b64(data: &str) -> Result<WasmEmbeddedRevocationList> {
    let embedded_revocation_list: EmbeddedRevocationList =
      EmbeddedRevocationList::deserialize_compressed_b64(data).wasm_result()?;
    Ok(WasmEmbeddedRevocationList(embedded_revocation_list))
  }

  /// Serializes and compressess [`EmbeddedRevocationList`] as a base64-encoded `String`.
  #[wasm_bindgen(js_name = serializeCompressedB64)]
  pub fn serialize_compressed_b64(&self) -> Result<String> {
    self.0.serialize_compressed_b64().wasm_result()
  }

  /// Serializes and compressess the [`EmbeddedRevocationList`] and returns its data url representation.
  #[wasm_bindgen(js_name = toEmbeddedServiceEndpoint)]
  pub fn to_embedded_service_endpoint(&self) -> Result<WasmEmbeddedRevocationEndpoint> {
    self.0.to_embedded_service_endpoint().wasm_result().map(Into::into)
  }

  /// Serializes a `EmbeddedRevocationList` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `EmbeddedRevocationList` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmEmbeddedRevocationList> {
    value.into_serde().map(Self).wasm_result()
  }
}

impl From<EmbeddedRevocationList> for WasmEmbeddedRevocationList {
  fn from(revocation_list: EmbeddedRevocationList) -> Self {
    WasmEmbeddedRevocationList(revocation_list)
  }
}

impl From<WasmEmbeddedRevocationList> for EmbeddedRevocationList {
  fn from(revocation_list: WasmEmbeddedRevocationList) -> Self {
    revocation_list.0
  }
}
