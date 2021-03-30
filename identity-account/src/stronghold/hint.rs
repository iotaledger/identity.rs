// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_stronghold::Error;
use iota_stronghold::RecordHint;

use crate::error::Result;

pub fn hint<T>(data: &T) -> Result<RecordHint>
where
  T: AsRef<[u8]> + ?Sized,
{
  RecordHint::new(data.as_ref()).map_err(Error::from).map_err(Into::into)
}

pub fn default_hint() -> RecordHint {
  // unwrap is okay, the hint is <= 24 bytes
  RecordHint::new([0; 24]).unwrap()
}
