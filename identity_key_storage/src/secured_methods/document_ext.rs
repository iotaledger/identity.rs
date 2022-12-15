// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_core::common::KeyComparable;
use identity_did::did::DID;
use identity_did::document::CoreDocument;

use crate::identity_storage::IdentityStorage;
use crate::key_generation::MultikeySchema;
use crate::key_storage::KeyStorage;
use crate::storage::Storage;

use super::method_creation_error::MethodCreationError;

#[async_trait(?Send)]
pub trait CoreDocumentExt: private::Sealed {
  async fn create_multikey_method<K, I>(
    &mut self,
    fragment: &str,
    schema: &MultikeySchema,
    storage: &Storage<K, I>,
  ) -> Result<(), MethodCreationError>
  where
    K: KeyStorage,
    I: IdentityStorage;

  /*
  async fn purge_method<K,I>(&mut self, fragment: &str, storage: &Storage<K,I>) -> Result<()>
  where
      K: KeyStorage,
      I: IdentityStorage;
  */
}

mod private {
  use identity_core::common::KeyComparable;
  use identity_did::did::DID;
  use identity_did::document::CoreDocument;

  pub trait Sealed {}

  impl<D, T, U, V> Sealed for CoreDocument<D, T, U, V> where D: DID + KeyComparable {}
}

#[async_trait(?Send)]
impl<D: DID + KeyComparable, T, U, V> CoreDocumentExt for CoreDocument<D, T, U, V> {
  async fn create_multikey_method<K, I>(
    &mut self,
    fragment: &str,
    schema: &MultikeySchema,
    storage: &Storage<K, I>,
  ) -> Result<(), MethodCreationError>
  where
    K: KeyStorage,
    I: IdentityStorage,
  {
    // check if the fragment already exists:
    todo!()
  }

  /*
  async fn purge_method(&mut self, fragment: &str, storage: &Storage<K,I>) -> Result<()>
  {
      todo!()
  }
  */
}
