// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[doc(inline)]
pub use crypto::hashes::sha::Sha256;

#[doc(inline)]
pub use crypto::hashes::blake2b::Blake2b256;

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleTag;
use crate::crypto::Ed25519;

// Add support for using SHA-256 as a Merkle Key Collection digest algorithm.
impl MerkleDigest for Sha256 {
  const TAG: MerkleTag = MerkleTag::SHA256;
}

// Add support for using Blake2b-256 as a Merkle Key Collection digest algorithm.
impl MerkleDigest for Blake2b256 {
  const TAG: MerkleTag = MerkleTag::BLAKE2B_256;
}

impl<T: ?Sized> MerkleSignature for Ed25519<T> {
  const TAG: MerkleTag = MerkleTag::ED25519;
}
