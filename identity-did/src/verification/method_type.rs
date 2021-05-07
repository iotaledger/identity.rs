// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use crate::error::Error;
use crate::error::Result;

/// Supported verification method types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(u32)]
pub enum MethodType {
  Ed25519VerificationKey2018 = 0,
  MerkleKeyCollection2021 = 1,
}

impl MethodType {
  pub fn from_u32(value: u32) -> Option<Self> {
    match value {
      0 => Some(Self::Ed25519VerificationKey2018),
      1 => Some(Self::MerkleKeyCollection2021),
      _ => None,
    }
  }

  pub const fn as_u32(self) -> u32 {
    self as u32
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Ed25519VerificationKey2018 => "Ed25519VerificationKey2018",
      Self::MerkleKeyCollection2021 => "MerkleKeyCollection2021",
    }
  }
}

impl FromStr for MethodType {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    match string {
      "Ed25519VerificationKey2018" => Ok(Self::Ed25519VerificationKey2018),
      "MerkleKeyCollection2021" => Ok(Self::MerkleKeyCollection2021),
      _ => Err(Error::UnknownMethodType),
    }
  }
}
