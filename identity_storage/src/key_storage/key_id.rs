// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// An identifier for a private key stored in a [`KeyStorage`](crate::key_storage::KeyStorage).
///
/// This type is returned by a [`KeyStorage`](crate::key_storage::KeyStorage) implementation when
/// generating cryptographic key pairs and later used as a parameter when signing data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KeyId(String);

impl KeyId {
  pub fn new(id: impl Into<String>) -> Self {
    Self(id.into())
  }

  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl std::fmt::Display for KeyId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}
