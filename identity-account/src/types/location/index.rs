// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Index(pub(crate) u32);

impl Index {
  const DEF: u32 = 0;
  const INC: u32 = 1;

  pub const fn new() -> Self {
    Self(Self::DEF)
  }

  pub fn next(self) -> Option<Self> {
    self.0.checked_add(Self::INC).map(Self)
  }

  pub const fn get(self) -> u32 {
    self.0
  }
}

impl Display for Index {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Display::fmt(&self.0, f)
  }
}

impl From<u32> for Index {
  fn from(other: u32) -> Self {
    Self(other)
  }
}
