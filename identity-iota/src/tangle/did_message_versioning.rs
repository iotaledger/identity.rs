// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Error;

#[derive(Copy, Clone)]
pub(crate) enum DIDMessageVersion {
  V1 = 1,
}

pub(crate) const CURRENT_MESSAGE_VERSION: DIDMessageVersion = DIDMessageVersion::V1;

/// Adds the current message version flag at the beginning of arbitrary data.
pub(crate) fn add_version_flag(mut data: Vec<u8>, message_version: DIDMessageVersion) -> Vec<u8> {
  let version_flag = message_version as u8;
  data.splice(0..0, [version_flag].iter().cloned());
  data
}

/// Checks if flag matches message_version.
pub(crate) fn check_version_flag(flag: &u8, message_version: DIDMessageVersion) -> Result<(), Error> {
  if message_version as u8 == *flag {
    Ok(())
  } else {
    Err(Error::InvalidMessageFlags)
  }
}

#[cfg(test)]
mod test {
  use crate::tangle::did_message_versioning::add_version_flag;
  use crate::tangle::did_message_versioning::CURRENT_MESSAGE_VERSION;

  #[test]
  fn test_add_version_flag() {
    let message: Vec<u8> = vec![10, 4, 5, 5];
    let message_with_flag = add_version_flag(message, CURRENT_MESSAGE_VERSION);
    assert_eq!(message_with_flag, [CURRENT_MESSAGE_VERSION as u8, 10, 4, 5, 5])
  }
}
