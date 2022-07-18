// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use num_traits::FromPrimitive;

use crate::Error;

/// Indicates the encoding of a DID document in state metadata.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, num_derive::FromPrimitive)]
pub enum StateMetadataEncoding {
  Json = 0,
}

impl TryFrom<u8> for StateMetadataEncoding {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    FromPrimitive::from_u8(value).ok_or(Error::InvalidMessageFlags)
  }
}
