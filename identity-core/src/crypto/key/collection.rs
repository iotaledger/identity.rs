// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::iter::Zip;
use core::ops::Index;
use core::ops::IndexMut;
use core::slice::Iter;
use core::slice::SliceIndex;
use std::vec::IntoIter;

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::SigningKey;
use crate::crypto::merkle_tree::compute_merkle_proof;
use crate::crypto::merkle_tree::compute_merkle_root;
use crate::crypto::merkle_tree::DigestExt;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::KeyPair;
use crate::crypto::KeyRef;
use crate::crypto::KeyType;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::error::Error;
use crate::error::Result;
use crate::utils::generate_ed25519_keypairs;

/// A collection of cryptographic keys.
#[derive(Clone, Debug)]
pub struct KeyCollection {
  type_: KeyType,
  public: Box<[PublicKey]>,
  secret: Box<[SecretKey]>,
}

impl KeyCollection {
  /// Creates a new [`KeyCollection`] from an iterator of
  /// [`PublicKey`]/[`SecretKey`] pairs.
  pub fn from_iterator<I>(type_: KeyType, iter: I) -> Result<Self>
  where
    I: IntoIterator<Item = (PublicKey, SecretKey)>,
  {
    let (public, secret): (Vec<_>, Vec<_>) = iter.into_iter().unzip();

    if public.is_empty() {
      return Err(Error::InvalidKeyCollectionSize(public.len()));
    }

    if secret.is_empty() {
      return Err(Error::InvalidKeyCollectionSize(secret.len()));
    }

    Ok(Self {
      type_,
      public: public.into_boxed_slice(),
      secret: secret.into_boxed_slice(),
    })
  }

  /// Creates a new [`KeyCollection`] with [`Ed25519`][`KeyType::Ed25519`] keys.
  pub fn new_ed25519(count: usize) -> Result<Self> {
    Self::new(KeyType::Ed25519, count)
  }

  /// Creates a new [`KeyCollection`] with the given [`key type`][`KeyType`].
  pub fn new(type_: KeyType, count: usize) -> Result<Self> {
    let keys: Vec<(PublicKey, SecretKey)> = match type_ {
      KeyType::Ed25519 => generate_ed25519_keypairs(count)?,
    };

    Self::from_iterator(type_, keys.into_iter())
  }

  /// Returns the [`type`][`KeyType`] of the `KeyCollection` object.
  pub const fn type_(&self) -> KeyType {
    self.type_
  }

  /// Returns the number of keys in the collection.
  pub fn len(&self) -> usize {
    self.public.len()
  }

  /// Returns `true` if the collection contains no keys.
  pub fn is_empty(&self) -> bool {
    self.public.is_empty()
  }

  /// Returns a reference to the public key at the specified `index`.
  pub fn public(&self, index: usize) -> Option<&PublicKey> {
    self.public.get(index)
  }

  /// Returns a [`KeyRef`] object referencing the public key at the specified `index`.
  pub fn public_ref(&self, index: usize) -> Option<KeyRef<'_>> {
    self.public.get(index).map(|key| KeyRef::new(self.type_, key.as_ref()))
  }

  /// Returns a reference to the secret key at the specified `index`.
  pub fn secret(&self, index: usize) -> Option<&SecretKey> {
    self.secret.get(index)
  }

  /// Returns a [`KeyRef`] object referencing the secret key at the specified `index`.
  pub fn secret_ref(&self, index: usize) -> Option<KeyRef<'_>> {
    self.secret.get(index).map(|key| KeyRef::new(self.type_, key.as_ref()))
  }

  /// Returns a [`KeyPair`] object for the keys at the specified `index`.
  pub fn keypair(&self, index: usize) -> Option<KeyPair> {
    if let (Some(public), Some(secret)) = (self.public.get(index), self.secret.get(index)) {
      Some((self.type_, public.clone(), secret.clone()).into())
    } else {
      None
    }
  }

  /// Returns an iterator over the key pairs in the collection.
  pub fn iter(&self) -> impl Iterator<Item = (&PublicKey, &SecretKey)> {
    self.public.iter().zip(self.secret.iter())
  }

  /// Returns an iterator over the public keys in the collection.
  pub fn iter_public(&self) -> Iter<'_, PublicKey> {
    self.public.iter()
  }

  /// Returns an iterator over the secret keys in the collection.
  pub fn iter_secret(&self) -> Iter<'_, SecretKey> {
    self.secret.iter()
  }

  /// Returns the Merkle root hash of the public keys in the collection.
  pub fn merkle_root<D>(&self) -> Hash<D>
  where
    D: DigestExt,
  {
    compute_merkle_root(&self.public)
  }

  /// Returns a proof-of-inclusion for the public key at the specified index.
  pub fn merkle_proof<D>(&self, index: usize) -> Option<Proof<D>>
  where
    D: DigestExt,
  {
    compute_merkle_proof(&self.public, index)
  }

  /// Returns a Merkle Key [`SigningKey`] for the key pair at the
  /// specified `index`.
  pub fn merkle_key<D>(&self, index: usize) -> Option<SigningKey<'_, D>>
  where
    D: MerkleDigest,
  {
    let proof: Proof<D> = self.merkle_proof(index)?;
    let public: &PublicKey = self.public(index)?;
    let secret: &SecretKey = self.secret(index)?;

    Some(SigningKey::from_owned(public, secret, proof))
  }

  /// Creates a DID Document public key value for the Merkle root of
  /// the key collection.
  pub fn encode_merkle_key<D>(&self) -> Vec<u8>
  where
    D: MerkleDigest,
  {
    self.type_.encode_merkle_key::<D>(&self.merkle_root())
  }
}

impl<I> Index<I> for KeyCollection
where
  I: SliceIndex<[PublicKey]>,
{
  type Output = <I as SliceIndex<[PublicKey]>>::Output;

  fn index(&self, index: I) -> &Self::Output {
    self.public.index(index)
  }
}

impl<I> IndexMut<I> for KeyCollection
where
  I: SliceIndex<[PublicKey]>,
{
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.public.index_mut(index)
  }
}

impl IntoIterator for KeyCollection {
  type Item = (PublicKey, SecretKey);
  type IntoIter = Zip<IntoIter<PublicKey>, IntoIter<SecretKey>>;

  fn into_iter(self) -> Self::IntoIter {
    self.public.to_vec().into_iter().zip(self.secret.to_vec().into_iter())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ed25519() {
    let keys: KeyCollection = KeyCollection::new_ed25519(100).unwrap();

    assert_eq!(keys.len(), 100);
    assert!(!keys.is_empty());

    let public: Vec<_> = keys.iter_public().cloned().collect();
    let secret: Vec<_> = keys.iter_secret().cloned().collect();

    assert_eq!(public.len(), keys.len());
    assert_eq!(secret.len(), keys.len());

    for (index, (public, secret)) in public.iter().zip(secret.iter()).enumerate() {
      assert_eq!(public.as_ref(), keys.public(index).unwrap().as_ref());
      assert_eq!(secret.as_ref(), keys.secret(index).unwrap().as_ref());
    }

    let iter: _ = public.into_iter().zip(secret.into_iter());
    let next: KeyCollection = KeyCollection::from_iterator(keys.type_(), iter).unwrap();

    assert_eq!(next.len(), keys.len());

    let public: Vec<_> = next.iter_public().cloned().collect();
    let secret: Vec<_> = next.iter_secret().cloned().collect();

    for (index, (public, secret)) in public.iter().zip(secret.iter()).enumerate() {
      assert_eq!(public.as_ref(), keys.public(index).unwrap().as_ref());
      assert_eq!(secret.as_ref(), keys.secret(index).unwrap().as_ref());
    }
  }
}
