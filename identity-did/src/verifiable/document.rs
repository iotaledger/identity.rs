// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::any::Any;
use identity_core::common::BitSet;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::crypto::merkle_key::Blake2b256;
use identity_core::crypto::merkle_key::MerkleDigest;
use identity_core::crypto::merkle_key::MerkleDigestTag;
use identity_core::crypto::merkle_key::MerkleKey;
use identity_core::crypto::merkle_key::MerkleSignature;
use identity_core::crypto::merkle_key::MerkleSignatureTag;
use identity_core::crypto::merkle_key::MerkleSigner;
use identity_core::crypto::merkle_key::MerkleVerifier;
use identity_core::crypto::merkle_key::Sha256;
use identity_core::crypto::merkle_key::SigningKey;
use identity_core::crypto::merkle_key::VerificationKey;
use identity_core::crypto::merkle_tree::Proof;
use identity_core::crypto::Ed25519;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Sign;
use identity_core::crypto::Signature;
use identity_core::crypto::Signer;
use identity_core::crypto::TrySignature;
use identity_core::crypto::Verifier;
use identity_core::crypto::Verify;
use identity_core::error::Error as CoreError;
use serde::Serialize;

use crate::document::CoreDocument;
use crate::error::Error;
use crate::error::Result;
use crate::verifiable::Revocation;
use crate::verification::MethodQuery;
use crate::verification::MethodScope;
use crate::verification::MethodType;
use crate::verification::MethodUriType;
use crate::verification::TryMethod;
use crate::verification::VerificationMethod;

impl<T, U, V> TryMethod for CoreDocument<T, U, V> {
  const TYPE: MethodUriType = MethodUriType::Relative;
}

// =============================================================================
// Signature Extensions
// =============================================================================

impl<T, U, V> CoreDocument<T, U, V> {
  /// Creates a new [`DocumentSigner`] that can be used to create digital
  /// signatures from verification methods in this DID Document.
  pub fn signer<'base>(&'base self, private: &'base PrivateKey) -> DocumentSigner<'base, '_, '_, T, U, V> {
    DocumentSigner::new(self, private)
  }

  /// Creates a new [`DocumentVerifier`] that can be used to verify signatures
  /// created with this DID Document.
  pub fn verifier(&self) -> DocumentVerifier<'_, T, U, V> {
    DocumentVerifier::new(self)
  }
}

// =============================================================================
// Document Signer - Simplifying Digital Signature Creation Since 2021
// =============================================================================

pub struct DocumentSigner<'base, 'query, 'proof, T = Object, U = Object, V = Object> {
  document: &'base CoreDocument<T, U, V>,
  private: &'base PrivateKey,
  method: Option<MethodQuery<'query>>,
  merkle_key: Option<(&'proof PublicKey, &'proof dyn Any)>,
  created: Option<Timestamp>,
  expires: Option<Timestamp>,
  challenge: Option<String>,
  domain: Option<String>,
  purpose: Option<String>,
}

impl<'base, T, U, V> DocumentSigner<'base, '_, '_, T, U, V> {
  pub fn new(document: &'base CoreDocument<T, U, V>, private: &'base PrivateKey) -> Self {
    Self {
      document,
      private,
      method: None,
      merkle_key: None,
      created: None,
      expires: None,
      challenge: None,
      domain: None,
      purpose: None,
    }
  }

  /// Sets the [`Signature::created`] field.
  pub fn created(mut self, created: Option<Timestamp>) -> Self {
    self.created = created;
    self
  }

  /// Sets the [`Signature::expires`] field. The signature will fail validation after the specified
  /// datetime.
  pub fn expires(mut self, expires: Option<Timestamp>) -> Self {
    self.expires = expires;
    self
  }

  /// Sets the [`Signature::challenge`] field.
  pub fn challenge(mut self, challenge: Option<String>) -> Self {
    self.challenge = challenge;
    self
  }

  /// Sets the [`Signature::domain`] field.
  pub fn domain(mut self, domain: Option<String>) -> Self {
    self.domain = domain;
    self
  }

  /// Sets the [`Signature::purpose`] field.
  pub fn purpose(mut self, purpose: Option<String>) -> Self {
    self.purpose = purpose;
    self
  }
}

impl<'base, 'query, T, U, V> DocumentSigner<'base, 'query, '_, T, U, V> {
  pub fn method<Q>(mut self, value: Q) -> Self
  where
    Q: Into<MethodQuery<'query>>,
  {
    self.method = Some(value.into());
    self
  }
}

impl<'proof, T, U, V> DocumentSigner<'_, '_, 'proof, T, U, V> {
  pub fn merkle_key<D>(mut self, proof: (&'proof PublicKey, &'proof Proof<D>)) -> Self
  where
    D: MerkleDigest,
  {
    self.merkle_key = Some((proof.0, proof.1));
    self
  }
}

impl<T, U, V> DocumentSigner<'_, '_, '_, T, U, V> {
  /// Signs the provided data with the configured verification method.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the signature operation fails.
  pub fn sign<X>(&self, that: &mut X) -> Result<()>
  where
    X: Serialize + SetSignature + TryMethod,
  {
    let query: MethodQuery<'_> = self.method.clone().ok_or(Error::MethodNotFound)?;
    let method: &VerificationMethod<U> = self.document.try_resolve_method(query)?;
    let method_uri: String = X::try_method(method)?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        JcsEd25519::<Ed25519>::create_signature(
          that,
          method_uri,
          self.private.as_ref(),
          self.created,
          self.expires,
          self.challenge.clone(),
          self.domain.clone(),
          self.purpose.clone(),
        )?;
      }
      MethodType::MerkleKeyCollection2021 => {
        let data: Vec<u8> = method.key_data().try_decode()?;

        match MerkleKey::extract_tags(&data)? {
          (MerkleSignatureTag::ED25519, MerkleDigestTag::SHA256) => {
            self.merkle_key_sign::<X, Sha256, Ed25519>(that, method_uri)?;
          }
          (MerkleSignatureTag::ED25519, MerkleDigestTag::BLAKE2B_256) => {
            self.merkle_key_sign::<X, Blake2b256, Ed25519>(that, method_uri)?;
          }
          (_, _) => {
            return Err(Error::InvalidMethodType);
          }
        }
      }
    }

    Ok(())
  }

  fn merkle_key_sign<X, D, S>(&self, that: &mut X, method: String) -> Result<()>
  where
    X: Serialize + SetSignature,
    D: MerkleDigest,
    S: MerkleSignature + Sign<Private = [u8]>,
    S::Output: AsRef<[u8]>,
  {
    match self.merkle_key {
      Some((public, proof)) => {
        let proof: &Proof<D> = proof
          .downcast_ref()
          .ok_or(Error::CoreError(CoreError::InvalidKeyFormat))?;

        let skey: SigningKey<'_, D> = SigningKey::from_borrowed(public, self.private, proof);

        MerkleSigner::<D, S>::create_signature(
          that,
          method,
          &skey,
          self.created,
          self.expires,
          self.challenge.clone(),
          self.domain.clone(),
          self.purpose.clone(),
        )?;

        Ok(())
      }
      None => Err(Error::CoreError(CoreError::InvalidKeyFormat)),
    }
  }
}

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
