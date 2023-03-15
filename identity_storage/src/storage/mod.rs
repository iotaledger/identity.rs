// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod error;
mod jwk_storage_document_ext;

pub use error::*;
pub use jwk_storage_document_ext::*;

pub struct Storage<K, I> {
  key_storage: K,
  key_id_storage: I,
}

impl<K, I> Storage<K, I> {
  pub fn new(key_storage: K, key_id_storage: I) -> Self {
    Self {
      key_storage,
      key_id_storage,
    }
  }

  pub fn key_storage(&self) -> &K {
    &self.key_storage
  }

  pub fn key_id_storage(&self) -> &I {
    &self.key_id_storage
  }
}
