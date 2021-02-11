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
pub type DynVerifier<'key, 'suite, D> = Verifier<'key, Box<dyn __Target + 'suite>, D>;

/// A signature verification helper for Merkle Key Collection Signatures.
#[derive(Clone)]
pub struct Verifier<'key, S, D: MerkleDigest>(Cow<'key, [u8]>, S, Option<BitSet>, PhantomData<D>);

impl<'key, S, D> Verifier<'key, S, D>
where
  D: MerkleDigest,
{
  /// Creates a new [`Verifier`] from a slice of bytes.
  pub fn from_borrowed(public: &'key [u8], suite: S) -> Self {
    Self(Cow::Borrowed(public), suite, None, PhantomData)
  }
}

impl<S, D> Verifier<'static, S, D>
where
  D: MerkleDigest,
{
  /// Creates a new [`Verifier`] from a vector of bytes.
  pub fn from_owned(public: Vec<u8>, suite: S) -> Self {
    Self(Cow::Owned(public), suite, None, PhantomData)
  }
}

impl<S, D> Verifier<'_, S, D>
where
  S: MerkleSignature,
  D: MerkleDigest,
{
  /// Sets the revocation flags associated with the verification object.
  pub fn set_revocation(&mut self, value: BitSet) {
    self.2 = Some(value);
  }

  /// Clears the revocation flags associated with the verification object.
  pub fn clear_revocation(&mut self) {
    self.2 = None;
  }

  fn decompose_public_key(&self, digest: &D) -> Result<Hash<D>> {
    let (tag_s, tag_d): (MerkleTag, MerkleTag) = MerkleKey::extract_tags(&self.0)?;

    // Validate the signature algorithm tag
    if tag_s != self.1.tag() {
      return Err(Error::InvalidMerkleKeyTag(Some(tag_d)));
    }

    // Validate the digest algorithm tag
    if tag_d != digest.tag() {
      return Err(Error::InvalidMerkleKeyTag(Some(tag_d)));
    }

    // Extract and return the Merkle root hash
    self
      .0
      .get(2..)
      .and_then(Hash::from_slice)
      .ok_or(Error::InvalidKeyFormat)
  }
}

impl<'key, S, D> SignatureName for Verifier<'key, S, D>
where
  D: MerkleDigest,
{
  fn name(&self) -> String {
    MerkleKey::TYPE_SIG.to_string()
  }
}

impl<'key, S, D> SignatureVerify for Verifier<'key, S, D>
where
  S: MerkleSignature + SignatureVerify,
  D: MerkleDigest,
{
  fn verify(&self, data: &dyn Serialize, signature: &SignatureValue, public: &[u8]) -> Result<()> {
    let mut digest: D = D::new();

    let merkle_root: Hash<D> = self.decompose_public_key(&digest)?;
    let target_hash: Hash<D> = digest.hash_leaf(public);

    let (proof, signature): (Vec<u8>, SignatureValue) = expand_signature_value(signature)?;
    let proof: Proof<D> = Proof::decode(&proof).ok_or(Error::InvalidProofFormat)?;

    // Ensure the target hash of the user-provided public key is part
    // of the Merkle tree
    if !proof.verify(&merkle_root, target_hash) {
      return Err(Error::InvalidProofValue);
    }

    // If a set of revocation flags was provided, ensure the public key
    // was not revoked
    if let Some(revocation) = self.2.as_ref() {
      if revocation.contains(proof.index() as u32) {
        return Err(Error::InvalidProofValue);
      }
    }

    // Verify the signature with the user-provided key
    self.1.verify(data, &signature, public)?;

    Ok(())
  }
}

fn expand_signature_value(signature: &SignatureValue) -> Result<(Vec<u8>, SignatureValue)> {
  let data: &str = signature.as_str();

  // Split the signature data into `encode-proof/encoded-signature`
  let (proof, signature): (&str, &str) = data
    .find('.')
    .ok_or(Error::InvalidProofFormat)
    .map(|index| data.split_at(index))
    .map(|(this, that)| (this, that.trim_start_matches('.')))?;

  // Extract bytes of the base58-encoded proof
  let proof: Vec<u8> = decode_b58(proof).map_err(|_| Error::InvalidProofFormat)?;

  Ok((proof, SignatureValue::Signature(signature.to_string())))
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
