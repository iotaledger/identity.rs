// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::diff::Diff;
use identity_core::diff::Result;

use crate::jwk::JwkUse;

impl Diff for JwkUse {
  type Type = JwkUse;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    Ok(*other)
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    Ok(diff)
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    Ok(diff)
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(self)
  }
}
