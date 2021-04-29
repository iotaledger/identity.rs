// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::identity::IdentityState;

/// A read-only view of an identity state with a read-write storage instance.
#[derive(Debug)]
pub struct Context<'a, T> {
  state: &'a IdentityState,
  store: &'a T,
}

impl<'a, T> Context<'a, T> {
  /// Creates a new `Context`.
  pub fn new(state: &'a IdentityState, store: &'a T) -> Self {
    Self { state, store }
  }

  /// Returns the context `state`.
  pub fn state(&self) -> &IdentityState {
    self.state
  }

  /// Returns the context `store`.
  pub fn store(&self) -> &T {
    self.store
  }
}
