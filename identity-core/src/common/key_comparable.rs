// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A trait for comparing types only by a certain key.
pub trait KeyComparable {
  /// Key type for comparisons.
  type Key: PartialEq + ?Sized;

  /// Returns a reference to the key.
  fn key(&self) -> &Self::Key;
}
