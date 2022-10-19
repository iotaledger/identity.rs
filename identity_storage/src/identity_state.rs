// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::KeyAlias;
use crate::StorageResult;

/// Holds the internal state for the identity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IdentityState {
  state: IdentityStateV0,
}

/// The map from method fragments to key aliases.
pub(crate) type MethodIndex = HashMap<String, KeyAlias>;

// #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
// pub enum IdentityStateVersion {
//   Version1(IdentityStateV1),
// }

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IdentityStateV0 {
  method_index: MethodIndex,
}

impl IdentityState {
  /// Creates a new [`IdentityState`].
  pub(crate) fn new(method_index: MethodIndex) -> StorageResult<Self> {
    Ok(IdentityState {
      state: IdentityStateV0 { method_index },
    })
  }

  /// Returns a reference to the [`MethodIndex`].
  pub(crate) fn method_index(&self) -> &MethodIndex {
    &self.state.method_index
  }

  /// Returns a mutable reference to the [`MethodIndex`].
  pub(crate) fn method_index_mut(&mut self) -> &mut MethodIndex {
    &mut self.state.method_index
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[repr(u8)]
pub(crate) enum StateVersion {
  #[default]
  V0 = 0u8,
}
