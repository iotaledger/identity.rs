// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_storage::StorageError;
use identity_storage::StorageResult;
use js_sys::Array;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub(crate) fn uint8array_to_bytes(value: JsValue) -> StorageResult<Vec<u8>> {
  if !JsCast::is_instance_of::<Uint8Array>(&value) {
    return Err(StorageError::new(identity_storage::StorageErrorKind::Other(
      "expected Uint8Array".into(),
    )));
  }
  let array_js_value = JsValue::from(Array::from(&value));
  array_js_value.into_serde().map_err(|e| {
    StorageError::new(identity_storage::StorageErrorKind::Other(
      format!("serialization error: {e}").into(),
    ))
  })
}
