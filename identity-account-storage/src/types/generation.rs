// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;
use crate::error::Result;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Generation(u32);

impl Generation {
  pub const MIN: Self = Self(u32::MIN);
  pub const MAX: Self = Self(u32::MAX);

  /// Creates a new `Generation`.
  pub const fn new() -> Self {
    Self::MIN
  }

  /// Creates a new generation from a 32-bit integer.
  pub const fn from_u32(value: u32) -> Self {
    Self(value)
  }

  /// Returns the generation as a 32-bit integer.
  pub const fn to_u32(self) -> u32 {
    self.0
  }

  /// Increments the generation.
  ///
  /// # Errors
  ///
  /// Fails if the generation overflows.
  pub fn try_increment(self) -> Result<Self> {
    self.0.checked_add(1).map(Self).ok_or(Error::GenerationOverflow)
  }

  /// Decrements the generation.
  ///
  /// # Errors
  ///
  /// Fails if the generation underflows.
  pub fn try_decrement(self) -> Result<Self> {
    self.0.checked_sub(1).map(Self).ok_or(Error::GenerationUnderflow)
  }
}

impl Debug for Generation {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("Generation({})", self.to_u32()))
  }
}

impl Display for Generation {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("{}", self.to_u32()))
  }
}

impl From<u32> for Generation {
  fn from(other: u32) -> Self {
    Self::from_u32(other)
  }
}

impl From<Generation> for u32 {
  fn from(other: Generation) -> Self {
    other.to_u32()
  }
}
