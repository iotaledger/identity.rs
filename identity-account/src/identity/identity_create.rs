// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;

/// Configuration used to create a new Identity.
#[derive(Clone, Debug)]
pub struct IdentityCreate {
  pub(crate) key_type: KeyType,
  pub(crate) name: Option<String>,
  pub(crate) network: Option<String>,
  pub(crate) shard: Option<String>,
}

impl IdentityCreate {
  /// Creates a new `IdentityCreate` instance.
  pub const fn new() -> Self {
    Self {
      key_type: KeyType::Ed25519,
      name: None,
      network: None,
      shard: None,
    }
  }

  /// Sets the key type of the initial authentication method.
  #[must_use]
  pub fn key_type(mut self, value: KeyType) -> Self {
    self.key_type = value;
    self
  }

  /// Sets the name of the Identity.
  #[must_use]
  pub fn name<T>(mut self, value: T) -> Self
  where
    T: Into<String>,
  {
    self.name = Some(value.into());
    self
  }

  /// Sets the IOTA Tangle network of the Identity DID.
  #[must_use]
  pub fn network<T>(mut self, value: T) -> Self
  where
    T: Into<String>,
  {
    self.network = Some(value.into());
    self
  }

  /// Sets the IOTA Tangle shard of the Identity DID.
  #[must_use]
  pub fn shard<T>(mut self, value: T) -> Self
  where
    T: Into<String>,
  {
    self.shard = Some(value.into());
    self
  }
}

impl Default for IdentityCreate {
  fn default() -> Self {
    Self::new()
  }
}
