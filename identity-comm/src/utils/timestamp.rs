// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// UTC Epoch Seconds
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Timestamp(i64);

impl Timestamp {
  pub const fn get(self) -> i64 {
    self.0
  }
}
