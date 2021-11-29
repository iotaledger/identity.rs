// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use errors::MerkleDigestKeyTagError;
pub use errors::MerkleKeyTagExtractionError;
pub use errors::MerkleSignatureKeyTagError;
/// A tag identifying a Merkle Key Collection digest algorithm.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MerkleDigestTag(u8);

impl MerkleDigestTag {
  /// A Merkle Key Collection tag specifying `SHA-256` as the digest algorithm.
  pub const SHA256: Self = Self::new(0x0);

  /// A Merkle Key Collection tag specifying `Blake2b-256` as the digest algorithm.
  pub const BLAKE2B_256: Self = Self::new(0x1);

  /// Creates a new [`MerkleDigestTag`] object.
  pub const fn new(tag: u8) -> Self {
    Self(tag)
  }
}

impl From<u8> for MerkleDigestTag {
  fn from(other: u8) -> Self {
    Self(other)
  }
}

impl From<MerkleDigestTag> for u8 {
  fn from(other: MerkleDigestTag) -> Self {
    other.0
  }
}

/// A tag identifying a Merkle Key Collection signature algorithm.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MerkleSignatureTag(u8);

impl MerkleSignatureTag {
  /// A Merkle Key Collection tag specifying `Ed25519` as the signature algorithm.
  pub const ED25519: Self = Self::new(0x0);

  /// Creates a new [`MerkleSignatureTag`]   object.
  pub const fn new(tag: u8) -> Self {
    Self(tag)
  }
}

impl From<u8> for MerkleSignatureTag {
  fn from(other: u8) -> Self {
    Self(other)
  }
}

impl From<MerkleSignatureTag> for u8 {
  fn from(other: MerkleSignatureTag) -> Self {
    other.0
  }
}

mod errors {
  use thiserror::Error as DeriveError;

  use crate::crypto::merkle_key::MerkleDigestTag;
  use crate::crypto::merkle_key::MerkleSignatureTag;
  /// Caused by attempting to parse an invalid Merkle Digest Key Collection tag.
  #[derive(Debug, DeriveError)]
  #[error("invalid Merkle digest key tag: {0:?}")]
  pub struct MerkleDigestKeyTagError(pub Option<MerkleDigestTag>);

  /// Caused by attempting to parse an invalid Merkle Signature Key Collection tag.  d
  #[derive(Debug, DeriveError)]
  #[error("Invalid Merkle Signature Key Tag: {0:?}")]
  pub struct MerkleSignatureKeyTagError(pub Option<MerkleSignatureTag>);

  #[derive(Debug, DeriveError)]
  /// Caused by an attempt to parse an invalid Merkle Digest or Signature key tag
  pub enum MerkleKeyTagExtractionError {
    /// See [`MerkleDigestKeyTagError`]
    #[error("{0}")]
    InvalidMerkleDigestKeyTag(#[from] MerkleDigestKeyTagError),
    /// See [`MerkleSignatureKeyTagError`]
    #[error("{0}")]
    InvalidMerkleSignatureKeyTag(#[from] MerkleSignatureKeyTagError),
  }
}
