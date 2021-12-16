// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::BitSet;
use identity_core::common::Object;
use identity_core::convert::FromJson;

use crate::error::Result;
use crate::verification::VerificationMethod;

pub trait Revocation {
  /// Returns a [`set`][BitSet] of Merkle Key Collection revocation flags.
  fn revocation(&self) -> Result<Option<BitSet>> {
    Ok(None)
  }
}

impl Revocation for () {}

impl Revocation for Object {
  fn revocation(&self) -> Result<Option<BitSet>> {
    self
      .get("revocation")
      .cloned()
      .map(BitSet::from_json_value)
      .transpose()
      .map_err(Into::into)
  }
}

impl<T> Revocation for VerificationMethod<T>
where
  T: Revocation,
{
  fn revocation(&self) -> Result<Option<BitSet>> {
    self.properties().revocation()
  }
}
