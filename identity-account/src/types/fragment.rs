// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
pub struct Fragment(String);

impl Fragment {
  pub const PREFIX: char = '#';

  pub fn new(value: String) -> Self {
    if value.starts_with(Self::PREFIX) {
      Self(value)
    } else {
      Self(format!("{}{}", Self::PREFIX, value))
    }
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn ident(&self) -> &str {
    &self.0
  }

  pub fn value(&self) -> &str {
    assert!(!self.0.is_empty());
    &self.0[1..]
  }
}

impl Debug for Fragment {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("Fragment({})", self.value()))
  }
}

impl Display for Fragment {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.value())
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
