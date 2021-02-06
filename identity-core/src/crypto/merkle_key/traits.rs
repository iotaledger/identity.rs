// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::merkle_tree::DigestExt;

/// A common interface for signature algorithms supported by Merkle Key Signatures.
pub trait MerkleSignature {
  /// A unique tag identifying the signature algorithm.
  fn tag(&self) -> MerkleTag;
}

/// A common interface for digest algorithms supported by Merkle Key Signatures.
pub trait MerkleDigest: DigestExt + 'static {
  /// A unique tag identifying the digest algorithm.
  fn tag(&self) -> MerkleTag;
}

/// A tag identifying a Merkle Key Collection signature or digest algorithm.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MerkleTag(u8);

impl MerkleTag {
  /// A Merkle Key Collection tag specifying `Ed25519` as the signature algorithm.
  pub const ED25519: Self = Self::new(0x0);

  /// A Merkle Key Collection tag specifying `SHA-256` as the digest algorithm.
  pub const SHA256: Self = Self::new(0x0);

  /// Creates a new [`MerkleTag`] object.
  pub const fn new(tag: u8) -> Self {
    Self(tag)
  }
}

impl From<u8> for MerkleTag {
  fn from(other: u8) -> Self {
    Self(other)
  }
}

impl From<MerkleTag> for u8 {
  fn from(other: MerkleTag) -> Self {
    other.0
  }
}
