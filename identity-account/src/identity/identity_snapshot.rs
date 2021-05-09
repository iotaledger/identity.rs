// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::identity::IdentityId;
use crate::identity::IdentityState;
use crate::types::Generation;

/// A snapshot of an identity state at a particular index.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentitySnapshot {
  pub(crate) sequence: Generation,
  pub(crate) identity: IdentityState,
}

impl IdentitySnapshot {
  /// Creates a new `IdentitySnapshot` instance.
  pub fn new(identity: IdentityState) -> Self {
    Self {
      sequence: Generation::new(),
      identity,
    }
  }

  /// Returns the identifier for this identity.
  pub fn id(&self) -> IdentityId {
    self.identity.id()
  }

  /// Returns the sequence index of the snapshot.
  pub fn sequence(&self) -> Generation {
    self.sequence
  }

  /// Returns the identity state of the snapshot.
  pub fn identity(&self) -> &IdentityState {
    &self.identity
  }
}
