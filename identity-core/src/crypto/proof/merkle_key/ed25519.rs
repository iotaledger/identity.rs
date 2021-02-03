// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::ed25519_sign;
use crate::crypto::ed25519_verify;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::MerkleKeyDigest;
use crate::crypto::MerkleKeySignature;
use crate::crypto::MerkleKeySigner;
use crate::crypto::MerkleKeyVerifier;
use crate::crypto::PublicKey;
use crate::error::Result;

/// A [`MerkleKeySigner`] using `Ed25519` as the signature algorithm.
pub type MerkleKeySignerEd25519<'a, D> = MerkleKeySigner<'a, D, MerkleKeyEd25519>;

/// A [`MerkleKeyVerifier`] using `Ed25519` as the signature algorithm.
pub type MerkleKeyVerifierEd25519<'a, D> = MerkleKeyVerifier<'a, D, MerkleKeyEd25519>;

/// Ed25519 for Merkle Key Collection Signatures.
#[derive(Clone, Copy, Debug)]
pub struct MerkleKeyEd25519;

impl MerkleKeySignature for MerkleKeyEd25519 {
  const TAG: u8 = 0;

  fn sign(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
    ed25519_sign(message, secret)
  }

  fn verify(&self, message: &[u8], signature: &[u8], public: &[u8]) -> Result<()> {
    ed25519_verify(message, signature, public)
  }
}

impl<'a, D> MerkleKeySigner<'a, D, MerkleKeyEd25519>
where
  D: MerkleKeyDigest,
{
  /// Creates a new [`MerkleKeySigner`] with `Ed25519` as the signature algorithm.
  pub fn new_ed25519(proof: &'a Proof<D>) -> Self {
    Self::new(proof, MerkleKeyEd25519)
  }
}

impl<'a, D> MerkleKeyVerifier<'a, D, MerkleKeyEd25519>
where
  D: MerkleKeyDigest,
{
  /// Creates a new [`MerkleKeyVerifier`] with `Ed25519` as the signature algorithm.
  pub fn new_ed25519(public: &'a PublicKey) -> Self {
    Self::new(public, MerkleKeyEd25519)
  }
}
