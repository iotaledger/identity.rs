// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::document::CoreDocument;
use identity_iota::prelude::IotaDocument;

use crate::did::CoreDocumentLock;
use crate::did::IAsCoreDocument;
use crate::iota::IotaDocumentLock;

/// A shallow copy of a document imported from JS.
pub(crate) enum ImportedDocumentLock {
  Core(Rc<CoreDocumentLock>),
  Iota(Rc<IotaDocumentLock>),
}

impl ImportedDocumentLock {
  /// Obtain a read guard which implements `AsRef<CoreDocument>`.
  pub(crate) fn blocking_read(&self) -> ImportedDocumentReadGuard<'_> {
    match self {
      Self::Iota(lock) => ImportedDocumentReadGuard::Iota(lock.blocking_read()),
      Self::Core(lock) => ImportedDocumentReadGuard::Core(lock.blocking_read()),
    }
  }
}

impl From<&IAsCoreDocument> for ImportedDocumentLock {
  fn from(value: &IAsCoreDocument) -> Self {
    // Use specially crafted functions that 1) Provide strongly typed values without expensive cloning and 2) use our
    // custom JS shims to make sure that pointers are not nulled after passing them to Rust.
    if let Some(doc) = crate::iota::maybe_get_iota_document(value.as_ref()) {
      Self::Iota(doc.0)
    } else {
      Self::Core(crate::did::getCoreDocument(value.as_ref()).0)
    }
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
