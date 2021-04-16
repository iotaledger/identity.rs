// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::chain::ChainData;

#[derive(Debug)]
pub struct Context<'a, T> {
  state: &'a ChainData,
  store: &'a T,
}

impl<'a, T> Context<'a, T> {
  pub fn new(state: &'a ChainData, store: &'a T) -> Self {
    Self { state, store }
  }

  pub fn state(&self) -> &ChainData {
    self.state
  }

  pub fn store(&self) -> &T {
    self.store
  }
}
