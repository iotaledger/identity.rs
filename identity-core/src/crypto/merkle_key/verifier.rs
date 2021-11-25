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
use crate::crypto::signature::errors::InvalidProofValue;
use crate::crypto::Named;
use crate::crypto::PublicKey;
use crate::crypto::SignatureValue;
use crate::crypto::Verifier;
use crate::crypto::Verify;
use crate::utils;

use self::errors::MerkleVerificationError;

use super::base::InvalidMerkleDigestKeyTag;
use super::base::InvalidMerkleSignatureKeyTag;

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
  type AuthenticityError = MerkleVerificationError;
  type SignatureVerificationError = MerkleVerificationError;
  fn verify<X>(
    data: &X,
    signature: &SignatureValue,
    public: &VerificationKey<'key>,
  ) -> Result<(), Self::AuthenticityError>
  where
    X: Serialize,
  {
    let mut digest: D = D::new();

    let (target, proof, signature): _ = expand_signature_value(signature)?;

    let merkle_root: Hash<D> = decompose_public_key::<D, S>(public)?;
    let merkle_proof: Proof<D> = Proof::decode(&proof).ok_or(errors::InvalidProofFormat)?;
    let target_hash: Hash<D> = digest.hash_leaf(target.as_ref());

    // Ensure the target hash of the user-provided public key is part
    // of the Merkle tree
    if !merkle_proof.verify(&merkle_root, target_hash) {
      return Err(InvalidProofValue("merkle key - bad proof").into());
    }

    // If a set of revocation flags was provided, ensure the public key
    // was not revoked
    if let Some(revocation) = public.revocation {
      if revocation.contains(merkle_proof.index() as u32) {
        return Err(InvalidProofValue("merkle key - revoked").into());
      }
    }

    // Verify the signature with underlying signature algorithm
    S::verify(
      &data.to_jcs().map_err(|_| {
        MerkleVerificationError::from(errors::MerkleVerificationProcessingErrorCause::SerializationFailure)
      })?,
      &signature,
      target.as_ref(),
    )
    .map_err(|err| match err.try_into() {
      Ok(invalid_proof_value_err) => errors::MerkleVerificationError::from(invalid_proof_value_err),
      _ => errors::MerkleVerificationError::ProcessingFailed(
        errors::MerkleVerificationProcessingErrorCause::Other(
          "unable to verify the authenticity of the given data and signature",
        )
        .into(),
      ),
    })?;

    Ok(())
  }
}

// =============================================================================
// =============================================================================

fn decompose_public_key<D, S>(
  key: &VerificationKey<'_>,
) -> Result<Hash<D>, errors::MerkleVerificationProcessingErrorCause>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  let (tag_s, tag_d): (MerkleSignatureTag, MerkleDigestTag) = MerkleKey::extract_tags(&key.merkle_key)?;

  // Validate the signature algorithm tag
  if tag_s != S::TAG {
    return Err(InvalidMerkleSignatureKeyTag(Some(tag_s)).into());
  }

  // Validate the digest algorithm tag
  if tag_d != D::TAG {
    return Err(InvalidMerkleDigestKeyTag(Some(tag_d)).into());
  }

  // Extract and return the Merkle root hash
  key
    .merkle_key
    .get(2..)
    .and_then(Hash::from_slice)
    .ok_or(errors::MerkleVerificationProcessingErrorCause::InvalidKeyFormat.into())
}

fn expand_signature_value(
  signature: &SignatureValue,
) -> Result<(PublicKey, Vec<u8>, Vec<u8>), errors::MerkleVerificationProcessingErrorCause> {
  let data: &str = signature.as_str();
  let mut parts: _ = data.split('.');

  // Split the signature data into `public-key/proof/signature`
  let public: &str = parts.next().ok_or(errors::InvalidProofFormat)?;
  let proof: &str = parts.next().ok_or(errors::InvalidProofFormat)?;
  let signature: &str = parts.next().ok_or(errors::InvalidProofFormat)?;

  // Extract bytes of the base58-encoded public key
  let public: PublicKey = utils::decode_b58(public)
    .map_err(|_| errors::InvalidProofFormat)
    .map(Into::into)?;

  // Extract bytes of the base58-encoded proof
  let proof: Vec<u8> = utils::decode_b58(proof).map_err(|_| errors::InvalidProofFormat)?;

  // Decode the signature value for the underlying signature implementation
  let signature: Vec<u8> = utils::decode_b58(signature)
    .map_err(|_| errors::MerkleVerificationProcessingErrorCause::SignatureParsingFailure)?;

  Ok((public, proof, signature))
}

mod errors {
  use crate::crypto::merkle_key::base::InvalidMerkleDigestKeyTag;
  use crate::crypto::merkle_key::base::InvalidMerkleSignatureKeyTag;
  use crate::crypto::merkle_key::base::MerkleTagExtractionError;
  use crate::crypto::signature::errors::InvalidProofValue;
  use crate::crypto::signature::errors::MissingSignatureError;
  use thiserror::Error as DeriveError;
  // Verification can typically fail by either actually verifying that the proof value is incorrect, or it can fail
  // before it gets to checking the proof value by for instance failing to (de)serialize some data etc. Hence the
  // verification error has two variants, where the latter wraps a private type.
  #[derive(Debug, DeriveError)]
  /// Caused by a failure to verify a cryptographic signature
  pub enum MerkleVerificationError {
    /// The provided signature does not match the expected value
    #[error("verification failed - invalid proof value: {0}")]
    InvalidProofValue(#[from] InvalidProofValue),

    /// Processing of the proof material failed before the proof value could be checked
    #[error("verification failed - processing failed before the proof value could be checked: {0}")]
    ProcessingFailed(#[from] MerkleVerificationProcessingError),
  }

  impl TryFrom<MerkleVerificationError> for InvalidProofValue {
    type Error = &'static str;
    fn try_from(value: MerkleVerificationError) -> Result<Self, Self::Error> {
      match value {
        MerkleVerificationError::InvalidProofValue(err) => Ok(err),
        _ => Err("processing failed before the proof value could be checked"),
      }
    }
  }

  impl From<MerkleVerificationProcessingErrorCause> for MerkleVerificationError {
    fn from(err: MerkleVerificationProcessingErrorCause) -> Self {
      Self::ProcessingFailed(MerkleVerificationProcessingError::from(err))
    }
  }

  impl From<MissingSignatureError> for MerkleVerificationError {
    fn from(_: MissingSignatureError) -> Self {
      Self::ProcessingFailed(MerkleVerificationProcessingErrorCause::MissingSignature("").into())
    }
  }

  impl From<InvalidProofFormat> for MerkleVerificationError {
    fn from(err: InvalidProofFormat) -> Self {
      MerkleVerificationError::ProcessingFailed(MerkleVerificationProcessingErrorCause::from(err).into())
    }
  }

  impl From<InvalidMerkleDigestKeyTag> for MerkleVerificationError {
    fn from(err: InvalidMerkleDigestKeyTag) -> Self {
      MerkleVerificationError::ProcessingFailed(MerkleVerificationProcessingErrorCause::from(err).into())
    }
  }

  impl From<InvalidMerkleSignatureKeyTag> for MerkleVerificationError {
    fn from(err: InvalidMerkleSignatureKeyTag) -> Self {
      MerkleVerificationError::ProcessingFailed(MerkleVerificationProcessingErrorCause::from(err).into())
    }
  }

  impl From<MerkleTagExtractionError> for MerkleVerificationError {
    fn from(err: MerkleTagExtractionError) -> Self {
      Self::ProcessingFailed(MerkleVerificationProcessingErrorCause::from(err).into())
    }
  }

  impl From<MerkleTagExtractionError> for MerkleVerificationProcessingErrorCause {
    fn from(err: MerkleTagExtractionError) -> Self {
      match err {
        MerkleTagExtractionError::InvalidMerkleDigestKeyTag(invalid_merkle_digest_tag) => {
          Self::from(invalid_merkle_digest_tag)
        }
        MerkleTagExtractionError::InvalidMerkleSignatureKeyTag(invalid_merkle_signature_key_tag) => {
          Self::from(invalid_merkle_signature_key_tag)
        }
      }
    }
  }

  #[derive(Debug, DeriveError)]
  /// Indicates that something went wrong during a signature verification process before one could check the validity of
  /// the signature.
  #[error("{cause}")]
  pub struct MerkleVerificationProcessingError {
    #[from]
    cause: MerkleVerificationProcessingErrorCause,
  }

  #[derive(Debug, DeriveError)]
  pub(super) enum MerkleVerificationProcessingErrorCause {
    #[error("could not serialize data")]
    SerializationFailure,
    #[error("could not parse signature")]
    SignatureParsingFailure,
    #[error("invalid key format")]
    InvalidKeyFormat,
    #[error("{0}")]
    InvalidProofFormat(#[from] InvalidProofFormat),
    #[error("{0}")]
    InvalidMerkleDigestKeyTag(#[from] InvalidMerkleDigestKeyTag),
    #[error("{0}")]
    InvalidMerkleSignatureKeyTag(#[from] InvalidMerkleSignatureKeyTag),
    // Unable to find the required signature
    #[error("missing signature:: {0}")]
    MissingSignature(&'static str),
    #[error("{0}")]
    Other(&'static str),
  }

  // Caused by attempting to parse an invalid DID proof.
  #[derive(Debug, DeriveError)]
  #[error("invalid proof format")]
  pub(super) struct InvalidProofFormat;
}
