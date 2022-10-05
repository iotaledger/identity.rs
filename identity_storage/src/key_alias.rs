// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// An alias for a secret key in KeyStorage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyAlias {
  alias: String,
}

impl KeyAlias {
  pub fn new(alias: impl Into<String>) -> Self {
    Self { alias: alias.into() }
  }
}

impl std::fmt::Display for KeyAlias {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{alias}", alias = self.alias))
  }
}
