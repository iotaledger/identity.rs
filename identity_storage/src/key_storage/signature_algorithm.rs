// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

/// A signing algorithm passed to a [`KeyStorage`](crate::key_storage::KeyStorage).
///
/// This type essentially a wrapper around a string and is used to communicate between
/// users and `KeyStorage` implementers. `KeyStorage` implementers can define constants
/// that represent the algorithms they support.
// # Example: TODO
// ```
// /// The Ed25519 Signing Algorithm.
//   pub const ED25519_SIGNING_ALGORITHM: SigningAlgorithm = SigningAlgorithm::from_static_str("Ed25519");
//
// // ...
//
// key_storage.sign(ED25519_SIGNING_ALGORITHM, ...).await?;
// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SignatureAlgorithm(Cow<'static, str>);

impl SignatureAlgorithm {
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

impl From<String> for SignatureAlgorithm {
  fn from(algorithm: String) -> Self {
    Self(Cow::Owned(algorithm))
  }
}

impl From<&'static str> for SignatureAlgorithm {
  fn from(algorithm: &'static str) -> Self {
    Self(Cow::Borrowed(algorithm))
  }
}

impl std::fmt::Display for SignatureAlgorithm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}
