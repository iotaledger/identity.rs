#[doc(import)]
pub use digest::Digest;

use crate::crypto::merkle_tree::consts;
use crate::crypto::merkle_tree::Hash;

/// An extension of the [`Digest`] trait for Merkle tree construction.
pub trait DigestExt: Sized + Digest {
    fn hash_data(&mut self, data: &[u8]) -> Hash<Self> {
        self.reset();
        self.update(data);
        self.finalize_reset().into()
    }

    fn hash_leaf(&mut self, data: &Hash<Self>) -> Hash<Self> {
        self.reset();
        self.update(consts::PREFIX_L);
        self.update(data.as_ref());
        self.finalize_reset().into()
    }

    fn hash_branch(&mut self, lhs: &Hash<Self>, rhs: &Hash<Self>) -> Hash<Self> {
        self.reset();
        self.update(consts::PREFIX_B);
        self.update(lhs.as_ref());
        self.update(rhs.as_ref());
        self.finalize_reset().into()
    }
}

impl<D> DigestExt for D where D: Digest {}
