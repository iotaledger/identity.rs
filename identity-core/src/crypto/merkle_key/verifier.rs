// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;
use erased_serde::Serialize;
use std::borrow::Cow;

use crate::common::BitSet;
use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleKey;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleTag;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::SignatureName;
use crate::crypto::SignatureValue;
use crate::crypto::SignatureVerify;
use crate::error::Error;
use crate::error::Result;
use crate::utils::decode_b58;

/// An alias for a [`Verifier`] with a dynamic [`signature`][`SignatureVerify`] type.
pub type DynVerifier<'key, D> = Verifier<'key, Box<dyn __Target + 'static>, D>;

/// A signature verification helper for Merkle Key Collection Signatures.
#[derive(Clone)]
pub struct Verifier<'key, S, D>
where
  D: MerkleDigest,
{
  suite: S,
  merkle_key: Cow<'key, [u8]>,
  revocation: Option<BitSet>,
  marker: PhantomData<D>,
}

impl<'key, S, D> Verifier<'key, S, D>
where
  D: MerkleDigest,
{
  /// Creates a new [`Verifier`] from a slice of bytes.
  pub fn from_borrowed(public: &'key [u8], suite: S) -> Self {
    Self {
      suite,
      merkle_key: Cow::Borrowed(public),
      revocation: None,
      marker: PhantomData,
    }
  }
}

impl<S, D> Verifier<'_, S, D>
where
  D: MerkleDigest,
{
  /// Creates a new [`Verifier`] from a vector of bytes.
  pub fn from_owned(public: Vec<u8>, suite: S) -> Self {
    Self {
      suite,
      merkle_key: Cow::Owned(public),
      revocation: None,
      marker: PhantomData,
    }
  }

  /// Sets the revocation flags associated with the verification object.
  pub fn set_revocation(&mut self, value: BitSet) {
    self.revocation = Some(value);
  }

  /// Clears the revocation flags associated with the verification object.
  pub fn clear_revocation(&mut self) {
    self.revocation = None;
  }
}

impl<S, D> Verifier<'_, S, D>
where
  S: MerkleSignature,
  D: MerkleDigest,
{
  fn decompose_public_key(&self, digest: &D) -> Result<Hash<D>> {
    let (tag_s, tag_d): (MerkleTag, MerkleTag) = MerkleKey::extract_tags(&self.merkle_key)?;

    // Validate the signature algorithm tag
    if tag_s != self.suite.tag() {
      return Err(Error::InvalidMerkleKeyTag(Some(tag_d)));
    }

    // Validate the digest algorithm tag
    if tag_d != digest.tag() {
      return Err(Error::InvalidMerkleKeyTag(Some(tag_d)));
    }

    // Extract and return the Merkle root hash
    self
      .merkle_key
      .get(2..)
      .and_then(Hash::from_slice)
      .ok_or(Error::InvalidKeyFormat)
  }
}

impl<S, D> SignatureName for Verifier<'_, S, D>
where
  D: MerkleDigest,
{
  fn name(&self) -> String {
    MerkleKey::TYPE_SIG.to_string()
  }
}

impl<S, D> SignatureVerify for Verifier<'_, S, D>
where
  S: MerkleSignature + SignatureVerify,
  D: MerkleDigest,
{
  fn verify(&self, data: &dyn Serialize, signature: &SignatureValue, public: &[u8]) -> Result<()> {
    // Merkle Key Collection signatures store their public key values
    // alongside their proofs (in the signature value). The `public` value
    // **SHOULD NOT** be provided as it will be extracted and verified
    // according to the decoded proof and revocation set.
    if !public.is_empty() {
      return Err(Error::InvalidKeyFormat);
    }

    let mut digest: D = D::new();

    let (target, proof, signature): _ = expand_signature_value(signature)?;

    let merkle_root: Hash<D> = self.decompose_public_key(&digest)?;
    let merkle_proof: Proof<D> = Proof::decode(&proof).ok_or(Error::InvalidProofFormat)?;
    let target_hash: Hash<D> = digest.hash_leaf(&target);

    // Ensure the target hash of the user-provided public key is part
    // of the Merkle tree
    if !merkle_proof.verify(&merkle_root, target_hash) {
      return Err(Error::InvalidProofValue);
    }

    // If a set of revocation flags was provided, ensure the public key
    // was not revoked
    if let Some(revocation) = self.revocation.as_ref() {
      if revocation.contains(merkle_proof.index() as u32) {
        return Err(Error::InvalidProofValue);
      }
    }

    // Verify the signature with underlying signature algorithm
    self.suite.verify(data, &signature, &target)?;

    Ok(())
  }
}

fn expand_signature_value(signature: &SignatureValue) -> Result<(Vec<u8>, Vec<u8>, SignatureValue)> {
  let data: &str = signature.as_str();
  let mut parts: _ = data.split('.');

  // Split the signature data into `public-key/proof/signature`
  let public: &str = parts.next().ok_or(Error::InvalidProofFormat)?;
  let proof: &str = parts.next().ok_or(Error::InvalidProofFormat)?;
  let signature: &str = parts.next().ok_or(Error::InvalidProofFormat)?;

  // Extract bytes of the base58-encoded public key
  let public: Vec<u8> = decode_b58(public).map_err(|_| Error::InvalidProofFormat)?;

  // Extract bytes of the base58-encoded proof
  let proof: Vec<u8> = decode_b58(proof).map_err(|_| Error::InvalidProofFormat)?;

  // Format the signature value for the underlying signature implementation
  let signature: SignatureValue = SignatureValue::Signature(signature.to_string());

  Ok((public, proof, signature))
}

// =============================================================================
// =============================================================================

#[doc(hidden)]
pub trait __Target: SignatureVerify + MerkleSignature {}

#[doc(hidden)]
impl<T> __Target for T where T: SignatureVerify + MerkleSignature {}

#[doc(hidden)]
impl<'target> MerkleSignature for Box<dyn __Target + 'target> {
  fn tag(&self) -> MerkleTag {
    (**self).tag()
  }
}

#[doc(hidden)]
impl<'target> SignatureVerify for Box<dyn __Target + 'target> {
  fn verify(&self, data: &dyn Serialize, signature: &SignatureValue, public: &[u8]) -> Result<()> {
    (**self).verify(data, signature, public)
  }
}
