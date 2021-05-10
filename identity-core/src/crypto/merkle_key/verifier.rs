// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;
use serde::Serialize;
use std::borrow::Cow;

use crate::common::BitSet;
use crate::convert::ToJson;
use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleDigestTag;
use crate::crypto::merkle_key::MerkleKey;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleSignatureTag;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::Named;
use crate::crypto::PublicKey;
use crate::crypto::SignatureValue;
use crate::crypto::Verifier;
use crate::crypto::Verify;
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
pub struct MerkleVerifier<D, S>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  marker_d: PhantomData<D>,
  marker_s: PhantomData<S>,
}

impl<D, S> Named for MerkleVerifier<D, S>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  const NAME: &'static str = MerkleKey::TYPE_SIG;
}

impl<'key, D, S> Verifier<VerificationKey<'key>> for MerkleVerifier<D, S>
where
  D: MerkleDigest,
  S: MerkleSignature + Verify<Public = [u8]>,
{
  fn verify<X>(data: &X, signature: &SignatureValue, public: &VerificationKey<'key>) -> Result<()>
  where
    X: Serialize,
  {
    let mut digest: D = D::new();

    let (target, proof, signature): _ = expand_signature_value(signature)?;

    let merkle_root: Hash<D> = decompose_public_key::<D, S>(public)?;
    let merkle_proof: Proof<D> = Proof::decode(&proof).ok_or(Error::InvalidProofFormat)?;
    let target_hash: Hash<D> = digest.hash_leaf(target.as_ref());

    // Ensure the target hash of the user-provided public key is part
    // of the Merkle tree
    if !merkle_proof.verify(&merkle_root, target_hash) {
      return Err(Error::InvalidProofValue("merkle key - bad proof"));
    }

    // If a set of revocation flags was provided, ensure the public key
    // was not revoked
    if let Some(revocation) = public.revocation {
      if revocation.contains(merkle_proof.index() as u32) {
        return Err(Error::InvalidProofValue("merkle key - revoked"));
      }
    }

    // Verify the signature with underlying signature algorithm
    S::verify(&data.to_jcs()?, &signature, target.as_ref())?;

    Ok(())
  }
}

// =============================================================================
// =============================================================================

fn decompose_public_key<D, S>(key: &VerificationKey<'_>) -> Result<Hash<D>>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  let (tag_s, tag_d): (MerkleSignatureTag, MerkleDigestTag) = MerkleKey::extract_tags(&key.merkle_key)?;

  // Validate the signature algorithm tag
  if tag_s != S::TAG {
    return Err(Error::InvalidMerkleSignatureKeyTag(Some(tag_s)));
  }

  // Validate the digest algorithm tag
  if tag_d != D::TAG {
    return Err(Error::InvalidMerkleDigestKeyTag(Some(tag_d)));
  }

  // Extract and return the Merkle root hash
  key
    .merkle_key
    .get(2..)
    .and_then(Hash::from_slice)
    .ok_or(Error::InvalidKeyFormat)
}

fn expand_signature_value(signature: &SignatureValue) -> Result<(PublicKey, Vec<u8>, Vec<u8>)> {
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

  // Decode the signature value for the underlying signature implementation
  let signature: Vec<u8> = decode_b58(signature)?;

  Ok((public, proof, signature))
}
