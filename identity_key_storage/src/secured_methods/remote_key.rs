// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;

use crate::key_storage::KeyStorageResult;
use crate::signature::Signature;

/// A type providing an API to a private key secured by a [`KeyStorage`](crate::key_storage::KeyStorage).
///
/// # Security
/// This type does not load the secured private key into memory, but may hold the corresponding
/// [`KeyId`](crate::identifiers::KeyId) in memory until dropped.
pub struct RemoteKey<F> {
  signer: F,
}

impl<F> RemoteKey<F> {
  /// Sign the given `data` according to the provided `signing_algorithm`. 
  pub async fn sign<T>(&self, signing_algorithm: &str, data: Vec<u8>) -> KeyStorageResult<Signature>
  where
    F: Fn(&str, Vec<u8>) -> T,
    T: Future<Output = KeyStorageResult<Signature>>,
  {
    (self.signer)(signing_algorithm, data).await
  }

  #[cfg(test)]
  /// A mock constructor that [`Cryptosuite`](crate::secured_methods::CryptoSuite) implementors may use in tests. 
  ///
  /// # Warning 
  ///  This method is not covered by SemVer. 
  pub fn mock(signer: F) -> Self {
    Self {
        signer
    }
  }
}
