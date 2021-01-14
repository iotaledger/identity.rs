use digest::Output;
use digest::Digest;

use crate::crypto::merkle_tree::log2c;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::DigestExt;

#[inline(always)]
pub fn __height(leaves: usize) -> usize {
  log2c(leaves)
}

#[inline(always)]
pub const fn __total(leaves: usize) -> usize {
  // 2l - 1
  (leaves << 1) - 1
}

#[inline(always)]
pub const fn __leaves(nodes: usize) -> usize {
  // l = (n + 1) / 2
  (nodes + 1) >> 1
}

#[inline(always)]
pub const fn __index_lhs(index: usize) -> usize {
  // 2i + 1
  (index << 1) + 1
}

#[inline(always)]
pub const fn __index_rhs(index: usize) -> usize {
  // 2i + 2
  (index << 1) + 2
}

pub fn __compute_nodes<D>(digest: &mut D, leaves: &[Hash<D>]) -> Box<[Hash<D>]>
where
  D: Digest,
  Output<D>: Copy,
{
  let count: usize = leaves.len();
  let total: usize = __total(count);
  let offset: usize = total - count;
  let height: usize = __height(count);

  assert_eq!(count, 1 << height);

  // Create a vec for the entire set of nodes
  let mut nodes: Vec<Hash<D>> = vec![Hash::default(); total];

  // Copy the initial hashes to the end of the vec
  nodes[offset..total].copy_from_slice(leaves);

  // Compute parent hashes in bottom-up order
  for index in 0..height {
    __compute(digest, &mut nodes, height - index);
  }

  nodes.into_boxed_slice()
}

pub fn __compute<D>(digest: &mut D, nodes: &mut Vec<Hash<D>>, index: usize)
where
  D: Digest,
{
  let update: usize = 1 << (index - 1);
  let offset: usize = update - 1;

  for index in 0..update {
    let local: usize = offset + index;

    assert!(nodes.len() > local);

    let lhs: &Hash<D> = &nodes[__index_lhs(local)];
    let rhs: &Hash<D> = &nodes[__index_rhs(local)];

    nodes[local] = digest.hash_branch(lhs, rhs);
  }
}
