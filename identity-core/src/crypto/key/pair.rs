// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryInto;

use crypto::signatures::ed25519;
use zeroize::Zeroize;

use crate::crypto::KeyRef;
use crate::crypto::KeyType;
use crate::crypto::PrivateKey;
use crate::crypto::PublicKey;
use crate::error::Result;
use crate::utils::generate_ed25519_keypair;
use crate::utils::keypair_from_ed25519_private_key;

/// A convenient type for representing a pair of cryptographic keys.
#[derive(Clone, Debug)]
pub struct KeyPair {
  type_: KeyType,
  public: PublicKey,
  private: PrivateKey,
}

impl KeyPair {
  /// Creates a new [`Ed25519`][`KeyType::Ed25519`] [`KeyPair`].
  pub fn new_ed25519() -> Result<Self> {
    Self::new(KeyType::Ed25519)
  }

  /// Creates a new [`KeyPair`] with the given [`key type`][`KeyType`].
  pub fn new(type_: KeyType) -> Result<Self> {
    let (public, private): (PublicKey, PrivateKey) = match type_ {
      KeyType::Ed25519 => generate_ed25519_keypair()?,
    };

    Ok(Self { type_, public, private })
  }

  /// Reconstructs the [`Ed25519`][`KeyType::Ed25519`] [`KeyPair`] from a private key.
  pub fn try_from_ed25519_bytes(private_key_bytes: &[u8]) -> Result<Self, crypto::Error> {
    let private_key_bytes: [u8; ed25519::SECRET_KEY_LENGTH] = private_key_bytes
      .try_into()
      .map_err(|_| crypto::Error::PrivateKeyError)?;

    let private_key = ed25519::SecretKey::from_bytes(private_key_bytes);

    let (public, private) = keypair_from_ed25519_private_key(private_key);

    Ok(Self {
      type_: KeyType::Ed25519,
      public,
      private,
    })
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

  /// Returns a reference to the [`PrivateKey`] object.
  pub const fn private(&self) -> &PrivateKey {
    &self.private
  }

  /// Returns the private key as a [`KeyRef`] object.
  pub fn private_ref(&self) -> KeyRef<'_> {
    KeyRef::new(self.type_, self.private.as_ref())
  }
}

impl Drop for KeyPair {
  fn drop(&mut self) {
    self.public.zeroize();
    self.private.zeroize();
  }
}

impl Zeroize for KeyPair {
  fn zeroize(&mut self) {
    self.public.zeroize();
    self.private.zeroize();
  }
}

impl From<(KeyType, PublicKey, PrivateKey)> for KeyPair {
  fn from(other: (KeyType, PublicKey, PrivateKey)) -> Self {
    Self {
      type_: other.0,
      public: other.1,
      private: other.2,
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
    assert_eq!(keypair.private().as_ref().len(), 32);
  }
}
