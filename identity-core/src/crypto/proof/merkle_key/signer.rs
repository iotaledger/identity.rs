// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::crypto::merkle_tree::Proof;
use crate::crypto::MerkleKey;
use crate::crypto::MerkleKeyDigest;
use crate::crypto::MerkleKeySignature;
use crate::crypto::SetSignature;
use crate::crypto::SigName;
use crate::crypto::SigSign;
use crate::crypto::Signature;
use crate::crypto::SignatureData;
use crate::crypto::SignatureOptions;
use crate::error::Result;
use crate::utils::encode_b58;
use crate::utils::jcs_sha256;

/// A signature creation helper for Merkle Key Collection Signatures.
#[derive(Clone, Copy, Debug)]
pub struct MerkleKeySigner<'a, D, S>
where
  D: MerkleKeyDigest,
{
  suite: S,
  proof: &'a Proof<D>,
}

impl<'a, D, S> MerkleKeySigner<'a, D, S>
where
  D: MerkleKeyDigest,
{
  /// Creates a new [`MerkleKeySigner`].
  pub fn new(proof: &'a Proof<D>, suite: S) -> Self {
    Self { suite, proof }
  }
}

impl<'a, D, S> MerkleKeySigner<'a, D, S>
where
  D: MerkleKeyDigest,
  S: MerkleKeySignature,
{
  /// Signs the given `message` with `secret` and embeds the signature in `message`.
  pub fn sign<T, K>(&self, message: &mut T, options: SignatureOptions, secret: &K) -> Result<()>
  where
    T: Serialize + SetSignature,
    K: AsRef<[u8]> + ?Sized,
  {
    message.set_signature(Signature::new(self.name(), options));

    let value: SignatureData = self.sign_data(message, secret.as_ref())?;

    message.try_signature_mut()?.set_data(value);

    Ok(())
  }

  /// Signs the given `message` with `secret` and returns a digital signature.
  pub fn sign_data<T>(&self, message: &T, secret: &[u8]) -> Result<SignatureData>
  where
    T: Serialize,
  {
    let digest: _ = jcs_sha256(message)?;
    let signature: Vec<u8> = self.suite.sign(&digest, secret)?;

    let encoded_signature: String = encode_b58(&signature);
    let encoded_proof: String = encode_b58(&MerkleKey::encode_proof(&self.proof));

    Ok(SignatureData::Signature(format!(
      "{}.{}",
      encoded_proof, encoded_signature
    )))
  }
}

impl<'a, D, S> SigName for MerkleKeySigner<'a, D, S>
where
  D: MerkleKeyDigest,
{
  fn name(&self) -> String {
    MerkleKey::SIGNATURE_NAME.to_string()
  }
}

impl<'a, D, S> SigSign for MerkleKeySigner<'a, D, S>
where
  D: MerkleKeyDigest,
  S: MerkleKeySignature,
{
  fn sign<T>(&self, data: &T, secret: &[u8]) -> Result<SignatureData>
  where
    T: Serialize,
  {
    self.sign_data(data, secret)
  }
}
