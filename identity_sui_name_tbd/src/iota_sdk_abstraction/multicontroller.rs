// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file has been moved here from identity_iota_core/src/client_dummy.
// The file will be removed after the TS-Client-SDK is integrated.
// The file provides a POC for the wasm-bindgen glue code needed to
// implement the TS-Client-SDK integration.

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use super::Hashable;
use super::types::base_types::{ObjectID};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Multicontroller<T> {
  controlled_value: T,
  controllers: HashMap<Hashable<ObjectID>, u64>,
  proposals: HashMap<String, Proposal>,
}

impl<T> Multicontroller<T> {
  /// TODO: remove this, added to test interface in ts
  pub fn new(controlled_value: T) -> Self {
    Self {
      controlled_value,
      controllers: HashMap::new(),
      proposals: HashMap::new(),
    }
  }

  pub fn controlled_value(&self) -> &T {
    &self.controlled_value
  }
  pub fn threshold(&self) -> u64 {
    123
  }
  pub fn controller_voting_power(&self, _controller_cap_id: ObjectID) -> Option<u64> {
    Some(123)
  }
  pub fn proposals(&self) -> &HashMap<String, Proposal> {
    &self.proposals
  }
  pub fn into_inner(self) -> T {
    self.controlled_value
  }
  pub fn has_member(&self, _cap_id: ObjectID) -> bool {
    true
  }
}
