// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::IotaDID;

use crate::identity::IdentityState;
use crate::storage::Storage;

/// A read-only view of an identity state with a read-write storage instance.
#[derive(Debug)]
pub struct Context<'a> {
  did: &'a IotaDID,
  state: &'a IdentityState,
  store: &'a dyn Storage,
}

impl<'a> Context<'a> {
  /// Creates a new `Context`.
  pub fn new(did: &'a IotaDID, state: &'a IdentityState, store: &'a dyn Storage) -> Self {
    Self { did, state, store }
  }

  pub fn did(&self) -> &IotaDID {
    self.did
  }

  /// Returns the context `state`.
  pub fn state(&self) -> &IdentityState {
    self.state
  }

  /// Returns the context `store`.
  pub fn store(&self) -> &dyn Storage {
    self.store
  }
}
