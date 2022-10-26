// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// An identifier for a secret key in KeyStorage.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KeyId {
  alias: String,
}

impl KeyId {
  pub fn new(alias: impl Into<String>) -> Self {
    Self { alias: alias.into() }
  }

  pub fn as_str(&self) -> &str {
    &self.alias
  }
}

impl TryFrom<Vec<u8>> for KeyId {
  type Error = ();

  fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
    Ok(KeyId {
      alias: String::from_utf8(bytes).expect("TODO"),
    })
  }
}

impl std::fmt::Display for KeyId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.alias)
  }
}
