// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::merkle_tree::AsLeaf;
use crate::crypto::merkle_tree::DigestExt;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Node;
use core::fmt::Debug;
use core::fmt::Formatter;
use thiserror::Error as DeriveError;

use subtle::ConstantTimeEq;

const MAX_PROOF_NODES: usize = 12;

/// A Merkle tree inclusion proof that allows proving the existence of a
/// particular leaf in a Merkle tree.
pub struct Proof<D: DigestExt> {
  nodes: Box<[Node<D>]>,
}

impl<D: DigestExt> Proof<D> {
  /// Maximum number of nodes in the proof.
  /// This value is equal to log₂[`crate::crypto::KeyCollection::MAX_KEYS_ALLOWED`], respecting the constraint for the
  /// maximum number of keys allowed in a `KeyCollection`
  pub const MAX_NODES: usize = MAX_PROOF_NODES;

  /// Creates a new [`Proof`] from a boxed slice of nodes.
  ///
  /// # Errors
  /// Fails if the length of `nodes` is greater than [`Self::MAX_NODES`]
  pub fn new(nodes: Box<[Node<D>]>) -> Result<Self, ProofSizeError> {
    if nodes.len() > Self::MAX_NODES {
      return Err(ProofSizeError);
    }
    Ok(Self { nodes })
  }

  /// Returns the nodes as a slice.
  pub fn nodes(&self) -> &[Node<D>] {
    &self.nodes
  }

  /// Returns the index of underlying leaf node in the Merkle tree.
  pub fn index(&self) -> usize {
    self.nodes.iter().enumerate().fold(0, |acc, (depth, node)| match node {
      Node::L(_) => acc + 2_usize.pow(depth as u32),
      Node::R(_) => acc,
    })
  }

  /// Verifies the computed root of `self` with the given `root` hash.
  pub fn verify<T>(&self, root: &Hash<D>, target: T) -> bool
  where
    T: AsLeaf<D>,
  {
    self.verify_hash(root, target.hash(&mut D::new()))
  }

  /// Verifies the computed root of `self` with the given `root` hash and
  /// a pre-computed target `hash`.
  pub fn verify_hash(&self, root: &Hash<D>, hash: Hash<D>) -> bool {
    self.root(hash).ct_eq(root).into()
  }

  /// Computes the root hash from `target` using a default digest.
  pub fn root(&self, target: Hash<D>) -> Hash<D> {
    self.root_with(&mut D::new(), target)
  }

  /// Computes the root hash from `target` using the given `digest`.
  pub fn root_with(&self, digest: &mut D, target: Hash<D>) -> Hash<D> {
    self.nodes.iter().fold(target, |acc, item| item.hash_with(digest, &acc))
  }
}

impl<D: DigestExt> Clone for Proof<D>
where
  Node<D>: Clone,
{
  fn clone(&self) -> Self {
    Self {
      nodes: self.nodes.clone(),
    }
  }
}

impl<D: DigestExt> Debug for Proof<D> {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("Proof").field("nodes", &self.nodes).finish()
  }
}

#[derive(Debug, DeriveError)]
#[error("too many nodes in the Proof. The maximum number allowed is {}", MAX_PROOF_NODES)]
/// Caused by attempting to create a [`Proof`] with too many nodes.
pub struct ProofSizeError;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::crypto::KeyCollection;

  #[test]
  fn max_proof_nodes_characterisation() {
    assert_eq!(2_usize.pow(MAX_PROOF_NODES as u32), KeyCollection::MAX_KEYS_ALLOWED);
  }
}
