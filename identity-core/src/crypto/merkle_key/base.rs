// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleTag;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::JcsEd25519Signature2020 as Ed25519;
use crate::error::Error;
use crate::error::Result;

/// Common utilities for working with Merkle Key Collection Signatures.
#[derive(Clone, Copy, Debug)]
pub struct MerkleKey;

impl MerkleKey {
  /// The `type` value of a Merkle Key Collection Verification Method.
  pub const TYPE_KEY: &'static str = "MerkleKeyCollection2021";

  /// The `type` value of a Merkle Key Collection Signature.
  pub const TYPE_SIG: &'static str = "MerkleKeySignature2021";

  /// Extracts the signature and digest algorithm tags from the public key value.
  pub fn extract_tags(data: &[u8]) -> Result<(MerkleTag, MerkleTag)> {
    let tag_s: MerkleTag = Self::__tag(data, 0)?;
    let tag_d: MerkleTag = Self::__tag(data, 1)?;

    Ok((tag_s, tag_d))
  }

  /// Creates a DID Document public key value for the given Merkle tree `root`.
  pub fn encode_key<S, D>(suite: &S, root: &Hash<D>) -> Vec<u8>
  where
    S: MerkleSignature,
    D: MerkleDigest,
  {
    let mut output: Vec<u8> = Vec::with_capacity(2 + D::OUTPUT_SIZE);
    output.push(suite.tag().into());
    output.push(D::new().tag().into());
    output.extend_from_slice(root.as_slice());
    output
  }

  /// Creates a DID Document public key value for the given Merkle tree `root`
  /// with `Ed25519` as the signature algorithm.
  pub fn encode_ed25519_key<D>(root: &Hash<D>) -> Vec<u8>
  where
    D: MerkleDigest,
  {
    Self::encode_key(&Ed25519, root)
  }

  fn __tag(data: &[u8], index: usize) -> Result<MerkleTag> {
    data
      .get(index)
      .copied()
      .map(MerkleTag::new)
      .ok_or(Error::InvalidKeyFormat)
  }
}

#[cfg(test)]
mod tests {
  use sha2::Sha256;

  use crate::crypto::merkle_key::MerkleKey;
  use crate::crypto::merkle_key::MerkleTag;
  use crate::crypto::merkle_tree::Digest;
  use crate::crypto::merkle_tree::DigestExt;
  use crate::crypto::merkle_tree::Hash;

  #[test]
  fn test_tags() {
    let root: Hash<Sha256> = Sha256::new().hash_leaf(b"Merkle Key Collection");
    let data: Vec<u8> = MerkleKey::encode_ed25519_key::<Sha256>(&root);
    let tags: (MerkleTag, MerkleTag) = MerkleKey::extract_tags(&data).unwrap();

    assert_eq!(tags.0, MerkleTag::ED25519);
    assert_eq!(tags.1, MerkleTag::SHA256);
  }
}
