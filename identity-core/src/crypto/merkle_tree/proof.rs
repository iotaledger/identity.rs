use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use digest::Digest;
use subtle::ConstantTimeEq;

use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Node;

/// An inclusion proof that allows proving the existence of data in a
/// [`Merkle tree`](`struct@super::MTree`).
pub struct Proof<D: Digest> {
    nodes: Box<[Node<D>]>,
}

impl<D: Digest> Proof<D> {
    /// Creates a new [`Proof`] from a boxed slice of nodes.
    pub fn new(nodes: Box<[Node<D>]>) -> Self {
        Self { nodes }
    }

    /// Returns the nodes as a slice.
    pub fn nodes(&self) -> &[Node<D>] {
        &self.nodes
    }

    /// Verifies the computed root of `self` with the given `root` hash.
    pub fn verify(&self, root: &Hash<D>, hash: Hash<D>) -> bool {
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

impl<D: Digest> Debug for Proof<D> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Proof").field("nodes", &self.nodes).finish()
    }
}
