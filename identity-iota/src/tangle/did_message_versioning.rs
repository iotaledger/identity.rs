// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Error;

#[derive(Copy, Clone)]
pub(crate) enum DIDMessageVersion {
  V1 = 1,
}

pub(crate) const CURRENT_MESSAGE_VERSION: DIDMessageVersion = DIDMessageVersion::V1;

/// Checks if flag matches message_version.
pub(crate) fn check_version_flag(flag: &u8, message_version: DIDMessageVersion) -> Result<(), Error> {
  if message_version as u8 == *flag {
    Ok(())
  } else {
    Err(Error::InvalidMessageFlags)
  }
}
