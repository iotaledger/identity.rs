// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use erased_serde::Serialize;
use std::borrow::Cow;

use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleKey;
use crate::crypto::merkle_tree::Proof;
use crate::crypto::SignatureName;
use crate::crypto::SignatureSign;
use crate::crypto::SignatureValue;
use crate::error::Result;
use crate::utils::encode_b58;

/// An alias for a [`Signer`] with a dynamic [`signature`][`SignatureSign`] type.
pub type DynSigner<'proof, 'suite, D> = Signer<'proof, Box<dyn SignatureSign + 'suite>, D>;

/// A signature creation helper for Merkle Key Collection Signatures.
///
/// Users should use the [`SignatureSign`] trait to access this implementation.
#[derive(Clone, Debug)]
pub struct Signer<'proof, S, D>(Cow<'proof, Proof<D>>, S)
where
  D: MerkleDigest;

impl<'proof, S, D> Signer<'proof, S, D>
where
  D: MerkleDigest,
{
  /// Creates a new [`Signer`] from a borrowed [`proof`][`Proof`].
  pub fn from_borrowed(proof: &'proof Proof<D>, suite: S) -> Self {
    Self(Cow::Borrowed(proof), suite)
  }
}

impl<S, D> Signer<'static, S, D>
where
  D: MerkleDigest,
{
  /// Creates a new [`Signer`] from an owned [`proof`][`Proof`].
  pub fn from_owned(proof: Proof<D>, suite: S) -> Self {
    Self(Cow::Owned(proof), suite)
  }
}

impl<'proof, S, D> SignatureName for Signer<'proof, S, D>
where
  D: MerkleDigest,
{
  fn name(&self) -> String {
    MerkleKey::TYPE_SIG.to_string()
  }
}

impl<'proof, S, D> SignatureSign for Signer<'proof, S, D>
where
  S: SignatureSign,
  D: MerkleDigest,
{
  fn sign(&self, message: &dyn Serialize, secret: &[u8]) -> Result<SignatureValue> {
    let signature: SignatureValue = self.1.sign(message, secret)?;
    let signature: String = signature.into_string();
    let proof: String = encode_b58(&self.0.encode());

    Ok(SignatureValue::Signature(format!("{}.{}", proof, signature)))
  }
}
