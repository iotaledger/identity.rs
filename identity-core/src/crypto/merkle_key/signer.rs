// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;
use serde::Serialize;
use std::borrow::Cow;

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleKey;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::crypto::SignatureName;
use crate::crypto::SignatureSign;
use crate::crypto::SignatureValue;
use crate::error::Result;
use crate::utils::encode_b58;

/// Key components used to create a Merkle Key Collection signature.
#[derive(Clone)]
pub struct SigningKey<'key, D>
where
  D: MerkleDigest,
{
  public: &'key PublicKey,
  secret: &'key SecretKey,
  proof: Cow<'key, Proof<D>>,
}

impl<'key, D> SigningKey<'key, D>
where
  D: MerkleDigest,
{
  /// Creates a new [`SigningKey`] instance.
  pub fn new(public: &'key PublicKey, secret: &'key SecretKey, proof: Cow<'key, Proof<D>>) -> Self {
    Self { public, secret, proof }
  }

  /// Creates a new [`SigningKey`] from a borrowed [`proof`][`Proof`].
  pub fn from_borrowed(public: &'key PublicKey, secret: &'key SecretKey, proof: &'key Proof<D>) -> Self {
    Self::new(public, secret, Cow::Borrowed(proof))
  }

  /// Creates a new [`SigningKey`] from an owned [`proof`][`Proof`].
  pub fn from_owned(public: &'key PublicKey, secret: &'key SecretKey, proof: Proof<D>) -> Self {
    Self::new(public, secret, Cow::Owned(proof))
  }

  fn reborrow(&self) -> Self {
    Self {
      public: self.public,
      secret: self.secret,
      proof: match self.proof {
        Cow::Borrowed(data) => Cow::Borrowed(data),
        Cow::Owned(ref data) => Cow::Owned(data.clone()),
      },
    }
  }
}

// =============================================================================
// =============================================================================

/// A signature creation helper for Merkle Key Collection Signatures.
///
/// Users should use the [`SignatureSign`] trait to access this implementation.
// #[derive(Clone)]
pub struct Signer<'key, D, S>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  key: SigningKey<'key, D>,
  marker: PhantomData<S>,
}

impl<'key, D, S> Signer<'key, D, S>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  fn encode_signature(&self, signature: SignatureValue) -> SignatureValue {
    let signature: String = signature.into_string();
    let proof: String = encode_b58(&self.key.proof.encode());
    let public: String = encode_b58(self.key.public.as_ref());
    let format: String = format!("{}.{}.{}", public, proof, signature);

    // TODO: Store the signature/digest tags and check during verification

    SignatureValue::Signature(format)
  }
}

impl<'key, D, S> SignatureName for Signer<'key, D, S>
where
  D: MerkleDigest,
  S: MerkleSignature,
{
  const NAME: &'static str = MerkleKey::TYPE_SIG;
}

impl<'key, D, S> SignatureSign<'key> for Signer<'key, D, S>
where
  D: MerkleDigest,
  S: MerkleSignature + SignatureSign<'key, Secret = SecretKey>,
{
  type Actual = Self;
  type Secret = SigningKey<'key, D>;

  fn create(key: &'key Self::Secret) -> Self::Actual {
    Self {
      key: key.reborrow(),
      marker: PhantomData,
    }
  }

  fn sign<T>(&self, message: &T) -> Result<SignatureValue>
  where
    T: Serialize,
  {
    S::create(self.key.secret)
      .sign(message)
      .map(|output| self.encode_signature(output))
  }
}
