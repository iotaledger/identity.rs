// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A tag identifying a Merkle Key Collection signature or digest algorithm.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MerkleTag(u8);

impl MerkleTag {
  /// A Merkle Key Collection tag specifying `Ed25519` as the signature algorithm.
  pub const ED25519: Self = Self::new(0x0);

  /// A Merkle Key Collection tag specifying `SHA-256` as the digest algorithm.
  pub const SHA256: Self = Self::new(0x0);

  /// A Merkle Key Collection tag specifying `Blake2b-256` as the digest algorithm.
  pub const BLAKE2B_256: Self = Self::new(0x1);

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
