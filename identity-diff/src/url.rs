// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use did_url::DID;

use crate::error::Error;
use crate::error::Result;
use crate::string::DiffString;
use crate::traits::Diff;

impl Diff for DID {
  type Type = DiffString;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    self.to_string().diff(&other.to_string())
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    self
      .to_string()
      .merge(diff)
      .and_then(|this| Self::parse(&this).map_err(Error::merge))
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    String::from_diff(diff).and_then(|this| Self::parse(&this).map_err(Error::convert))
  }

  fn into_diff(self) -> Result<Self::Type> {
    self.to_string().into_diff()
  }
}
