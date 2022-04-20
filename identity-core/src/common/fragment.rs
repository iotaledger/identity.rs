// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

use serde;
use serde::Deserialize;
use serde::Serialize;

/// Represents a DID URL fragment.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(from = "String", into = "String")]
#[repr(transparent)]
pub struct Fragment(String);

impl Fragment {
  const PREFIX: char = '#';

  /// Creates a new Fragment from the given `value`.
  pub fn new(value: impl Into<String>) -> Self {
    let value = value.into();
    if value.starts_with(Self::PREFIX) {
      Self(value)
    } else {
      Self(format!("{}{}", Self::PREFIX, value))
    }
  }

  /// Returns the complete fragment identifier.
  pub fn identifier(&self) -> &str {
    &self.0
  }

  /// Returns the fragment name.
  pub fn name(&self) -> &str {
    assert!(!self.0.is_empty());
    &self.0[1..]
  }
}

impl Debug for Fragment {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("Fragment({})", self.name()))
  }
}

impl Display for Fragment {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.name())
  }
}

impl From<String> for Fragment {
  fn from(other: String) -> Self {
    Self::new(other)
  }
}

impl From<Fragment> for String {
  fn from(other: Fragment) -> Self {
    other.0
  }
}
