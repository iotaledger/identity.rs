// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use erased_serde::Serialize;
use std::borrow::Cow;

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleKey;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::PublicKey;
use crate::crypto::SignatureName;
use crate::crypto::SignatureSign;
use crate::crypto::SignatureValue;
use crate::error::Result;
use crate::utils::encode_b58;

/// An alias for a [`Signer`] with a dynamic [`signature`][`SignatureSign`] type.
pub type DynSigner<'proof, 'key, D> = Signer<'proof, 'key, Box<dyn SignatureSign + 'static>, D>;

/// A signature creation helper for Merkle Key Collection Signatures.
///
/// Users should use the [`SignatureSign`] trait to access this implementation.
#[derive(Clone, Debug)]
pub struct Signer<'proof, 'key, S, D>
where
  D: MerkleDigest,
{
  proof: Cow<'proof, Proof<D>>,
  public: &'key PublicKey,
  suite: S,
}

impl<'proof, 'key, S, D> Signer<'proof, 'key, S, D>
where
  D: MerkleDigest,
{
  /// Creates a new Merkle Key Collection [`Signer`] from a borrowed
  /// [`proof`][`Proof`].
  pub fn from_borrowed(suite: S, public: &'key PublicKey, proof: &'proof Proof<D>) -> Self {
    Self {
      proof: Cow::Borrowed(proof),
      public,
      suite,
    }
  }
}

impl<'key, S, D> Signer<'static, 'key, S, D>
where
  D: MerkleDigest,
{
  /// Creates a new Merkle Key Collection [`Signer`] from an owned
  /// [`proof`][`Proof`].
  pub fn from_owned(suite: S, public: &'key PublicKey, proof: Proof<D>) -> Self {
    Self {
      proof: Cow::Owned(proof),
      public,
      suite,
    }
  }
}

impl<'proof, 'key, S, D> SignatureName for Signer<'proof, 'key, S, D>
where
  D: MerkleDigest,
{
  fn name(&self) -> String {
    MerkleKey::TYPE_SIG.to_string()
  }
}

impl<'proof, 'key, S, D> SignatureSign for Signer<'proof, 'key, S, D>
where
  S: SignatureSign,
  D: MerkleDigest,
{
  fn sign(&self, message: &dyn Serialize, secret: &[u8]) -> Result<SignatureValue> {
    let signature: SignatureValue = self.suite.sign(message, secret)?;
    let signature: String = signature.into_string();
    let proof: String = encode_b58(&self.proof.encode());
    let public: String = encode_b58(self.public.as_ref());

    Ok(SignatureValue::Signature(format!("{}.{}.{}", public, proof, signature)))
  }
}
