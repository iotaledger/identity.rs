// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::identifiers::KeyId;
use crate::key_storage::KeyStorage;
use crate::key_storage::KeyStorageError;
use crate::key_storage::KeyStorageResult;
use crate::signature::Signature;

/// A type providing an API to a private key secured by a [`KeyStorage`](crate::key_storage::KeyStorage).
///
/// # Security
/// This type does not load the secured private key into memory, but may hold the corresponding
/// [`KeyId`](crate::identifiers::KeyId) in memory until dropped.
pub struct RemoteKey<K> {
  //TODO: Consider abstracting over the pointer type using GAT.
  key_storage: Arc<K>,
  key_id: KeyId,
}

impl<K: KeyStorage> RemoteKey<K> {
  pub(crate) fn new(key_storage: Arc<K>, key_id: KeyId) -> Self {
    Self { key_storage, key_id }
  }

  /// Sign the given `data` according to the provided `signing_algorithm`.
  pub async fn sign(&self, signing_algorithm: &str, data: Vec<u8>) -> KeyStorageResult<Signature>
  where
    K: KeyStorage,
  {
    let Ok(algorithm) = K::SigningAlgorithm::try_from(signing_algorithm) else {
      return Err(KeyStorageError::new(crate::key_storage::KeyStorageErrorKind::UnrecognizedSigningAlgorithm))
    };

    self.key_storage.sign(&self.key_id, &algorithm, data).await
  }
}
