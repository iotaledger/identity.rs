// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::BitSet;
use identity_core::common::Object;
use identity_core::convert::FromJson;

use crate::did::DID;
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

impl<D, T> Revocation for VerificationMethod<D, T>
where
  D: DID,
  T: Revocation,
{
  fn revocation(&self) -> Result<Option<BitSet>> {
    self.properties().revocation()
  }
}
