// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use zeroize::Zeroize;

use crate::crypto::KeyRef;
use crate::crypto::KeyType;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::error::Result;
use crate::utils::generate_ed25519_keypair;

/// A convenient type for representing a pair of cryptographic keys.
#[derive(Clone, Debug)]
pub struct KeyPair {
  type_: KeyType,
  public: PublicKey,
  secret: SecretKey,
}

impl KeyPair {
  /// Creates a new [`Ed25519`][`KeyType::Ed25519`] [`KeyPair`].
  pub fn new_ed25519() -> Result<Self> {
    Self::new(KeyType::Ed25519)
  }

  /// Creates a new [`KeyPair`] with the given [`key type`][`KeyType`].
  pub fn new(type_: KeyType) -> Result<Self> {
    let (public, secret): (PublicKey, SecretKey) = match type_ {
      KeyType::Ed25519 => generate_ed25519_keypair()?,
    };

    Ok(Self { type_, public, secret })
  }

  /// Returns the [`type`][`KeyType`] of the `KeyPair` object.
  pub const fn type_(&self) -> KeyType {
    self.type_
  }

  /// Returns a reference to the [`PublicKey`] object.
  pub const fn public(&self) -> &PublicKey {
    &self.public
  }

  /// Returns the public key as a [`KeyRef`] object.
  pub fn public_ref(&self) -> KeyRef<'_> {
    KeyRef::new(self.type_, self.public.as_ref())
  }

  /// Returns a reference to the [`SecretKey`] object.
  pub const fn secret(&self) -> &SecretKey {
    &self.secret
  }

  /// Returns the secret key as a [`KeyRef`] object.
  pub fn secret_ref(&self) -> KeyRef<'_> {
    KeyRef::new(self.type_, self.secret.as_ref())
  }
}

impl Drop for KeyPair {
  fn drop(&mut self) {
    self.public.zeroize();
    self.secret.zeroize();
  }
}

impl Zeroize for KeyPair {
  fn zeroize(&mut self) {
    self.public.zeroize();
    self.secret.zeroize();
  }
}

impl From<(KeyType, PublicKey, SecretKey)> for KeyPair {
  fn from(other: (KeyType, PublicKey, SecretKey)) -> Self {
    Self {
      type_: other.0,
      public: other.1,
      secret: other.2,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_ed25519() {
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    assert_eq!(keypair.type_(), KeyType::Ed25519);
    assert_eq!(keypair.public().as_ref().len(), 32);
    assert_eq!(keypair.secret().as_ref().len(), 32);
  }
}
