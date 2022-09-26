// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use num_traits::FromPrimitive;

use crate::Error;

/// Indicates the encoding and compression of a DID Message.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, num_derive::FromPrimitive)]
pub enum DIDMessageEncoding {
  Json = 0,
  JsonBrotli = 1,
}

impl TryFrom<u8> for DIDMessageEncoding {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    FromPrimitive::from_u8(value).ok_or(Error::InvalidMessageFlags)
  }
}
