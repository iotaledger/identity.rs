// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Add;
use core::ops::AddAssign;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Index(u32);

impl Index {
  pub const ZERO: Self = Self(0);
  pub const ONE: Self = Self(1);

  const STEP: u32 = 1;

  pub const fn new() -> Self {
    Self::ZERO
  }

  pub const fn increment(self) -> Self {
    Self(self.0 + Self::STEP)
  }

  pub const fn from_u32(value: u32) -> Self {
    Self(value)
  }

  pub const fn to_u32(self) -> u32 {
    self.0
  }

  pub fn to_bytes(&self) -> [u8; 4] {
    self.0.to_be_bytes()
  }
}

impl From<u32> for Index {
  fn from(other: u32) -> Self {
    Self::from_u32(other)
  }
}

impl From<Index> for u32 {
  fn from(other: Index) -> Self {
    other.to_u32()
  }
}

impl Add for Index {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self(self.0 + other.0)
  }
}

impl AddAssign for Index {
  fn add_assign(&mut self, other: Self) {
    self.0 += other.0;
  }
}
