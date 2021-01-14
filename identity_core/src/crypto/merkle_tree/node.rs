use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use digest::Digest;

use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::DigestExt;

pub enum Node<D: Digest> {
  L(Hash<D>),
  R(Hash<D>),
}

impl<D: Digest> Node<D> {
  pub fn get(&self) -> &Hash<D> {
    match self {
      Self::L(hash) => hash,
      Self::R(hash) => hash,
    }
  }

  pub fn hash(&self, other: &Hash<D>) -> Hash<D> {
    self.hash_with(&mut D::new(), other)
  }

  pub fn hash_with(&self, digest: &mut D, other: &Hash<D>) -> Hash<D> {
    match self {
      Self::L(hash) => digest.hash_branch(hash, other),
      Self::R(hash) => digest.hash_branch(other, hash),
    }
  }
}

impl<D: Digest> Debug for Node<D> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::L(hash) => f.write_fmt(format_args!("L({:?})", hash)),
      Self::R(hash) => f.write_fmt(format_args!("R({:?})", hash)),
    }
  }
}

#[cfg(test)]
mod tests {
  use sha2::Sha256;

  use crate::crypto::merkle_tree::Hash;
  use crate::crypto::merkle_tree::Node;
  use crate::crypto::merkle_tree::Digest;
  use crate::crypto::merkle_tree::DigestExt;

  #[test]
  fn test_hash() {
    let mut digest: Sha256 = Sha256::new();

    let h1: Hash<Sha256> = digest.hash_data(b"A");
    let h2: Hash<Sha256> = digest.hash_data(b"B");

    assert_eq!(Node::L(h1).hash(&h2), digest.hash_branch(&h1, &h2));
    assert_eq!(Node::R(h1).hash(&h2), digest.hash_branch(&h2, &h1));

    assert_eq!(Node::L(h2).hash(&h1), digest.hash_branch(&h2, &h1));
    assert_eq!(Node::R(h2).hash(&h1), digest.hash_branch(&h1, &h2));
  }
}
