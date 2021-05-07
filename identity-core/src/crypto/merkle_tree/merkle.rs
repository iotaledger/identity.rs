// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::merkle_tree::AsLeaf;
use crate::crypto::merkle_tree::DigestExt;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Node;
use crate::crypto::merkle_tree::Proof;

/// Compute the Merkle root hash for the given slice of `leaves`.
///
/// The values in `leaves` can be a pre-hashed slice of [`struct@Hash<D>`] or
/// any type that implements [`AsRef<[u8]>`][`AsRef`].
///
/// For types implementing [`AsRef<[u8]>`][`AsRef`], the values will be hashed
/// according to the [`Digest`][`DigestExt`] implementation, `D`.
pub fn compute_merkle_root<D, L>(leaves: &[L]) -> Hash<D>
where
  D: DigestExt,
  L: AsLeaf<D>,
{
  #[inline]
  fn __generate<D, L>(digest: &mut D, leaves: &[L]) -> Hash<D>
  where
    D: DigestExt,
    L: AsLeaf<D>,
  {
    match leaves {
      [] => digest.hash_empty(),
      [leaf] => leaf.hash(digest),
      leaves => {
        let (this, that): _ = __split_pow2(leaves);

        let lhs: Hash<D> = __generate(digest, this);
        let rhs: Hash<D> = __generate(digest, that);

        digest.hash_node(&lhs, &rhs)
      }
    }
  }

  __generate::<D, L>(&mut D::new(), leaves)
}

/// Generate a proof-of-inclusion for the leaf node at the specified `index`.
pub fn compute_merkle_proof<D, L>(leaves: &[L], index: usize) -> Option<Proof<D>>
where
  D: DigestExt,
  L: AsLeaf<D>,
{
  #[inline]
  fn __generate<D, L>(digest: &mut D, path: &mut Vec<Node<D>>, leaves: &[L], index: usize)
  where
    D: DigestExt,
    L: AsLeaf<D>,
  {
    if leaves.len() > 1 {
      let k: usize = __pow2(leaves.len() as u32 - 1);
      let (this, that): _ = leaves.split_at(k);

      if index < k {
        __generate::<D, L>(digest, path, this, index);
        path.push(Node::R(compute_merkle_root::<D, L>(&that)));
      } else {
        __generate::<D, L>(digest, path, that, index - k);
        path.push(Node::L(compute_merkle_root::<D, L>(&this)));
      }
    }
  }

  match (index, leaves.len()) {
    (_, 0) => None,
    (0, 1) => Some(Proof::new(Box::new([]))),
    (_, 1) => None,
    (index, length) => {
      if index >= length {
        return None;
      }

      // TODO: Support proofs for any number of leaves
      if !length.is_power_of_two() {
        return None;
      }

      let height: usize = __log2c(leaves.len() as u32) as usize;
      let mut path: Vec<Node<D>> = Vec::with_capacity(height);

      __generate(&mut D::new(), &mut path, leaves, index);

      Some(Proof::new(path.into_boxed_slice()))
    }
  }
}

#[inline]
fn __pow2(value: u32) -> usize {
  1 << __log2c(value)
}

#[inline]
fn __log2c(value: u32) -> u32 {
  32 - value.leading_zeros() - 1
}

#[inline]
fn __split_pow2<T>(slice: &[T]) -> (&[T], &[T]) {
  slice.split_at(__pow2(slice.len() as u32 - 1))
}

#[cfg(test)]
mod tests {
  use crypto::hashes::sha::Sha256;

  use crate::crypto::merkle_tree::compute_merkle_proof;
  use crate::crypto::merkle_tree::compute_merkle_root;
  use crate::crypto::merkle_tree::Digest;
  use crate::crypto::merkle_tree::DigestExt;
  use crate::crypto::merkle_tree::Hash;
  use crate::crypto::merkle_tree::Proof;

  macro_rules! h {
    ($leaf:expr) => {
      Sha256::new().hash_leaf($leaf)
    };
    ($lhs:expr, $rhs:expr) => {
      Sha256::new().hash_node(&$lhs, &$rhs)
    };
  }

  type Sha256Hash = Hash<Sha256>;
  type Sha256Proof = Proof<Sha256>;

  #[test]
  fn test_compute_proof_and_index() {
    for exp in 0..6 {
      let mut digest: Sha256 = Sha256::new();

      let nodes: Vec<[u8; 4]> = (0..(1 << exp)).map(u32::to_be_bytes).collect();
      let hashes: Vec<Sha256Hash> = nodes.iter().map(|node| digest.hash_leaf(node.as_ref())).collect();
      let root: Sha256Hash = compute_merkle_root(&hashes);

      for (index, hash) in hashes.iter().enumerate() {
        let proof: Sha256Proof = compute_merkle_proof(&hashes, index).unwrap();

        assert_eq!(proof.index(), index);
        assert_eq!(proof.root(*hash), root);
        assert!(proof.verify(&root, &nodes[index]));
        assert!(proof.verify_hash(&root, *hash));
      }

      assert!(compute_merkle_proof::<Sha256, _>(&hashes, hashes.len()).is_none());
    }
  }

  #[test]
  #[allow(non_snake_case)]
  fn test_root() {
    let A: Sha256Hash = h!(b"A");
    let B: Sha256Hash = h!(b"B");
    let C: Sha256Hash = h!(b"C");
    let D: Sha256Hash = h!(b"D");
    let E: Sha256Hash = h!(b"E");
    let F: Sha256Hash = h!(b"F");
    let G: Sha256Hash = h!(b"G");
    let H: Sha256Hash = h!(b"H");

    let AB: Sha256Hash = h!(A, B);
    let CD: Sha256Hash = h!(C, D);
    let EF: Sha256Hash = h!(E, F);
    let GH: Sha256Hash = h!(G, H);

    let ABCD: Sha256Hash = h!(AB, CD);
    let EFGH: Sha256Hash = h!(EF, GH);

    let ABCDEFGH: Sha256Hash = h!(ABCD, EFGH);

    assert_eq!(AB, compute_merkle_root(&[A, B]));
    assert_eq!(CD, compute_merkle_root(&[C, D]));
    assert_eq!(EF, compute_merkle_root(&[E, F]));
    assert_eq!(GH, compute_merkle_root(&[G, H]));

    assert_eq!(ABCD, compute_merkle_root(&[A, B, C, D]));
    assert_eq!(EFGH, compute_merkle_root(&[E, F, G, H]));

    assert_eq!(ABCDEFGH, compute_merkle_root(&[A, B, C, D, E, F, G, H]));
  }
}
