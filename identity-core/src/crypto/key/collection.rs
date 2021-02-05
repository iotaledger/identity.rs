// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::iter::FromIterator;
use core::ops::Index;
use core::ops::IndexMut;
use core::slice::SliceIndex;
use digest::Output;

use crate::crypto::merkle_tree::DigestExt;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::MTree;
use crate::crypto::KeyPair;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::error::Result;

/// A collection of cryptographic keys.
#[derive(Clone, Debug, Default)]
pub struct KeyCollection(Vec<KeyPair>);

impl KeyCollection {
  /// Creates a new empty [`KeyCollection`].
  pub fn new() -> Self {
    Self(Vec::new())
  }

  /// Creates a new [`KeyCollection`] with `ed25519` keys.
  pub fn new_ed25519(count: usize) -> Result<Self> {
    (0..count).map(|_| KeyPair::new_ed25519()).collect()
  }

  /// Returns the number of keys in the collection.
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Returns `true` if the collection contains no keys.
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Returns a reference to the public key at the specified `index`.
  pub fn public(&self, index: usize) -> Option<&PublicKey> {
    self.0.get(index).map(KeyPair::public)
  }

  /// Returns a reference to the secret key at the specified `index`.
  pub fn secret(&self, index: usize) -> Option<&SecretKey> {
    self.0.get(index).map(KeyPair::secret)
  }

  /// Returns an iterator over the key pairs in the collection.
  pub fn iter(&self) -> impl Iterator<Item = &KeyPair> {
    self.0.iter()
  }

  /// Returns an iterator over the public keys in the collection.
  pub fn iter_public(&self) -> impl Iterator<Item = &PublicKey> {
    self.0.iter().map(KeyPair::public)
  }

  /// Returns an iterator over the secret keys in the collection.
  pub fn iter_secret(&self) -> impl Iterator<Item = &SecretKey> {
    self.0.iter().map(KeyPair::secret)
  }

  /// Creates a new Merkle tree from the public keys in the collection.
  pub fn to_merkle_tree<D>(&self) -> Option<MTree<D>>
  where
    D: DigestExt,
    Output<D>: Copy,
  {
    let mut digest: D = D::new();

    let keys: Vec<Hash<D>> = self
      .iter_public()
      .map(AsRef::as_ref)
      .map(|public| digest.hash_leaf(public))
      .collect();

    MTree::from_leaves(&keys)
  }
}

impl<I> Index<I> for KeyCollection
where
  I: SliceIndex<[KeyPair]>,
{
  type Output = <I as SliceIndex<[KeyPair]>>::Output;

  fn index(&self, index: I) -> &Self::Output {
    self.0.index(index)
  }
}

impl<I> IndexMut<I> for KeyCollection
where
  I: SliceIndex<[KeyPair]>,
{
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl FromIterator<KeyPair> for KeyCollection {
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = KeyPair>,
  {
    Self(Vec::from_iter(iter))
  }
}
