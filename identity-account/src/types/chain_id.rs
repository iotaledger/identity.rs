// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use crate::error::Error;
use crate::error::Result;
use crate::types::Index;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(from = "u32", into = "u32")]
#[repr(transparent)]
pub struct ChainId([u8; 4]);

impl ChainId {
  pub const DEFAULT: Self = Self::from_u32(0);

  pub fn from_slice(slice: &[u8]) -> Result<Self> {
    slice
      .try_into()
      .map_err(|_| Error::InvalidChainId)
      .map(Self::from_bytes)
  }

  pub const fn from_bytes(bytes: [u8; 4]) -> Self {
    Self(bytes)
  }

  pub const fn from_u32(value: u32) -> Self {
    Self(value.to_be_bytes())
  }

  pub const fn as_bytes(&self) -> &[u8] {
    &self.0
  }

  pub const fn to_bytes(self) -> [u8; 4] {
    self.0
  }

  pub const fn to_u32(self) -> u32 {
    u32::from_be_bytes(self.0)
  }

  pub const fn next(self) -> Self {
    Self::from_u32(self.to_u32() + 1)
  }
}

impl Debug for ChainId {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("ChainId({:#010x})", self.to_u32()))
  }
}

impl Display for ChainId {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{:#010x}", self.to_u32()))
  }
}

impl Default for ChainId {
  fn default() -> Self {
    Self::DEFAULT
  }
}

impl From<u32> for ChainId {
  fn from(other: u32) -> Self {
    Self::from_u32(other)
  }
}

impl From<ChainId> for u32 {
  fn from(other: ChainId) -> Self {
    other.to_u32()
  }
}

impl From<Index> for ChainId {
  fn from(other: Index) -> Self {
    Self::from_u32(other.to_u32())
  }
}

impl From<ChainId> for Index {
  fn from(other: ChainId) -> Self {
    Self::from_u32(other.to_u32())
  }
}
