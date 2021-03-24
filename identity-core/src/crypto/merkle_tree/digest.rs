// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[doc(inline)]
pub use crypto::hashes::Digest;
#[doc(inline)]
pub use crypto::hashes::Output;

use typenum::Unsigned;

use crate::crypto::merkle_tree::Hash;

/// Leaf domain separation prefix.
const PREFIX_LEAF: &[u8] = &[0x00];

/// Node domain separation prefix.
const PREFIX_NODE: &[u8] = &[0x01];

/// An extension of the [`Digest`] trait for Merkle tree construction.
pub trait DigestExt: Sized + Digest {
  /// The output size of the digest function.
  const OUTPUT_SIZE: usize;

  /// Computes the [`struct@Hash`] of a Merkle tree leaf node.
  fn hash_leaf(&mut self, data: &[u8]) -> Hash<Self> {
    self.reset();
    self.update(PREFIX_LEAF);
    self.update(data);
    self.finalize_reset().into()
  }

  /// Computes the parent [`struct@Hash`] of two Merkle tree nodes.
  fn hash_node(&mut self, lhs: &Hash<Self>, rhs: &Hash<Self>) -> Hash<Self> {
    self.reset();
    self.update(PREFIX_NODE);
    self.update(lhs.as_slice());
    self.update(rhs.as_slice());
    self.finalize_reset().into()
  }

  /// Computes the [`struct@Hash`] of an empty Merkle tree.
  fn hash_empty(&mut self) -> Hash<Self> {
    self.reset();
    self.update(&[]);
    self.finalize_reset().into()
  }
}

impl<D> DigestExt for D
where
  D: Digest,
{
  const OUTPUT_SIZE: usize = <D::OutputSize>::USIZE;
}
