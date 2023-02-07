// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// An identifier for a private key stored in a [`KeyStorage`](crate::key_storage::KeyStorage).
///
/// This type is returned by a [`KeyStorage`](crate::key_storage::KeyStorage) implementation when
/// generating cryptographic key pairs and later used as a parameter when signing data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KeyId(String);

impl KeyId {
  pub fn new(alias: impl Into<String>) -> Self {
    Self(alias.into())
  }

  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl TryFrom<Vec<u8>> for KeyId {
  type Error = ();

  fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
    Ok(KeyId(String::from_utf8(bytes).expect("TODO")))
  }
}

impl std::fmt::Display for KeyId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}
