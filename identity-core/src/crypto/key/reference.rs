// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::KeyType;

/// A borrowed reference to a cryptographic key.
#[derive(Clone, Copy, Debug)]
pub struct KeyRef<'key> {
  kty: KeyType,
  key: &'key [u8],
}

impl<'key> KeyRef<'key> {
  /// Creates a new [`KeyRef`] object.
  pub fn new(kty: KeyType, key: &'key [u8]) -> Self {
    Self { kty, key }
  }

  /// Returns the [`KeyType`] of the key reference.
  pub const fn kty(&self) -> KeyType {
    self.kty
  }

  /// Returns the key material as a slice of bytes.
  pub const fn key(&self) -> &'key [u8] {
    self.key
  }
}
