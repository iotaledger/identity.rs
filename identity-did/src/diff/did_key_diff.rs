// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::diff::Diff;
use identity_core::diff::Result;

use crate::did::DID;
use crate::utils::DIDKey;

impl<T> Diff for DIDKey<T>
where
  T: AsRef<DID> + Diff,
{
  type Type = <T as Diff>::Type;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    self.clone().into_inner().diff(&other.clone().into_inner())
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    self.clone().into_inner().merge(diff).map(Self::new)
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    T::from_diff(diff).map(Self::new)
  }

  fn into_diff(self) -> Result<Self::Type> {
    self.into_inner().into_diff()
  }
}
