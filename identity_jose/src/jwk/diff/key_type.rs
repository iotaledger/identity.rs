// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::diff::Diff;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::jwk::JwkType;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffJwkType(#[serde(skip_serializing_if = "Option::is_none")] Option<JwkType>);

impl Diff for JwkType {
  type Type = DiffJwkType;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    if self == other {
      Ok(DiffJwkType(None))
    } else {
      Ok(DiffJwkType(Some(*other)))
    }
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    Ok(diff.0.unwrap_or_else(|| *self))
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    diff
      .0
      .ok_or_else(|| identity_core::diff::Error::ConversionError("cannot convert from empty diff".to_owned()))
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffJwkType(Some(self)))
  }
}
