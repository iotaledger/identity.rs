// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Rem;
use core::ops::RemAssign;

use crate::error::Error;
use crate::error::Result;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Index(u32);

impl Index {
  pub const MIN: Self = Self::ZERO;
  pub const MAX: Self = Self(u32::MAX);

  pub const ZERO: Self = Self(0);
  pub const ONE: Self = Self(1);

  const STEP: u32 = 1;

  pub const fn new() -> Self {
    Self::MIN
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

  pub fn try_increment(self) -> Result<Self> {
    self.0.checked_add(1).map(Self).ok_or(Error::IndexOverflow)
  }

  pub fn try_decrement(self) -> Result<Self> {
    self.0.checked_sub(1).map(Self).ok_or(Error::IndexUnderflow)
  }
}

impl Debug for Index {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("Index({})", self.to_u32()))
  }
}

impl Display for Index {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{}", self.to_u32()))
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

impl Rem for Index {
  type Output = Self;

  fn rem(self, other: Self) -> Self::Output {
    Self(self.0 % other.0)
  }
}

impl RemAssign for Index {
  fn rem_assign(&mut self, other: Self) {
    self.0 %= other.0;
  }
}
