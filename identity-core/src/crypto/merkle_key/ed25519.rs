// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::ed25519_sign;
use crate::crypto::ed25519_verify;
use crate::crypto::merkle_key::Digest;
use crate::crypto::merkle_key::Signature;
use crate::crypto::merkle_key::Signer;
use crate::crypto::merkle_key::Verifier;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::PublicKey;
use crate::error::Result;

/// A [`Signer`] using `Ed25519` as the signature algorithm.
pub type SignerEd25519<'a, D> = Signer<'a, D, Ed25519>;

/// A [`Verifier`] using `Ed25519` as the signature algorithm.
pub type VerifierEd25519<'a, D> = Verifier<'a, D, Ed25519>;

/// Ed25519 for Merkle Key Collection Signatures.
#[derive(Clone, Copy, Debug)]
pub struct Ed25519;

impl Signature for Ed25519 {
  const TAG: u8 = 0;

  fn sign(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
    ed25519_sign(message, secret)
  }

  fn verify(&self, message: &[u8], signature: &[u8], public: &[u8]) -> Result<()> {
    ed25519_verify(message, signature, public)
  }
}

impl<'a, D> Signer<'a, D, Ed25519>
where
  D: Digest,
{
  /// Creates a new [`Signer`] with `Ed25519` as the signature algorithm.
  pub fn new_ed25519(proof: &'a Proof<D>) -> Self {
    Self::new(proof, Ed25519)
  }
}

impl<'a, D> Verifier<'a, D, Ed25519>
where
  D: Digest,
{
  /// Creates a new [`Verifier`] with `Ed25519` as the signature algorithm.
  pub fn new_ed25519(public: &'a PublicKey) -> Self {
    Self::new(public, Ed25519)
  }
}
