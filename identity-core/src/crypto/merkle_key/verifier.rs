// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;
use serde::Serialize;

use crate::common::BitSet;
use crate::crypto::merkle_key::Digest;
use crate::crypto::merkle_key::MerkleKey;
use crate::crypto::merkle_key::Signature;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::PublicKey;
use crate::crypto::SigName;
use crate::crypto::SigVerify;
use crate::crypto::SignatureData;
use crate::error::Error;
use crate::error::Result;
use crate::utils::decode_b58;
use crate::utils::jcs_sha256;

/// A signature verification helper for Merkle Key Collection Signatures.
#[derive(Clone, Copy, Debug)]
pub struct Verifier<'a, D, S>
where
  D: Digest,
{
  suite: S,
  public: &'a PublicKey,
  marker: PhantomData<D>,
}

impl<'a, D, S> Verifier<'a, D, S>
where
  D: Digest,
{
  /// Creates a new [`Verifier`].
  pub fn new(public: &'a PublicKey, suite: S) -> Self {
    Self {
      suite,
      public,
      marker: PhantomData,
    }
  }
}

impl<'a, D, S> Verifier<'a, D, S>
where
  D: Digest,
  S: Signature,
{
  /// Verifies the authenticity of `message` using `signature` and `public`.
  pub fn verify_signature<T>(
    &self,
    message: &T,
    signature: &SignatureData,
    public: &[u8],
    revocation: Option<BitSet>,
  ) -> Result<()>
  where
    T: Serialize,
  {
    let merkle_root: Hash<D> = decompose_public_key::<D, S>(public)?;
    let target_hash: Hash<D> = D::new().hash_leaf(self.public.as_ref());

    let (proof, signature): (Vec<u8>, Vec<u8>) = expand_signature_value(signature)?;
    let proof: Proof<D> = MerkleKey::decode_proof(&proof).ok_or(Error::InvalidProofFormat)?;

    // Ensure the target hash of the user-provided public key is part
    // of the Merkle tree
    if !proof.verify(&merkle_root, target_hash) {
      return Err(Error::InvalidProofFormat);
    }

    // If a set of revocation flags was provided, ensure the public key
    // was not revoked
    if let Some(revocation) = revocation {
      if revocation.contains(proof.index() as u32) {
        return Err(Error::InvalidProofFormat);
      }
    }

    // Hash the document and verify the signature with the user-provided key
    self
      .suite
      .verify(&jcs_sha256(message)?, &signature, self.public.as_ref())?;

    Ok(())
  }
}

impl<'a, D, S> SigName for Verifier<'a, D, S>
where
  D: Digest,
{
  fn name(&self) -> String {
    MerkleKey::SIGNATURE_NAME.to_string()
  }
}

impl<'a, D, S> SigVerify for Verifier<'a, D, S>
where
  D: Digest,
  S: Signature,
{
  fn verify<T>(&self, data: &T, signature: &SignatureData, public: &[u8]) -> Result<()>
  where
    T: Serialize,
  {
    self.verify_signature(data, signature, public, None)
  }
}

fn decompose_public_key<D, S>(data: &[u8]) -> Result<Hash<D>>
where
  D: Digest,
  S: Signature,
{
  // Extract and validate the signature algorithm tag
  if *data.get(0).ok_or(Error::InvalidProofFormat)? != S::TAG {
    return Err(Error::InvalidProofFormat);
  }

  // Extract and validate the digest algorithm tag
  if *data.get(1).ok_or(Error::InvalidProofFormat)? != D::TAG {
    return Err(Error::InvalidProofFormat);
  }

  // Extract and return the Merkle root hash
  data
    .get(2..)
    .and_then(Hash::from_slice)
    .ok_or(Error::InvalidProofFormat)
}

fn expand_signature_value(signature: &SignatureData) -> Result<(Vec<u8>, Vec<u8>)> {
  let data: &str = signature.as_str();

  // Split the signature data into `encode-proof/encoded-signature`
  let (proof, signature): (&str, &str) = data
    .find('.')
    .ok_or(Error::InvalidProofFormat)
    .map(|index| data.split_at(index))
    .map(|(this, that)| (this, that.trim_start_matches('.')))?;

  // Extract bytes of the base58-encoded proof
  let proof: Vec<u8> = decode_b58(proof).map_err(|_| Error::InvalidProofFormat)?;

  // Extract bytes of the base58-encoded signature
  let signature: Vec<u8> = decode_b58(signature).map_err(|_| Error::InvalidProofFormat)?;

  Ok((proof, signature))
}
