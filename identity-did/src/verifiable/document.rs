// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use identity_core::common::BitSet;
use identity_core::crypto::MerkleKey;
use identity_core::crypto::MerkleKeyDigest;
use identity_core::crypto::MerkleKeyRevocation;
use identity_core::crypto::MerkleKeySignature;
use identity_core::crypto::MerkleKeyVerifier;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;
use identity_core::error::Error as CoreError;
use serde::Serialize;

use crate::document::Document;
use crate::error::Error;
use crate::error::Result;
use crate::verifiable::Properties;
use crate::verification::Method;
use crate::verification::MethodQuery;
use crate::verification::MethodType;
use crate::verification::MethodWrap;

// =============================================================================
// Generic Crypto Extensions
// =============================================================================

impl<T, U, V> Document<T, U, V> {
  pub fn into_verifiable(self) -> Document<Properties<T>, U, V> {
    self.map(Properties::new)
  }

  pub fn into_verifiable2(self, proof: Signature) -> Document<Properties<T>, U, V> {
    self.map(|old| Properties::with_proof(old, proof))
  }
}

impl<T, U, V> Document<Properties<T>, U, V> {
  pub fn proof(&self) -> Option<&Signature> {
    self.properties().proof()
  }

  pub fn proof_mut(&mut self) -> Option<&mut Signature> {
    self.properties_mut().proof_mut()
  }

  pub fn set_proof(&mut self, signature: Signature) {
    self.properties_mut().set_proof(signature);
  }
}

impl<T, U, V> TrySignature for Document<Properties<T>, U, V> {
  fn signature(&self) -> Option<&Signature> {
    self.proof()
  }
}

impl<T, U, V> TrySignatureMut for Document<Properties<T>, U, V> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof_mut()
  }
}

impl<T, U, V> SetSignature for Document<Properties<T>, U, V> {
  fn set_signature(&mut self, signature: Signature) {
    self.set_proof(signature)
  }
}

// =============================================================================
// Merkle Key Collection Crypto Extensions
// =============================================================================

impl<T> MerkleKeyRevocation for Method<T>
where
  T: MerkleKeyRevocation,
{
  fn revocation(&self) -> Result<Option<BitSet>, CoreError> {
    self.properties().revocation()
  }
}

impl<T, U, V> Document<T, U, V>
where
  U: MerkleKeyRevocation,
{
  pub fn verify_merkle_key<M, D, S>(&self, message: &M, verifier: MerkleKeyVerifier<'_, D, S>) -> Result<()>
  where
    M: Serialize + TrySignature,
    D: MerkleKeyDigest,
    S: MerkleKeySignature,
  {
    let signature: &Signature = message.try_signature()?;

    if signature.type_() != MerkleKey::SIGNATURE_NAME {
      return Err(Error::UnknownSignatureType);
    }

    let query: MethodQuery<'_> = signature.try_into()?;
    let method: MethodWrap<'_, U> = self.try_resolve(query)?;

    if method.key_type() != MethodType::MerkleKeyCollection2021 {
      return Err(Error::UnknownMethodType);
    }

    let decoded: Vec<u8> = method.key_data().try_decode()?;
    let revocation: Option<BitSet> = method.revocation()?;

    signature
      .verifiable(move |data| verifier.verify_signature(message, data, &decoded, revocation))
      .map_err(Into::into)
  }
}
