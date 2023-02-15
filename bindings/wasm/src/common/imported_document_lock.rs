// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::document::CoreDocument;
use identity_iota::prelude::IotaDocument;
use js_sys::Array;

use crate::did::ArrayIAsCoreDocument;
use crate::did::CoreDocumentLock;
use crate::did::IAsCoreDocument;
use crate::did::WasmCoreDocument;
use crate::iota::IotaDocumentLock;
use crate::iota::WasmIotaDocument;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// A shallow copy of a document imported from JS.
/// Instances of this type are expected to be short lived (for the duration of a function call)
/// in order to avoid unintentional memory leaks.
pub(crate) enum ImportedDocumentLock {
  Core(Rc<CoreDocumentLock>),
  Iota(Rc<IotaDocumentLock>),
}

impl ImportedDocumentLock {
  /// Obtain a read guard which implements `AsRef<CoreDocument>`.
  pub(crate) fn blocking_read(&self) -> ImportedDocumentReadGuard<'_> {
    match self {
      Self::Iota(lock) => lock.blocking_read().into(),
      Self::Core(lock) => lock.blocking_read().into(),
    }
  }
  /// Must only be called on values implementing `IAsCoreDocument`.
  pub(crate) fn from_js_value_unchecked(value: &JsValue) -> Self {
    if let Some(doc) = maybe_get_iota_document(value) {
      Self::Iota(doc.0)
    } else {
      Self::Core(getCoreDocument(value).0)
    }
  }
}

impl From<&IAsCoreDocument> for ImportedDocumentLock {
  fn from(value: &IAsCoreDocument) -> Self {
    Self::from_js_value_unchecked(value.as_ref())
  }
}

impl From<&ArrayIAsCoreDocument> for Vec<ImportedDocumentLock> {
  fn from(value: &ArrayIAsCoreDocument) -> Self {
    let value_array = value
      .dyn_ref::<Array>()
      .expect("the provided argument should be of type `Array`");
    value_array
      .iter()
      .map(|value| ImportedDocumentLock::from_js_value_unchecked(&value))
      .collect()
  }
}

pub(crate) enum ImportedDocumentReadGuard<'a> {
  Core(tokio::sync::RwLockReadGuard<'a, CoreDocument>),
  Iota(tokio::sync::RwLockReadGuard<'a, IotaDocument>),
}

impl<'a> AsRef<CoreDocument> for ImportedDocumentReadGuard<'a> {
  fn as_ref(&self) -> &CoreDocument {
    match self {
      Self::Core(doc) => doc.as_ref(),
      Self::Iota(doc) => doc.as_ref(),
    }
  }
}

impl<'a> From<tokio::sync::RwLockReadGuard<'a, CoreDocument>> for ImportedDocumentReadGuard<'a> {
  fn from(value: tokio::sync::RwLockReadGuard<'a, CoreDocument>) -> Self {
    Self::Core(value)
  }
}

impl<'a> From<tokio::sync::RwLockReadGuard<'a, IotaDocument>> for ImportedDocumentReadGuard<'a> {
  fn from(value: tokio::sync::RwLockReadGuard<'a, IotaDocument>) -> Self {
    Self::Iota(value)
  }
}

// Specially crafted functions that 1) Provide strongly typed values without expensive cloning and 2) use our
// custom JS shims to make sure that pointers are not nulled after passing them to Rust.
#[wasm_bindgen]
extern "C" {
  /// Called internally by `ImportedDocumentLock`, if used elsewhere panics or memory leaks may occur.  
  #[wasm_bindgen(js_name = _getCoreDocumentInternal)]
  pub fn getCoreDocument(input: &JsValue) -> WasmCoreDocument;

  /// Called internally by `ImportedDocumentLock`, if used elsewhere panics or memory leaks may occur.
  #[wasm_bindgen(js_name = _maybeGetIotaDocumentInternal)]
  pub fn maybe_get_iota_document(input: &JsValue) -> Option<WasmIotaDocument>;
}