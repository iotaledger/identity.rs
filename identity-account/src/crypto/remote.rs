// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;
use futures::executor;
use identity_core::crypto::Sign;
use identity_core::error::Error;
use identity_core::error::Result;
use identity_iota::did::IotaDID;

use crate::storage::Storage;
use crate::types::KeyLocation;

/// A reference to a storage instance and identity key location.
#[derive(Debug)]
pub struct RemoteKey<'a> {
  did: &'a IotaDID,
  location: &'a KeyLocation,
  store: &'a dyn Storage,
}

impl<'a> RemoteKey<'a> {
  /// Creates a new `RemoteKey` instance.
  pub fn new(did: &'a IotaDID, location: &'a KeyLocation, store: &'a dyn Storage) -> Self {
    Self { did, location, store }
  }
}

// =============================================================================
// RemoteSign
// =============================================================================

/// A remote signature that delegates to a storage implementation.
///
/// Note: The signature implementation is specified by the associated `RemoteKey`.
#[derive(Clone, Copy, Debug)]
pub struct RemoteSign<'a> {
  marker: PhantomData<RemoteKey<'a>>,
}

impl<'a> Sign for RemoteSign<'a>
{
  type Private = RemoteKey<'a>;
  type Output = Vec<u8>;

  fn sign(message: &[u8], key: &Self::Private) -> Result<Self::Output> {
    let future: _ = key.store.key_sign(key.did, key.location, message.to_vec());

    executor::block_on(future)
      .map_err(|_| Error::InvalidProofValue("remote sign"))
      .map(|signature| signature.data)
  }
}
