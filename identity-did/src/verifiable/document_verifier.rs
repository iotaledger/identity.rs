// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use identity_core::common::BitSet;
use identity_core::common::Object;
use identity_core::crypto::merkle_key::Blake2b256;
use identity_core::crypto::merkle_key::MerkleDigest;
use identity_core::crypto::merkle_key::MerkleDigestTag;
use identity_core::crypto::merkle_key::MerkleKey;
use identity_core::crypto::merkle_key::MerkleSignature;
use identity_core::crypto::merkle_key::MerkleSignatureTag;
use identity_core::crypto::merkle_key::MerkleVerifier;
use identity_core::crypto::merkle_key::Sha256;
use identity_core::crypto::merkle_key::VerificationKey;
use identity_core::crypto::Ed25519;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::Verifier;
use identity_core::crypto::Verify;

use crate::document::CoreDocument;
use crate::verifiable::Revocation;
use crate::verification::MethodScope;
use crate::verification::MethodType;
use crate::verification::VerificationMethod;
use crate::Error;
use crate::Result;

// =============================================================================
// Document Verifier - Simplifying Digital Signature Verification Since 2021
// =============================================================================

pub struct DocumentVerifier<'base, T = Object, U = Object, V = Object> {
  document: &'base CoreDocument<T, U, V>,
}

impl<'base, T, U, V> DocumentVerifier<'base, T, U, V> {
  pub fn new(document: &'base CoreDocument<T, U, V>) -> Self {
    Self { document }
  }
}

impl<T, U, V> DocumentVerifier<'_, T, U, V>
where
  U: Revocation,
{
  /// Verifies the signature of the provided data.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the verification operation fails.
  pub fn verify<X>(&self, that: &X) -> Result<()>
  where
    X: Serialize + TrySignature,
  {
    let signature: &Signature = that.try_signature()?;
    let method: &VerificationMethod<U> = self.document.try_resolve_method(signature)?;

    Self::do_verify(method, that)
  }

  /// Verifies the signature of the provided data and that it was signed with a verification method
  /// with a verification relationship specified by `scope`.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the verification operation fails.
  pub fn verify_with_scope<X>(&self, that: &X, scope: MethodScope) -> Result<()>
  where
    X: Serialize + TrySignature,
  {
    let signature: &Signature = that.try_signature()?;
    let method: &VerificationMethod<U> = self.document.try_resolve_method_with_scope(signature, scope)?;

    Self::do_verify(method, that)
  }

  /// Verifies the signature of the provided data.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the verification operation fails.
  pub fn do_verify<X>(method: &VerificationMethod<U>, that: &X) -> Result<()>
  where
    X: Serialize + TrySignature,
  {
    let data: Vec<u8> = method.key_data().try_decode()?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        JcsEd25519::<Ed25519>::verify_signature(that, &data)?;
      }
      MethodType::MerkleKeyCollection2021 => match MerkleKey::extract_tags(&data)? {
        (MerkleSignatureTag::ED25519, MerkleDigestTag::SHA256) => {
          merkle_key_verify::<X, Sha256, Ed25519, U>(that, method, &data)?;
        }
        (MerkleSignatureTag::ED25519, MerkleDigestTag::BLAKE2B_256) => {
          merkle_key_verify::<X, Blake2b256, Ed25519, U>(that, method, &data)?;
        }
        (_, _) => {
          return Err(Error::InvalidMethodType);
        }
      },
    }

    Ok(())
  }
}

fn merkle_key_verify<X, D, S, U>(that: &X, method: &VerificationMethod<U>, data: &[u8]) -> Result<()>
where
  X: Serialize + TrySignature,
  D: MerkleDigest,
  S: MerkleSignature + Verify<Public = [u8]>,
  U: Revocation,
{
  let revocation: Option<BitSet> = method.revocation()?;
  let mut vkey: VerificationKey<'_> = VerificationKey::from_borrowed(data);

  if let Some(revocation) = revocation.as_ref() {
    vkey.set_revocation(revocation);
  }

  MerkleVerifier::<D, S>::verify_signature(that, &vkey)?;

  Ok(())
}
