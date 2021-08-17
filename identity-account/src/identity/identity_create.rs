// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::{KeyType, SecretKey};

/// Configuration used to create a new Identity.
#[derive(Clone, Debug)]
pub struct IdentityCreate {
  pub(crate) key_type: KeyType,
  pub(crate) name: Option<String>,
  pub(crate) network: Option<String>,
  pub(crate) secret_key: Option<SecretKey>,
}

impl IdentityCreate {
  /// Creates a new `IdentityCreate` instance.
  pub const fn new() -> Self {
    Self {
      key_type: KeyType::Ed25519,
      name: None,
      network: None,
      secret_key: None,
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

  /// Sets the [SecretKey] for the Identity creation.
  /// This needs to be the correct number of bytes for the respective [KeyType].
  ///
  /// For [KeyType::Ed25519] the key needs to be 32 bytes in size.
  #[must_use]
  pub fn secret_key(mut self, value: SecretKey) -> Self {
    self.secret_key = Some(value);
    self
  }
}

impl Default for IdentityCreate {
  fn default() -> Self {
    Self::new()
  }
}
