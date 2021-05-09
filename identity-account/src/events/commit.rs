// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::events::Event;
use crate::identity::IdentityId;
use crate::types::Generation;

/// An [event][Event] and position in an identity event sequence.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Commit {
  identity: IdentityId,
  sequence: Generation,
  event: Event,
}

impl Commit {
  /// Creates a new `Commit`.
  pub const fn new(identity: IdentityId, sequence: Generation, event: Event) -> Self {
    Self {
      identity,
      sequence,
      event,
    }
  }

  /// Returns the identifier of the associated identity.
  pub const fn identity(&self) -> IdentityId {
    self.identity
  }

  /// Returns the sequence index of the event.
  pub const fn sequence(&self) -> Generation {
    self.sequence
  }

  /// Returns a reference to the underlying event.
  pub const fn event(&self) -> &Event {
    &self.event
  }

  /// Consumes the commit and returns the underlying event.
  pub fn into_event(self) -> Event {
    self.event
  }
}
