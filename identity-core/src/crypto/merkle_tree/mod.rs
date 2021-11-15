// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and traits for [Merkle tree][WIKI] operations.
//!
//! [WIKI]: https://en.wikipedia.org/wiki/Merkle_tree

mod digest;
mod hash;
mod merkle;
mod node;
mod proof;
mod serde;
mod traits;

pub use self::digest::Digest;
pub use self::digest::DigestExt;
pub use self::digest::Output;
pub use self::hash::Hash;
pub use self::merkle::compute_merkle_proof;
pub use self::merkle::compute_merkle_root;
pub use self::node::Node;
pub use self::proof::Proof;
pub use self::traits::AsLeaf;
