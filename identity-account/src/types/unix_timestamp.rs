// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use identity_core::common::Timestamp;

/// A simple representation of a unix timestamp.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct UnixTimestamp(i64);

impl UnixTimestamp {
  pub const EPOCH: Self = Self(0);

  /// Returns the current time as a unix timestamp.
  pub fn now() -> Self {
    Timestamp::now().into()
  }

  /// Returns true if this time is the unix epoch.
  pub fn is_epoch(&self) -> bool {
    static EPOCH: &UnixTimestamp = &UnixTimestamp::EPOCH;
    self == EPOCH
  }
}

impl Debug for UnixTimestamp {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("UnixTimestamp({})", self.0))
  }
}

impl Display for UnixTimestamp {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Display::fmt(&self.0, f)
  }
}

impl Default for UnixTimestamp {
  fn default() -> Self {
    Self::EPOCH
  }
}

impl From<Timestamp> for UnixTimestamp {
  fn from(other: Timestamp) -> Self {
    Self(other.to_unix())
  }
}

impl From<UnixTimestamp> for Timestamp {
  fn from(other: UnixTimestamp) -> Self {
    Timestamp::from_unix(other.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_roundtrip() {
    let time: UnixTimestamp = UnixTimestamp::now();
    let core: Timestamp = time.into();

    assert_eq!(time, UnixTimestamp::from(core));
  }
}
