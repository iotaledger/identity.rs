// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use crate::Diff;
use std::fmt::Debug;
use std::fmt::Formatter;

/// The Diff Type for a `String` type.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct DiffString(#[serde(skip_serializing_if = "Option::is_none")] pub Option<String>);

/// `Diff` trait implementation for `String`.
impl Diff for String {
  /// Diff type for `String`
  type Type = DiffString;

  /// compares two `String` types; `self`, `other` and returns a `DiffString` type.
  fn diff(&self, other: &Self) -> crate::Result<Self::Type> {
    if self == other {
      Ok(DiffString(None))
    } else {
      other.clone().into_diff()
    }
  }

  /// Merges a `DiffString`; `diff` with a `String`; `self`.
  fn merge(&self, diff: Self::Type) -> crate::Result<Self> {
    if diff.0.is_none() {
      Ok(self.to_string())
    } else {
      Self::from_diff(diff)
    }
  }

  /// Converts a `DiffString` into a `String` type.
  fn from_diff(diff: Self::Type) -> crate::Result<Self> {
    match diff.0 {
      Some(s) => Ok(s),
      None => Err(crate::Error::ConversionError(
        "Problem converting from DiffString".into(),
      )),
    }
  }

  /// Converts a `String` into a `DiffString` type.
  fn into_diff(self) -> crate::Result<Self::Type> {
    Ok(DiffString(Some(self)))
  }
}

/// Debug trait implementation for DiffString.
impl Debug for DiffString {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match &self.0 {
      Some(val) => write!(f, "DiffString({val:#?})"),
      None => write!(f, "DiffString None"),
    }
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string_diff() {
    let sa = String::from("test");
    let sb = String::from("another_string");

    let diff = sa.diff(&sb).unwrap();

    assert_eq!(diff, DiffString(Some("another_string".into())));

    let sc = sa.merge(diff).unwrap();

    assert_eq!(sb, sc);
  }

  #[test]
  fn test_same_string() {
    let sa = String::from("test");
    let sb = String::from("test");

    let diff = sa.diff(&sb).unwrap();

    assert_eq!(diff, DiffString(None));

    let sc = sa.merge(diff).unwrap();

    assert_eq!(sb, sc);
    assert_eq!(sa, sc);
  }
}
