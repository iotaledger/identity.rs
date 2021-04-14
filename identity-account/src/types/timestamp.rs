// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp as CoreTimestamp;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
pub struct Timestamp(i64);

impl Timestamp {
  pub const EPOCH: Self = Self(0);

  pub fn now() -> Self {
    CoreTimestamp::now().into()
  }

  pub fn is_epoch(&self) -> bool {
    static EPOCH: &'static Timestamp = &Timestamp::EPOCH;
    self == EPOCH
  }
}

impl From<CoreTimestamp> for Timestamp {
  fn from(other: CoreTimestamp) -> Self {
    Self(other.to_unix())
  }
}

impl From<Timestamp> for CoreTimestamp {
  fn from(other: Timestamp) -> Self {
    CoreTimestamp::from_unix(other.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_roundtrip() {
    let time: Timestamp = Timestamp::now();
    let core: CoreTimestamp = time.into();

    assert_eq!(time, Timestamp::from(core));
  }
}
