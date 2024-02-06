// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use num_traits::FromPrimitive;

use crate::Error;

/// Indicates the version of a DID document in state metadata.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, num_derive::FromPrimitive)]
#[non_exhaustive]
pub(crate) enum StateMetadataVersion {
  V1 = 1,
}

impl StateMetadataVersion {
  pub(crate) const CURRENT: Self = Self::V1;
}

impl TryFrom<u8> for StateMetadataVersion {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    FromPrimitive::from_u8(value).ok_or(Error::InvalidStateMetadata("unsupported version number"))
  }
}
