// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::BitSet;
use identity_core::common::Object;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;

use crate::did::DID;
use crate::error::Result;
use crate::verification::VerificationMethod;

pub trait Revocation {
  /// Returns a [`set`][BitSet] of Merkle Key Collection revocation flags.
  fn revocation(&self) -> Result<Option<BitSet>>;

  /// Sets the [`set`][BitSet] of Merkle Key Collection revocation flags, removes
  /// it completely if `None`.
  fn set_revocation(&mut self, revocation: Option<BitSet>) -> Result<()>;
}

impl Revocation for Object {
  fn revocation(&self) -> Result<Option<BitSet>> {
    self
      .get("revocation")
      .cloned()
      .map(BitSet::from_json_value)
      .transpose()
      .map_err(Into::into)
  }

  fn set_revocation(&mut self, revocation: Option<BitSet>) -> Result<()> {
    match revocation {
      Some(bitset) => {
        self.insert("revocation".to_owned(), bitset.to_json_value()?);
      }
      None => {
        let _ = self.remove("revocation");
      }
    }
    Ok(())
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

  fn set_revocation(&mut self, revocation: Option<BitSet>) -> Result<()> {
    self.properties_mut().set_revocation(revocation)
  }
}
