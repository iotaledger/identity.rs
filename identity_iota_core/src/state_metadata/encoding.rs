// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use num_traits::FromPrimitive;

use crate::Error;

/// Indicates the encoding of a DID document in state metadata.
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, num_derive::FromPrimitive)]
#[non_exhaustive]
pub enum StateMetadataEncoding {
  /// State Metadata encoded as JSON.
  #[default]
  Json = 0,
}

impl TryFrom<u8> for StateMetadataEncoding {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    FromPrimitive::from_u8(value).ok_or(Error::InvalidStateMetadata("unsupported encoding"))
  }
}
