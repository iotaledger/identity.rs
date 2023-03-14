// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use std::fmt::Debug;
use std::fmt::Formatter;

use crate::Diff;

/// A `DiffOption<T>` type which represents a Diffed `Option<T>`.  By default this value is untagged for `serde`. It
/// also converts `to` and `from` `Option<T>` when serialized/deserialized
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged, into = "Option<T>", from = "Option<T>")]
pub enum DiffOption<T: Diff> {
  Some(<T as Diff>::Type),
  None,
}

/// `Diff` Implementation for `Option<T>`
impl<T> Diff for Option<T>
where
  T: Diff + Clone + Debug + PartialEq + for<'de> Deserialize<'de> + Serialize,
{
  /// The Corresponding Diff type for `Option<T>`
  type Type = DiffOption<T>;

  /// Compares two `Option<T>` types; `self` and `other` and finds the Difference between them, returning a
  /// `DiffOption<T>` type.
  fn diff(&self, other: &Self) -> crate::Result<Self::Type> {
    match (self, other) {
      (Some(x), Some(y)) => Ok(Self::Type::Some(x.diff(y)?)),
      (None, Some(y)) => Ok(Self::Type::Some(y.clone().into_diff()?)),
      _ => Ok(Self::Type::None),
    }
  }

  /// Merges a `DiffOption<T>`; `diff` type with an `Option<T>` type; `self`.
  fn merge(&self, diff: Self::Type) -> crate::Result<Self> {
    match (self, diff) {
      (None, DiffOption::None) => Ok(None),
      (Some(_), DiffOption::None) => Ok(None),
      (None, DiffOption::Some(ref d)) => Ok(Some(<T>::from_diff(d.clone())?)),
      (Some(t), DiffOption::Some(ref d)) => Ok(Some(t.merge(d.clone())?)),
    }
  }

  /// converts a `DiffOption<T>`; `diff` to an `Option<T>` type.
  fn from_diff(diff: Self::Type) -> crate::Result<Self> {
    match diff {
      Self::Type::None => Ok(None),
      Self::Type::Some(diff) => Ok(Some(<T>::from_diff(diff)?)),
    }
  }

  /// converts a `Option<T>`; `self` to an `DiffOption<T>` type.
  fn into_diff(self) -> crate::Result<Self::Type> {
    match self {
      Self::None => Ok(DiffOption::None),
      Self::Some(t) => Ok(DiffOption::Some(t.into_diff()?)),
    }
  }
}

/// Debug implementation for `DiffOption<T>`.
impl<T: Diff> std::fmt::Debug for DiffOption<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match &self {
      Self::Some(d) => write!(f, "DiffOption::Some({d:#?})"),
      Self::None => write!(f, "DiffOption::None"),
    }
  }
}

/// Default implementation for `DiffOption<T>`.
impl<T: Diff> Default for DiffOption<T> {
  fn default() -> Self {
    Self::None
  }
}

/// From `DiffOption<T>` implementation for `Option<T>`.
impl<T> From<DiffOption<T>> for Option<T>
where
  T: Diff,
{
  fn from(other: DiffOption<T>) -> Self {
    match other {
      DiffOption::Some(s) => Some(Diff::from_diff(s).expect("Unable to convert from diff")),
      DiffOption::None => None,
    }
  }
}

/// From `Option<T>` implementation for `DiffOption<T>`.
impl<T> From<Option<T>> for DiffOption<T>
where
  T: Diff,
{
  fn from(opt: Option<T>) -> Self {
    match opt {
      Some(s) => DiffOption::Some(s.into_diff().expect("Unable to convert to diff")),
      None => DiffOption::None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::string::DiffString;

  #[test]
  fn test_option_diff() {
    let a = Some("A".to_owned());
    let b = Some("B".to_owned());

    let diff = a.diff(&b).unwrap();

    assert_eq!(diff, DiffOption::Some(DiffString(Some("B".to_owned()))));

    let c = a.merge(diff).unwrap();

    assert_eq!(b, c);
  }
}
