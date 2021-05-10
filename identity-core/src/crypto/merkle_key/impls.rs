// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[doc(inline)]
pub use crypto::hashes::sha::Sha256;

#[doc(inline)]
pub use crypto::hashes::blake2b::Blake2b256;

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleDigestTag;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleSignatureTag;
use crate::crypto::Ed25519;

// Add support for using SHA-256 as a Merkle Key Collection digest algorithm.
impl MerkleDigest for Sha256 {
  const TAG: MerkleDigestTag = MerkleDigestTag::SHA256;
}

// Add support for using Blake2b-256 as a Merkle Key Collection digest algorithm.
impl MerkleDigest for Blake2b256 {
  const TAG: MerkleDigestTag = MerkleDigestTag::BLAKE2B_256;
}

// Add support for using Ed25519 as a Merkle Key Collection signature algorithm.
impl<T: ?Sized> MerkleSignature for Ed25519<T> {
  const TAG: MerkleSignatureTag = MerkleSignatureTag::ED25519;
}
