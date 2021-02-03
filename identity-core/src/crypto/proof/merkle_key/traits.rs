// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::BitSet;
use crate::common::Object;
use crate::convert::FromJson;
use crate::crypto::merkle_tree::DigestExt;
use crate::error::Result;

// =============================================================================
// =============================================================================

/// A common interface for digest algorithms supported by Merkle Key Signatures.
pub trait MerkleKeyDigest: DigestExt {
  /// A unique tag identifying the digest algorithm.
  const TAG: u8;
}

// =============================================================================
// =============================================================================

/// A common interface for signature algorithms supported by Merkle Key Signatures.
pub trait MerkleKeySignature {
  /// A unique tag identifying the signature algorithm.
  const TAG: u8;

  /// Signs the given `message` with `secret` and returns a digital signature.
  fn sign(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>>;

  /// Verifies the authenticity of `message` using `signature` and `public`.
  fn verify(&self, message: &[u8], signature: &[u8], public: &[u8]) -> Result<()>;
}

// =============================================================================
// =============================================================================

/// A helper-trait for dynamic sources of revocation flags.
pub trait MerkleKeyRevocation {
  /// Returns the revocation [`BitSet`] of the verification method, if any.
  fn revocation(&self) -> Result<Option<BitSet>>;
}

impl<'a, T> MerkleKeyRevocation for &'a T
where
  T: MerkleKeyRevocation,
{
  fn revocation(&self) -> Result<Option<BitSet>> {
    (**self).revocation()
  }
}

impl MerkleKeyRevocation for () {
  fn revocation(&self) -> Result<Option<BitSet>> {
    Ok(None)
  }
}

impl MerkleKeyRevocation for Object {
  fn revocation(&self) -> Result<Option<BitSet>> {
    self.get("revocation").cloned().map(BitSet::from_json_value).transpose()
  }
}
