// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::diff::Diff;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::jwk::JwkUse;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct DiffJwkUse(#[serde(skip_serializing_if = "Option::is_none")] Option<JwkUse>);

impl Diff for JwkUse {
  type Type = DiffJwkUse;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    if self == other {
      Ok(DiffJwkUse(None))
    } else {
      Ok(DiffJwkUse(Some(*other)))
    }
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    Ok(diff.0.unwrap_or(*self))
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    diff
      .0
      .ok_or_else(|| identity_core::diff::Error::ConversionError("cannot convert from empty diff".to_owned()))
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffJwkUse(Some(self)))
  }
}
