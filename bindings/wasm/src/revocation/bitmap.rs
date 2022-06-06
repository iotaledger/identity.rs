// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::did::RevocationBitmap;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// A compressed bitmap for managing credential revocation.
#[wasm_bindgen(js_name = RevocationBitmap, inspectable)]
#[derive(Clone, Debug, Default)]
pub struct WasmRevocationBitmap(pub(crate) RevocationBitmap);

#[wasm_bindgen(js_class = RevocationBitmap)]
impl WasmRevocationBitmap {
  /// Creates a new `RevocationBitmap` instance.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    WasmRevocationBitmap(RevocationBitmap::new())
  }

  /// Returns `true` if the credential at the given `index` is revoked.
  #[wasm_bindgen(js_name = isRevoked)]
  pub fn is_revoked(&self, index: u32) -> bool {
    self.0.is_revoked(index)
  }

  /// Revokes the credential at the given `index`.
  ///
  /// Return whether the value was absent from the set.
  #[wasm_bindgen]
  pub fn revoke(&mut self, index: u32) -> bool {
    self.0.revoke(index)
  }

  /// The credential at the given `index` will be set to valid.
  ///
  /// Returns ture is the value was present in the set.
  #[wasm_bindgen(js_name = undoRevocation)]
  pub fn undo_revocation(&mut self, index: u32) -> bool {
    self.0.undo_revocation(index)
  }

  /// Deserializes a compressed [`RevocationBitmap`] base64-encoded `data`.
  #[wasm_bindgen(js_name = deserializeCompressedB64)]
  pub fn deserialize_compressed_b64(data: &str) -> Result<WasmRevocationBitmap> {
    let embedded_revocation_list: RevocationBitmap =
      RevocationBitmap::deserialize_compressed_b64(data).wasm_result()?;
    Ok(WasmRevocationBitmap(embedded_revocation_list))
  }

  /// Serializes and compressess [`RevocationBitmap`] as a base64-encoded `String`.
  #[wasm_bindgen(js_name = serializeCompressedB64)]
  pub fn serialize_compressed_b64(&self) -> Result<String> {
    self.0.serialize_compressed_b64().wasm_result()
  }

  /// Serializes a `RevocationBitmap` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `RevocationBitmap` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmRevocationBitmap> {
    value.into_serde().map(Self).wasm_result()
  }
}

impl From<RevocationBitmap> for WasmRevocationBitmap {
  fn from(revocation_list: RevocationBitmap) -> Self {
    WasmRevocationBitmap(revocation_list)
  }
}

impl From<WasmRevocationBitmap> for RevocationBitmap {
  fn from(revocation_list: WasmRevocationBitmap) -> Self {
    revocation_list.0
  }
}
