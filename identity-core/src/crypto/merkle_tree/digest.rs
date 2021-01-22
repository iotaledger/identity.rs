// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[doc(import)]
pub use digest::Digest;

use crate::crypto::merkle_tree::consts;
use crate::crypto::merkle_tree::Hash;

/// An extension of the [`Digest`] trait for Merkle tree construction.
pub trait DigestExt: Sized + Digest {
    /// Computes the [`struct@Hash`] of a Merkle tree leaf node.
    fn hash_leaf(&mut self, data: &[u8]) -> Hash<Self> {
        self.reset();
        self.update(consts::PREFIX_L);
        self.update(data);
        self.finalize_reset().into()
    }

    /// Computes the parent [`struct@Hash`] of two Merkle tree nodes.
    fn hash_branch(&mut self, lhs: &Hash<Self>, rhs: &Hash<Self>) -> Hash<Self> {
        self.reset();
        self.update(consts::PREFIX_B);
        self.update(lhs.as_ref());
        self.update(rhs.as_ref());
        self.finalize_reset().into()
    }
}

impl<D> DigestExt for D where D: Digest {}
