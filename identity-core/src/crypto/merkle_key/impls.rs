// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[doc(inline)]
pub use crypto::hashes::sha::Sha256;

#[doc(inline)]
pub use crypto::hashes::blake2b::Blake2b256;

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleTag;
use crate::crypto::JcsEd25519Signature2020;
use crate::crypto::KeyRef;
use crate::crypto::KeyType;

// Add support for using SHA-256 as a Merkle Key Collection digest algorithm.
impl MerkleDigest for Sha256 {
  fn tag(&self) -> MerkleTag {
    MerkleTag::SHA256
  }
}

// Add support for using Blake2b-256 as a Merkle Key Collection digest algorithm.
impl MerkleDigest for Blake2b256 {
  fn tag(&self) -> MerkleTag {
    MerkleTag::BLAKE2B_256
  }
}

// Add support for using Ed25519 as a Merkle Key Collection signature algorithm.
//
// Note that we use the `JcsEd25519Signature2020` type which implements the
// exact same signature algorithm as the Merkle Key Collection spec - other
// implementations may not be so fortunate.
impl MerkleSignature for JcsEd25519Signature2020 {
  fn tag(&self) -> MerkleTag {
    MerkleTag::ED25519
  }
}

// Add an implementation for `KeyType` because we know the signature type.
//
// TODO: May need to expand this if/when `KeyType` has values we don't want
//       to support with Merkle Key Collections
impl MerkleSignature for KeyType {
  fn tag(&self) -> MerkleTag {
    match self {
      Self::Ed25519 => MerkleTag::ED25519,
    }
  }
}

// Add an implementation for `KeyRef` - we just delegate to the key type.
impl<'key> MerkleSignature for KeyRef<'key> {
  fn tag(&self) -> MerkleTag {
    self.kty().tag()
  }
}
