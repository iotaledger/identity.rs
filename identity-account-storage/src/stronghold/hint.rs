// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_stronghold::RecordHint;

pub fn hint<T>(data: &T) -> Option<RecordHint>
where
  T: AsRef<[u8]> + ?Sized,
{
  RecordHint::new(data.as_ref())
}

pub fn default_hint() -> RecordHint {
  // unwrap is okay, the hint is <= 24 bytes
  RecordHint::new([0; 24]).unwrap()
}
