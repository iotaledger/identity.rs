// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;
use futures::executor;
use identity_core::crypto::KeyType;
use identity_core::crypto::Sign;
use identity_core::error::Error;
use identity_core::error::Result;

use crate::storage::StorageHandle;
use crate::types::KeyLocation;

pub struct RemoteKey<'a> {
  storage: &'a StorageHandle,
  location: KeyLocation<'a>,
}

impl<'a> RemoteKey<'a> {
  pub fn new(storage: &'a StorageHandle, location: KeyLocation<'a>) -> Self {
    Self { storage, location }
  }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub struct RemoteEd25519<'a>(PhantomData<RemoteKey<'a>>);

impl<'a> Sign for RemoteEd25519<'a> {
  type Secret = RemoteKey<'a>;
  type Output = Vec<u8>;

  fn sign(message: &[u8], key: &Self::Secret) -> Result<Self::Output> {
    let future: _ = key
      .storage
      .key_sign(KeyType::Ed25519, &key.location, message.to_vec());

    executor::block_on(future)
      .map_err(|_| Error::InvalidProofValue)
      .map(|signature| signature.data)
  }
}
