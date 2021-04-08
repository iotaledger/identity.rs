// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;
use futures::executor;
use identity_core::crypto::Sign;
use identity_core::error::Error;
use identity_core::error::Result;

use crate::chain::ChainKey;
use crate::storage::Storage;
use crate::types::ChainId;

#[derive(Debug)]
pub struct RemoteKey<'a, T> {
  chain: ChainId,
  key: &'a ChainKey,
  store: &'a T,
}

impl<'a, T> RemoteKey<'a, T> {
  pub fn new(chain: ChainId, key: &'a ChainKey, store: &'a T) -> Self {
    Self { chain, key, store }
  }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub struct RemoteSign<'a, T> {
  marker: PhantomData<RemoteKey<'a, T>>,
}

impl<'a, T> Sign for RemoteSign<'a, T>
where
  T: Storage,
{
  type Secret = RemoteKey<'a, T>;
  type Output = Vec<u8>;

  fn sign(message: &[u8], key: &Self::Secret) -> Result<Self::Output> {
    let future: _ = key.store.key_sign(key.chain, key.key, message.to_vec());

    executor::block_on(future)
      .map_err(|_| Error::InvalidProofValue("remote sign"))
      .map(|signature| signature.data)
  }
}
