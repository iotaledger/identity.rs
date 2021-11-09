// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use num_traits::FromPrimitive;

use crate::Error;

/// Indicates the version of a DID Message.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, num_derive::FromPrimitive)]
pub(crate) enum DIDMessageVersion {
  V1 = 1,
}

pub(crate) const CURRENT_MESSAGE_VERSION: DIDMessageVersion = DIDMessageVersion::V1;

impl TryFrom<u8> for DIDMessageVersion {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    FromPrimitive::from_u8(value).ok_or(Error::InvalidMessageFlags)
  }
}
