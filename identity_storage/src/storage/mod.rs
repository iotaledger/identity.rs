// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/*
 * Modifications Copyright 2024 Fondazione LINKS.
 */

//! This module provides a type wrapping a key and key id storage.

mod error;
#[macro_use]
mod jwk_document_ext;
#[cfg(feature = "jpt-bbs-plus")]
mod jwp_document_ext;
mod signature_options;
#[cfg(feature = "jpt-bbs-plus")]
mod timeframe_revocation_ext;
#[cfg(feature = "hybrid")]
mod hybrid_jws_document_ext;
#[cfg(feature = "pqc")]
mod pqc_jws_document_ext;

mod did_jwk_document_ext;

#[cfg(all(test, feature = "memstore"))]
pub(crate) mod tests;

pub use error::*;

pub use jwk_document_ext::*;
#[cfg(feature = "jpt-bbs-plus")]
pub use jwp_document_ext::*;
pub use signature_options::*;
#[cfg(feature = "jpt-bbs-plus")]
pub use timeframe_revocation_ext::*;
#[cfg(feature = "hybrid")]
pub use hybrid_jws_document_ext::*;
#[cfg(feature = "pqc")]
pub use pqc_jws_document_ext::*;

pub use did_jwk_document_ext::*;

/// A type wrapping a key and key id storage, typically used with [`JwkStorage`](crate::key_storage::JwkStorage) and
/// [`KeyIdStorage`](crate::key_id_storage::KeyIdStorage) that should always be used together when calling methods from
/// [`JwkDocumentExt`](crate::storage::JwkDocumentExt).
pub struct Storage<K, I> {
  key_storage: K,
  key_id_storage: I,
}

impl<K, I> Storage<K, I> {
  /// Constructs a new [`Storage`].
  pub fn new(key_storage: K, key_id_storage: I) -> Self {
    Self {
      key_storage,
      key_id_storage,
    }
  }

  /// Obtain a reference to the wrapped [`JwkStorage`](crate::key_storage::JwkStorage).
  pub fn key_storage(&self) -> &K {
    &self.key_storage
  }

  /// Obtain a reference to the wrapped [`KeyIdStorage`](crate::key_id_storage::KeyIdStorage).
  pub fn key_id_storage(&self) -> &I {
    &self.key_id_storage
  }
}
