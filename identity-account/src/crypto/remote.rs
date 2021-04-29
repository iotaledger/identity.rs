// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;
use futures::executor;
use identity_core::crypto::Sign;
use identity_core::error::Error;
use identity_core::error::Result;

use crate::identity::IdentityId;
use crate::storage::Storage;
use crate::types::KeyLocation;

/// A reference to a storage instance and identity key location.
#[derive(Debug)]
pub struct RemoteKey<'a, T> {
  id: IdentityId,
  location: &'a KeyLocation,
  store: &'a T,
}

impl<'a, T> RemoteKey<'a, T> {
  /// Creates a new `RemoteKey` instance.
  pub fn new(id: IdentityId, location: &'a KeyLocation, store: &'a T) -> Self {
    Self { id, location, store }
  }
}

// =============================================================================
// RemoteSign
// =============================================================================

/// A remote signature that delegates to a storage implementation.
///
/// Note: The signature implementation is specified by the associated `RemoteKey`.
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
    let future: _ = key.store.key_sign(key.id, key.location, message.to_vec());

    executor::block_on(future)
      .map_err(|_| Error::InvalidProofValue("remote sign"))
      .map(|signature| signature.data)
  }
}
