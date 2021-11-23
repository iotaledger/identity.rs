// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleDigestTag;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleSignatureTag;
use crate::crypto::merkle_tree::Hash;
pub(crate) use self::errors::{InvalidMerkleDigestKeyTag, InvalidMerkleSignatureKeyTag, MerkleTagExtractionError}; 
/// Common utilities for working with Merkle Key Collection Signatures.
#[derive(Clone, Copy, Debug)]
pub struct MerkleKey;

impl MerkleKey {
  /// The `type` value of a Merkle Key Collection Verification Method.
  pub const TYPE_KEY: &'static str = "MerkleKeyCollection2021";

  /// The `type` value of a Merkle Key Collection Signature.
  pub const TYPE_SIG: &'static str = "MerkleKeySignature2021";

  /// Extracts the signature and digest algorithm tags from the public key value.
  pub fn extract_tags(data: &[u8]) -> Result<(MerkleSignatureTag, MerkleDigestTag),MerkleTagExtractionError> {
    let tag_s: MerkleSignatureTag = Self::signature_tag(data, 0)?;
    let tag_d: MerkleDigestTag = Self::digest_tag(data, 1)?;

    Ok((tag_s, tag_d))
  }

  /// Creates a DID Document public key value for the given Merkle tree `root`.
  pub fn encode_key<D, S>(root: &Hash<D>) -> Vec<u8>
  where
    D: MerkleDigest,
    S: MerkleSignature,
  {
    let mut output: Vec<u8> = Vec::with_capacity(2 + D::OUTPUT_SIZE);
    output.push(S::TAG.into());
    output.push(D::TAG.into());
    output.extend_from_slice(root.as_slice());
    output
  }

  fn digest_tag(data: &[u8], index: usize) -> Result<MerkleDigestTag, InvalidMerkleDigestKeyTag> {
    data
      .get(index)
      .copied()
      .map(MerkleDigestTag::new)
      .ok_or(InvalidMerkleDigestKeyTag(None))
  }

  fn signature_tag(data: &[u8], index: usize) -> Result<MerkleSignatureTag, InvalidMerkleSignatureKeyTag> {
    data
      .get(index)
      .copied()
      .map(MerkleSignatureTag::new)
      .ok_or(InvalidMerkleSignatureKeyTag(None))
  }
}


mod errors {
    use thiserror::Error as DeriveError;

    use crate::crypto::merkle_key::{MerkleDigestTag, MerkleSignatureTag}; 
    // Caused by attempting to parse an invalid Merkle Digest Key Collection tag.
    #[derive(Debug, DeriveError)]
    #[error("invalid Merkle digest key tag: {0:?}")]
    pub struct InvalidMerkleDigestKeyTag(pub Option<MerkleDigestTag>); 
  
  // Caused by attempting to parse an invalid Merkle Signature Key Collection tag.  #[error("Invalid Merkle Signature Key Tag: {0:?}")]
  #[derive(Debug, DeriveError)]
  #[error("Invalid Merkle Signature Key Tag: {0:?}")]
    pub struct InvalidMerkleSignatureKeyTag(pub Option<MerkleSignatureTag>);

    #[derive(Debug, DeriveError)]
    pub enum MerkleTagExtractionError{
      #[error("{0}")]
      InvalidMerkleDigestKeyTag(#[from] InvalidMerkleDigestKeyTag),
      #[error("{0}")]
      InvalidMerkleSignatureKeyTag(#[from] InvalidMerkleSignatureKeyTag),
    }
}
#[cfg(test)]
mod tests {
  use crate::crypto::merkle_key::Blake2b256;
  use crate::crypto::merkle_key::MerkleDigest;
  use crate::crypto::merkle_key::MerkleDigestTag;
  use crate::crypto::merkle_key::MerkleKey;
  use crate::crypto::merkle_key::MerkleSignature;
  use crate::crypto::merkle_key::MerkleSignatureTag;
  use crate::crypto::merkle_key::Sha256;
  use crate::crypto::merkle_tree::Hash;
  use crate::crypto::Ed25519;

  fn assert_tag<D, S>()
  where
    D: MerkleDigest,
    S: MerkleSignature,
  {
    let mut digest: D = D::new();
    let root: Hash<D> = digest.hash_leaf(b"Merkle Key Collection");
    let data: Vec<u8> = MerkleKey::encode_key::<D, S>(&root);
    let tags: (MerkleSignatureTag, MerkleDigestTag) = MerkleKey::extract_tags(&data).unwrap();

    assert_eq!(tags.0, S::TAG);
    assert_eq!(tags.1, D::TAG);
  }

  #[test]
  fn test_blake2b_tag() {
    assert_tag::<Blake2b256, Ed25519>();
  }

  #[test]
  fn test_sha256_tag() {
    assert_tag::<Sha256, Ed25519>();
  }
}
