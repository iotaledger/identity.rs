// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

/// The primary `Diff` Trait type.
pub trait Diff: Clone + Debug + PartialEq {
  /// The Corresponding Diff Type for the implemented Type.
  type Type: Sized + Clone + Debug + PartialEq + for<'de> Deserialize<'de> + Serialize;

  /// Finds the difference between two types; `self` and `other` and returns `Self::Type`
  fn diff(&self, other: &Self) -> crate::Result<Self::Type>;

  /// Merges a `Self::Type` with `Self`
  fn merge(&self, diff: Self::Type) -> crate::Result<Self>;

  /// Converts a `diff` of type `Self::Type` to a `Self`.
  fn from_diff(diff: Self::Type) -> crate::Result<Self>;

  /// Converts a type of `Self` to a `diff` of `Self::Type`.
  fn into_diff(self) -> crate::Result<Self::Type>;
}
