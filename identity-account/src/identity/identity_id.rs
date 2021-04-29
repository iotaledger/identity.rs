// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use crate::error::Error;
use crate::error::Result;

/// A 32-bit identifier for stored Identities.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(from = "u32", into = "u32")]
#[repr(transparent)]
pub struct IdentityId([u8; 4]);

impl IdentityId {
  const ZERO: Self = Self::from_u32(0);

  /// Creates a new identity id from a slice of bytes.
  ///
  /// # Errors
  ///
  /// Fails if the given bytes are not a valid id.
  pub fn from_slice(slice: &[u8]) -> Result<Self> {
    slice
      .try_into()
      .map_err(|_| Error::IdentityIdInvalid)
      .map(Self::from_bytes)
  }

  /// Creates a new identity id from an array of bytes.
  pub const fn from_bytes(bytes: [u8; 4]) -> Self {
    Self(bytes)
  }

  /// Creates a new identity id from a 32-bit integer.
  pub const fn from_u32(value: u32) -> Self {
    Self(value.to_be_bytes())
  }

  /// Returns the identity id as a slice of bytes.
  pub const fn as_bytes(&self) -> &[u8] {
    &self.0
  }

  /// Returns the identity id as an array of bytes.
  pub const fn to_bytes(self) -> [u8; 4] {
    self.0
  }

  /// Returns the identity id as a 32-bit integer.
  pub const fn to_u32(self) -> u32 {
    u32::from_be_bytes(self.0)
  }

  /// Returns the next identity id in the sequence.
  ///
  /// # Errors
  ///
  /// Fails if the current id is the maximum supported value.
  pub fn try_next(self) -> Result<Self> {
    self
      .to_u32()
      .checked_add(1)
      .map(Self::from_u32)
      .ok_or(Error::IdentityIdOverflow)
  }
}

impl Debug for IdentityId {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("IdentityId({:#010x})", self.to_u32()))
  }
}

impl Display for IdentityId {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{:#010x}", self.to_u32()))
  }
}

impl Default for IdentityId {
  fn default() -> Self {
    Self::ZERO
  }
}

impl From<u32> for IdentityId {
  fn from(other: u32) -> Self {
    Self::from_u32(other)
  }
}

impl From<IdentityId> for u32 {
  fn from(other: IdentityId) -> Self {
    other.to_u32()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_roundtrip() {
    let id: IdentityId = IdentityId::from_u32(123);

    assert_eq!(id.to_u32(), 123);
    assert_eq!(IdentityId::from_bytes(id.to_bytes()), id);
    assert_eq!(IdentityId::from_slice(id.as_bytes()).unwrap(), id);
  }

  #[test]
  fn test_from_slice() {
    assert!(IdentityId::from_slice(&[]).is_err());
    assert!(IdentityId::from_slice(&[0x0]).is_err());
    assert!(IdentityId::from_slice(&[0x0; 3]).is_err());
    assert!(IdentityId::from_slice(&[0x0; 5]).is_err());
    assert!(IdentityId::from_slice(&[0x0; 4]).is_ok());
  }
}
