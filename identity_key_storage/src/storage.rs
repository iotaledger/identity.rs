// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::identity_storage::IdentityStorage;
use crate::key_storage::KeyStorage;
use std::sync::Arc;

// TODO: Write decent documentation explaining what this is for at a high level.
struct Storage<K, I>
where
  K: KeyStorage,
  I: IdentityStorage,
{
  key_storage: Arc<K>,
  identity_storage: Arc<I>,
}

impl<K, I> Storage<K, I>
where
  K: KeyStorage,
  I: IdentityStorage,
{
  /// Constructs a new [`Storage`].
  pub fn new(key_storage: impl Into<Arc<K>>, identity_storage: impl Into<Arc<I>>) -> Self {
    Self {
      key_storage: key_storage.into(),
      identity_storage: identity_storage.into(),
    }
  }

  pub(crate) fn key_storage(&self) -> &K {
    &self.key_storage
  }

  pub(crate) fn identity_storage(&self) -> &I {
    &self.identity_storage
  }
}
