// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

#[derive(Clone, Debug)]
pub enum SnapshotStatus {
  /// Snapshot is locked. This means that the password must be set again.
  Locked,
  /// Snapshot is unlocked. The duration is the amount of time left before it locks again.
  Unlocked(Duration),
}

impl SnapshotStatus {
  pub fn locked() -> Self {
    Self::Locked
  }

  pub fn unlocked(duration: Duration) -> Self {
    Self::Unlocked(duration)
  }
}
