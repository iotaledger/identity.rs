// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::diff::Diff;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::jwk::JwkOperation;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DiffJwkOperation(#[serde(skip_serializing_if = "Option::is_none")] Option<JwkOperation>);

impl Diff for JwkOperation {
  type Type = DiffJwkOperation;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    if self == other {
      Ok(DiffJwkOperation(None))
    } else {
      Ok(DiffJwkOperation(Some(*other)))
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
    Ok(DiffJwkOperation(Some(self)))
  }
}
