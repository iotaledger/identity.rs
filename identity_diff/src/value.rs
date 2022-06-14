// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::Diff;

/// The Diff Type for `serde_json::Value`.
#[derive(Clone, Default, Eq, Serialize, Deserialize, PartialEq, Debug)]
#[serde(transparent)]
pub struct DiffValue(#[serde(skip_serializing_if = "Option::is_none")] pub Option<Value>);

/// The Diff implementation for `serde_json::Value`.
impl Diff for Value {
  /// The Diff Type for `serde_json::Value`.
  type Type = DiffValue;

  /// Compares two `serde_json::Value` types; `self`, `diff` and outputs a `DiffValue` type.
  fn diff(&self, other: &Self) -> crate::Result<Self::Type> {
    if self == other {
      Ok(DiffValue(None))
    } else {
      other.clone().into_diff()
    }
  }

  /// Merges a `DiffValue`; `diff` with `self`; a `serde_json::Value` to create a new `serde_json::Value`.
  fn merge(&self, diff: Self::Type) -> crate::Result<Self> {
    if diff.0.is_none() {
      Ok(self.clone())
    } else {
      Self::from_diff(diff)
    }
  }

  /// Converts from a `diff` of type `DiffValue` to a `serde_json::Value`.
  fn from_diff(diff: Self::Type) -> crate::Result<Self> {
    match diff.0 {
      Some(s) => Ok(s),
      None => Err(crate::Error::ConversionError(
        "Error converting from serde_json::Value".into(),
      )),
    }
  }

  /// converts a `serde_json::Value` to a `DiffValue`.
  fn into_diff(self) -> crate::Result<Self::Type> {
    Ok(DiffValue(Some(self)))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use serde_json::json;

  #[test]
  fn test_json_value() {
    let v = Value::Null;

    let v2 = Value::Bool(true);

    let diff = v.diff(&v2).unwrap();

    let res = v.merge(diff).unwrap();

    let expected = Value::Bool(true);

    assert_eq!(expected, res);

    let v = json!("A string");

    let v2 = json!("A string");

    let diff = v.diff(&v2).unwrap();

    let res = v.merge(diff).unwrap();

    assert_eq!(res, v2);

    let v3 = json!("Another string");

    let diff = v.diff(&v3).unwrap();

    let res = v.merge(diff).unwrap();

    assert_eq!(v3, res);
  }
}
