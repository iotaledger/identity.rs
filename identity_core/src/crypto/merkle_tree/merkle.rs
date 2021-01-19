use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use core::marker::PhantomData;
use sha2::digest::Output;
use sha2::Digest;
use sha2::Sha256;

use crate::crypto::merkle_tree::math;
use crate::crypto::merkle_tree::tree;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Node;
use crate::crypto::merkle_tree::Proof;

/// A [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree) designed for
/// static data.
// # Overview
//
// The Merkle tree is implemented as a **perfect binary tree** where all
// interior nodes have two children and all leaves have the same depth.
//
// # Layout
//
// An example tree with 8 leaves [A..H]:
//
// 0-|                 0
//  -|                 |
// 1-|         1 ------------- 2
//  -|         |               |
// 2-|     3 ----- 4      5 ------ 6
//  -|     |       |      |        |
// 3-|   A - B   C - D   E - F   G - H
//
// The tree will have the following layout:
//
//   [0, 1, 2, 3, 4, 5, 6, A, B, C, D, E, F, G, H]
//
// Building the tree is straight-forward:
//
//   1. Allocate Vec:  [_, _, _, _, _, _, _, _, _, _, _, _, _, _, _]
//   2. Insert Hashes: [_, _, _, _, _, _, _, A, B, C, D, E, F, G, H]
//   3. Update (H=2):  [_, _, _, 3, 4, 5, 6, A, B, C, D, E, F, G, H]
//   4. Update (H=1):  [_, 1, 2, 3, 4, 5, 6, A, B, C, D, E, F, G, H]
//   5. Update (H=0):  [0, 1, 2, 3, 4, 5, 6, A, B, C, D, E, F, G, H]
//
// Computing the root hash:
//
//   H(H(H(A | B) | H(C | D)) | H(H(E | F) | H(G | H)))
pub struct MTree<D = Sha256>
where
    D: Digest,
{
    nodes: Box<[Hash<D>]>,
    marker: PhantomData<D>,
}

impl<D> MTree<D>
where
    D: Digest,
{
    /// Returns the number of leaf nodes in the tree.
    pub fn leaves(&self) -> usize {
        tree::leaves(self.nodes.len())
    }

    /// Returns the height of the tree.
    pub fn height(&self) -> usize {
        tree::height(tree::leaves(self.nodes.len()))
    }

    /// Returns the root hash of the tree.
    pub fn root(&self) -> &Hash<D> {
        &self.nodes[0]
    }

    /// Returns a slice of the leaf nodes in the tree.
    pub fn data(&self) -> &[Hash<D>] {
        &self.nodes[self.nodes.len() - self.leaves()..]
    }

    /// Returns a slice of nodes at the specified `height`.
    pub fn layer(&self, height: usize) -> &[Hash<D>] {
        let leaves: usize = 2_usize.pow(height as u32);
        let total: usize = tree::total(leaves);

        if total <= self.nodes.len() {
            &self.nodes[total - leaves..total]
        } else {
            &[]
        }
    }
}

impl<D> MTree<D>
where
    D: Digest,
    Output<D>: Copy,
{
    /// Creates a new [`MTree`] from a slice of pre-hashed data.
    pub fn from_leaves(leaves: &[Hash<D>]) -> Option<Self> {
        // This Merkle tree only supports pow2 sequences
        if !math::is_pow2(leaves.len()) {
            return None;
        }

        Some(Self {
            nodes: tree::compute_nodes(&mut D::new(), leaves),
            marker: PhantomData,
        })
    }

    pub fn proof(&self, local: usize) -> Option<Proof<D>> {
        let leaves: usize = self.leaves();

        assert!(leaves >= 2);

        if local >= leaves {
            return None;
        }

        let mut nodes: Vec<Node<D>> = Vec::new();
        let mut index: usize = tree::total(leaves) - leaves + local;

        while index > 0 {
            if index & 1 == 0 {
                nodes.push(Node::L(self.nodes[index - 1]));
            } else {
                nodes.push(Node::R(self.nodes[index + 1]));
            }

            index = (index - 1) >> 1;
        }

        Some(Proof::new(nodes.into_boxed_slice()))
    }

    pub fn verify(&self, proof: &Proof<D>, hash: Hash<D>) -> bool {
        proof.verify(self.root(), hash)
    }
}

impl<D> Debug for MTree<D>
where
    D: Digest,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut f = f.debug_struct("MTree");

        let total: usize = self.height();
        let count: usize = total.min(4);

        f.field("layer (root)", &self.layer(0));

        for index in 1..count {
            f.field(&format!("layer (#{})", index), &self.layer(index));
        }

        f.field("height", &total);
        f.field("leaves", &self.leaves());

        f.finish()
    }
}

#[cfg(test)]
mod tests {
    use digest::Digest;
    use sha2::Sha256;

    use crate::crypto::merkle_tree::DigestExt;
    use crate::crypto::merkle_tree::Hash;
    use crate::crypto::merkle_tree::MTree;
    use crate::crypto::merkle_tree::Proof;

    macro_rules! h {
        ($leaf:expr) => {
            Sha256::new().hash_data($leaf)
        };
        ($lhs:expr, $rhs:expr) => {
            Sha256::new().hash_branch(&$lhs, &$rhs)
        };
    }

    type Sha256Hash = Hash<Sha256>;
    type Sha256Proof = Proof<Sha256>;

    #[test]
    fn test_works() {
        let nodes: Vec<Vec<u8>> = (0..(1 << 7))
            .map(|byte: u8| byte as char)
            .map(String::from)
            .map(String::into_bytes)
            .collect();

        let mut digest: Sha256 = Sha256::new();

        let hashes: Vec<Sha256Hash> = nodes.iter().map(|node| digest.hash_data(node.as_ref())).collect();

        let tree: MTree = MTree::from_leaves(&hashes).unwrap();

        assert_eq!(tree.data(), &hashes[..]);
        assert_eq!(tree.leaves(), hashes.len());

        for (index, hash) in hashes.iter().enumerate() {
            let proof: Sha256Proof = tree.proof(index).unwrap();
            let root: Sha256Hash = proof.root(*hash);

            assert_eq!(tree.root(), &root);
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

        let data: [Sha256Hash; 8] = [A, B, C, D, E, F, G, H];
        let tree: MTree = MTree::from_leaves(&data).unwrap();

        assert_eq!(tree.root(), &ABCDEFGH);
        assert_eq!(tree.data(), &[A, B, C, D, E, F, G, H]);
        assert_eq!(tree.height(), 3);
        assert_eq!(tree.leaves(), 8);

        assert_eq!(tree.layer(0), &[ABCDEFGH]);
        assert_eq!(tree.layer(1), &[ABCD, EFGH]);
        assert_eq!(tree.layer(2), &[AB, CD, EF, GH]);
        assert_eq!(tree.layer(3), &[A, B, C, D, E, F, G, H]);
        assert_eq!(tree.layer(4), &[]);
    }
}
