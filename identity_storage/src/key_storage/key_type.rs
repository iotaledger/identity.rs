// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KeyType(Cow<'static, str>);

impl KeyType {
  pub fn new(algorithm: impl Into<String>) -> Self {
    Self(Cow::Owned(algorithm.into()))
  }

  pub const fn from_static_str(algorithm: &'static str) -> Self {
    Self(Cow::Borrowed(algorithm))
  }

  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl From<String> for KeyType {
  fn from(algorithm: String) -> Self {
    Self(Cow::Owned(algorithm))
  }
}

impl From<&'static str> for KeyType {
  fn from(algorithm: &'static str) -> Self {
    Self(Cow::Borrowed(algorithm))
  }
}

impl std::fmt::Display for KeyType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}
