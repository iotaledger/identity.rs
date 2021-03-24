// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use identity_core::diff::Diff;
use identity_core::diff::DiffVec;
use identity_core::diff::Error;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::OrderedSet;

impl<T> Diff for OrderedSet<T>
where
  T: Diff + Serialize + for<'de> Deserialize<'de>,
{
  type Type = DiffVec<T>;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    self.clone().into_vec().diff(&other.clone().into_vec())
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    self
      .clone()
      .into_vec()
      .merge(diff)
      .and_then(|this| Self::try_from(this).map_err(Error::merge))
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    Vec::from_diff(diff).and_then(|this| Self::try_from(this).map_err(Error::convert))
  }

  fn into_diff(self) -> Result<Self::Type> {
    self.into_vec().into_diff()
  }
}
