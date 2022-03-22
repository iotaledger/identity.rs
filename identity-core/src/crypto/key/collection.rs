// Copyright 2020-2022 IOTA Stiftung
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
use crate::crypto::PrivateKey;
use crate::crypto::PublicKey;
use crate::error::Error;
use crate::error::Result;
use crate::utils::generate_ed25519_keypairs;

/// Defines an upper limit to the amount of keys that can be created (2^12)
/// This value respects a current stronghold limitation
const MAX_KEYS_ALLOWED: usize = 4_096;

/// A collection of cryptographic keys.
#[derive(Clone, Debug)]
pub struct KeyCollection {
  type_: KeyType,
  public: Box<[PublicKey]>,
  private: Box<[PrivateKey]>,
}

impl KeyCollection {
  /// Creates a new [`KeyCollection`] from an iterator of
  /// [`PublicKey`]/[`PrivateKey`] pairs.
  pub fn from_iterator<I>(type_: KeyType, iter: I) -> Result<Self>
  where
    I: IntoIterator<Item = (PublicKey, PrivateKey)>,
  {
    let (public, private): (Vec<_>, Vec<_>) = iter.into_iter().unzip();

    if public.is_empty() {
      return Err(Error::InvalidKeyCollectionSize(public.len()));
    }

    if private.is_empty() {
      return Err(Error::InvalidKeyCollectionSize(private.len()));
    }

    Ok(Self {
      type_,
      public: public.into_boxed_slice(),
      private: private.into_boxed_slice(),
    })
  }

  /// Creates a new [`KeyCollection`] with [`Ed25519`][`KeyType::Ed25519`] keys.
  /// If `count` is not a power of two, with the exception of 0, which will result in an error,
  /// it will be rounded up to the next one.
  /// E.g. 230 -> 256
  pub fn new_ed25519(count: usize) -> Result<Self> {
    Self::new(KeyType::Ed25519, count)
  }

  /// Creates a new [`KeyCollection`] with the given [`key type`][`KeyType`].
  /// If `count` is not a power of two, with the exception of 0, which will result in an error,
  /// it will be rounded up to the next one.
  /// E.g. 230 -> 256
  pub fn new(type_: KeyType, count: usize) -> Result<Self> {
    if count == 0 {
      return Err(Error::InvalidKeyCollectionSize(0));
    }
    let count_next_power = count.checked_next_power_of_two().unwrap_or(0);
    if count_next_power == 0 || count_next_power > MAX_KEYS_ALLOWED {
      return Err(Error::InvalidKeyCollectionSize(count_next_power));
    }

    let keys: Vec<(PublicKey, PrivateKey)> = match type_ {
      KeyType::Ed25519 => generate_ed25519_keypairs(count_next_power)?,
      KeyType::X25519 => unimplemented!("x25519 not supported"),
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

  /// Returns a reference to the private key at the specified `index`.
  pub fn private(&self, index: usize) -> Option<&PrivateKey> {
    self.private.get(index)
  }

  /// Returns a [`KeyRef`] object referencing the private key at the specified `index`.
  pub fn private_ref(&self, index: usize) -> Option<KeyRef<'_>> {
    self.private.get(index).map(|key| KeyRef::new(self.type_, key.as_ref()))
  }

  /// Returns a [`KeyPair`] object for the keys at the specified `index`.
  pub fn keypair(&self, index: usize) -> Option<KeyPair> {
    if let (Some(public), Some(private)) = (self.public.get(index), self.private.get(index)) {
      Some((self.type_, public.clone(), private.clone()).into())
    } else {
      None
    }
  }

  /// Returns an iterator over the key pairs in the collection.
  pub fn iter(&self) -> impl Iterator<Item = (&PublicKey, &PrivateKey)> {
    self.public.iter().zip(self.private.iter())
  }

  /// Returns an iterator over the public keys in the collection.
  pub fn iter_public(&self) -> Iter<'_, PublicKey> {
    self.public.iter()
  }

  /// Returns an iterator over the private keys in the collection.
  pub fn iter_private(&self) -> Iter<'_, PrivateKey> {
    self.private.iter()
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
    let private: &PrivateKey = self.private(index)?;

    Some(SigningKey::from_owned(public, private, proof))
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
  type Item = (PublicKey, PrivateKey);
  type IntoIter = Zip<IntoIter<PublicKey>, IntoIter<PrivateKey>>;

  // Vec conversion is necessary for Box<[T]>, see https://github.com/rust-lang/rust/issues/59878
  fn into_iter(self) -> Self::IntoIter {
    self
      .public
      .into_vec()
      .into_iter()
      .zip(self.private.into_vec().into_iter())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ed25519() {
    let keys: KeyCollection = KeyCollection::new_ed25519(100).unwrap();

    assert_eq!(keys.len(), 128);
    assert!(!keys.is_empty());

    let public: Vec<_> = keys.iter_public().cloned().collect();
    let private: Vec<_> = keys.iter_private().cloned().collect();

    assert_eq!(public.len(), keys.len());
    assert_eq!(private.len(), keys.len());

    for (index, (public, private)) in public.iter().zip(private.iter()).enumerate() {
      assert_eq!(public.as_ref(), keys.public(index).unwrap().as_ref());
      assert_eq!(private.as_ref(), keys.private(index).unwrap().as_ref());
    }

    let iter: _ = public.into_iter().zip(private.into_iter());
    let next: KeyCollection = KeyCollection::from_iterator(keys.type_(), iter).unwrap();

    assert_eq!(next.len(), keys.len());

    let public: Vec<_> = next.iter_public().cloned().collect();
    let private: Vec<_> = next.iter_private().cloned().collect();

    for (index, (public, private)) in public.iter().zip(private.iter()).enumerate() {
      assert_eq!(public.as_ref(), keys.public(index).unwrap().as_ref());
      assert_eq!(private.as_ref(), keys.private(index).unwrap().as_ref());
    }
  }

  #[test]
  fn test_key_collection_size() {
    // Key Collection can not exceed 4_096 keys
    let keys: Result<KeyCollection, Error> = KeyCollection::new_ed25519(4_097);
    assert!(keys.is_err());
    // Key Collection should not hold 0 keys
    let keys: Result<KeyCollection, Error> = KeyCollection::new_ed25519(0);
    assert!(keys.is_err());
    // The number of keys created rounds up to the next power of two
    let keys: KeyCollection = KeyCollection::new_ed25519(2_049).unwrap();
    assert_eq!(keys.len(), 4_096);
    // The number of keys created rounds up to the next power of two
    let keys: KeyCollection = KeyCollection::new_ed25519(4_096).unwrap();
    assert_eq!(keys.len(), 4_096);
    // In case of overflow an error is returned
    let keys: Result<KeyCollection, Error> = KeyCollection::new_ed25519(usize::MAX);
    assert!(keys.is_err());
  }
}
