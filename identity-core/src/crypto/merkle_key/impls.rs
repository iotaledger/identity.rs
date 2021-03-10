// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

#[doc(inline)]
pub use crypto::hashes::sha::Sha256;

#[doc(inline)]
pub use crypto::hashes::blake2b::Blake2b256;

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleTag;
use crate::crypto::JcsEd25519Signer;
use crate::crypto::JcsEd25519Verifier;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::crypto::SignatureName;
use crate::crypto::SignatureSign;
use crate::crypto::SignatureValue;
use crate::crypto::SignatureVerify;
use crate::error::Error;
use crate::error::Result;

// Add support for using SHA-256 as a Merkle Key Collection digest algorithm.
impl MerkleDigest for Sha256 {
  const TAG: MerkleTag = MerkleTag::SHA256;
}

// Add support for using Blake2b-256 as a Merkle Key Collection digest algorithm.
impl MerkleDigest for Blake2b256 {
  const TAG: MerkleTag = MerkleTag::BLAKE2B_256;
}

/// An implementation of Ed25519 for Merkle Key Collections.
#[derive(Clone, Copy, Debug)]
pub struct Ed25519;

// Add support for using Ed25519 as a Merkle Key Collection signature algorithm.
//
// Note that we delegate to the `JcsEd25519Signature2020` suite which implements
// the exact same signature algorithm as the Merkle Key Collection spec - other
// implementations may not be so fortunate.
impl MerkleSignature for Ed25519 {
  const TAG: MerkleTag = MerkleTag::ED25519;
}

impl SignatureName for Ed25519 {
  const NAME: &'static str = "";
}

impl<'key> SignatureSign<'key> for Ed25519 {
  type Actual = JcsEd25519Signer<'key>;
  type Secret = SecretKey;

  fn create(key: &'key Self::Secret) -> Self::Actual {
    JcsEd25519Signer::create(key)
  }

  fn sign<T>(&self, _: &T) -> Result<SignatureValue>
  where
    T: Serialize,
  {
    Err(Error::DelegatedSignatureCall)
  }
}

impl<'key> SignatureVerify<'key> for Ed25519 {
  type Actual = JcsEd25519Verifier<'key>;
  type Public = PublicKey;

  fn create(key: &'key Self::Public) -> Self::Actual {
    JcsEd25519Verifier::create(key)
  }

  fn verify<T>(&self, _: &T, _: &SignatureValue) -> Result<()>
  where
    T: Serialize,
  {
    Err(Error::DelegatedSignatureCall)
  }
}
