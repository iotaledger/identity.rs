use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use digest::Digest;
use subtle::ConstantTimeEq;

use crate::crypto::merkle_tree::Node;
use crate::crypto::merkle_tree::Hash;

pub struct Proof<D: Digest> {
  nodes: Box<[Node<D>]>,
}

impl<D: Digest> Proof<D> {
  pub fn new(nodes: Box<[Node<D>]>) -> Self {
    Self { nodes }
  }

  pub fn nodes(&self) -> &[Node<D>] {
    &self.nodes
  }

  pub fn verify(&self, root: &Hash<D>, hash: Hash<D>) -> bool {
    self.root(hash).ct_eq(root).into()
  }

  pub fn root(&self, other: Hash<D>) -> Hash<D> {
    self.root_with(&mut D::new(), other)
  }

  pub fn root_with(&self, digest: &mut D, other: Hash<D>) -> Hash<D> {
    self.nodes.iter().fold(other, |acc, item| item.hash_with(digest, &acc))
  }
}

impl<D: Digest> Debug for Proof<D> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    f.debug_struct("Proof")
      .field("nodes", &self.nodes)
      .finish()
  }
}
