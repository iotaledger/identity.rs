// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;
use serde::Serialize;
use std::borrow::Cow;

use crate::common::BitSet;
use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleKey;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleTag;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::PublicKey;
use crate::crypto::SignatureName;
use crate::crypto::SignatureValue;
use crate::crypto::SignatureVerify;
use crate::error::Error;
use crate::error::Result;
use crate::utils::decode_b58;

/// Key components used to verify a Merkle Key Collection signature.
#[derive(Clone)]
pub struct VerificationKey<'key> {
  merkle_key: Cow<'key, [u8]>,
  revocation: Option<&'key BitSet>,
}

impl<'key> VerificationKey<'key> {
  /// Creates a new [`VerificationKey`] instance.
  pub fn new(merkle_key: Cow<'key, [u8]>) -> Self {
    Self {
      merkle_key,
      revocation: None,
    }
  }

  /// Creates a new [`VerificationKey`] from a slice of bytes.
  pub fn from_borrowed(merkle_key: &'key [u8]) -> Self {
    Self::new(Cow::Borrowed(merkle_key))
  }

  /// Creates a new [`VerificationKey`] from a vector of bytes.
  pub fn from_owned(merkle_key: Vec<u8>) -> Self {
    Self::new(Cow::Owned(merkle_key))
  }

  /// Sets the revocation flags associated with the verification object.
  pub fn set_revocation(&mut self, value: &'key BitSet) {
    self.revocation.replace(value);
  }

  /// Clears the revocation flags associated with the verification object.
  pub fn clear_revocation(&mut self) {
    self.revocation = None;
  }
}

// =============================================================================
// =============================================================================

/// A signature verification helper for Merkle Key Collection Signatures.
#[derive(Clone)]
pub struct Verifier<'key, D, S>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  merkle_key: Cow<'key, [u8]>,
  revocation: Option<&'key BitSet>,
  marker_d: PhantomData<D>,
  marker_s: PhantomData<S>,
}

impl<D, S> Verifier<'_, D, S>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  fn decompose_public_key(&self) -> Result<Hash<D>> {
    let (tag_s, tag_d): (MerkleTag, MerkleTag) = MerkleKey::extract_tags(&self.merkle_key)?;

    // Validate the signature algorithm tag
    if tag_s != S::TAG {
      return Err(Error::InvalidMerkleKeyTag(Some(tag_d)));
    }

    // Validate the digest algorithm tag
    if tag_d != D::TAG {
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

impl<'key, D, S> SignatureName for Verifier<'key, D, S>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  const NAME: &'static str = MerkleKey::TYPE_SIG;
}

impl<'borrow, 'key: 'borrow, D, S> SignatureVerify<'key> for Verifier<'key, D, S>
where
  D: MerkleDigest,
  S: MerkleSignature + for<'scope> SignatureVerify<'scope, Public = PublicKey>,
{
  type Actual = Self;
  type Public = VerificationKey<'key>;

  fn create(key: &'borrow VerificationKey<'key>) -> Self::Actual {
    Self {
      merkle_key: match key.merkle_key {
        Cow::Borrowed(data) => Cow::Borrowed(data),
        Cow::Owned(ref data) => Cow::Owned(data.clone()),
      },
      revocation: key.revocation,
      marker_d: PhantomData,
      marker_s: PhantomData,
    }
  }

  fn verify<T>(&self, data: &T, signature: &SignatureValue) -> Result<()>
  where
    T: Serialize,
  {
    let mut digest: D = D::new();

    let (target, proof, signature): _ = expand_signature_value(signature)?;

    let merkle_root: Hash<D> = self.decompose_public_key()?;
    let merkle_proof: Proof<D> = Proof::decode(&proof).ok_or(Error::InvalidProofFormat)?;
    let target_hash: Hash<D> = digest.hash_leaf(target.as_ref());

    // Ensure the target hash of the user-provided public key is part
    // of the Merkle tree
    if !merkle_proof.verify(&merkle_root, target_hash) {
      return Err(Error::InvalidProofValue);
    }

    // If a set of revocation flags was provided, ensure the public key
    // was not revoked
    if let Some(revocation) = self.revocation {
      if revocation.contains(merkle_proof.index() as u32) {
        return Err(Error::InvalidProofValue);
      }
    }

    // Verify the signature with underlying signature algorithm
    S::create(&target).verify(data, &signature)?;

    Ok(())
  }
}

// =============================================================================
// =============================================================================

fn expand_signature_value(signature: &SignatureValue) -> Result<(PublicKey, Vec<u8>, SignatureValue)> {
  let data: &str = signature.as_str();
  let mut parts: _ = data.split('.');

  // Split the signature data into `public-key/proof/signature`
  let public: &str = parts.next().ok_or(Error::InvalidProofFormat)?;
  let proof: &str = parts.next().ok_or(Error::InvalidProofFormat)?;
  let signature: &str = parts.next().ok_or(Error::InvalidProofFormat)?;

  // Extract bytes of the base58-encoded public key
  let public: PublicKey = decode_b58(public)
    .map_err(|_| Error::InvalidProofFormat)
    .map(Into::into)?;

  // Extract bytes of the base58-encoded proof
  let proof: Vec<u8> = decode_b58(proof).map_err(|_| Error::InvalidProofFormat)?;

  // Format the signature value for the underlying signature implementation
  let signature: SignatureValue = SignatureValue::Signature(signature.to_string());

  Ok((public, proof, signature))
}
