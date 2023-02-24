// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

/// The type or class of a cryptographic key.
///
/// Each storage implementation should expose constants of this type to signal
/// which key types are supported.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KeyType(Cow<'static, str>);

impl KeyType {
  /// Creates a new algorithm from a string.
  pub fn new(algorithm: impl Into<String>) -> Self {
    Self(Cow::Owned(algorithm.into()))
  }

  /// Creates a new algorithm from a static string.
  ///
  /// This can used in const contexts.
  pub const fn from_static_str(algorithm: &'static str) -> Self {
    Self(Cow::Borrowed(algorithm))
  }

  /// Returns the string representation of the key type.
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

impl From<KeyType> for String {
  fn from(value: KeyType) -> Self {
    value.0.into()
  }
}

impl std::fmt::Display for KeyType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}
