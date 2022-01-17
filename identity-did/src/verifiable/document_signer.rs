// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::crypto::merkle_key::Blake2b256;
use identity_core::crypto::merkle_key::MerkleDigest;
use identity_core::crypto::merkle_key::MerkleDigestTag;
use identity_core::crypto::merkle_key::MerkleKey;
use identity_core::crypto::merkle_key::MerkleSignature;
use identity_core::crypto::merkle_key::MerkleSignatureTag;
use identity_core::crypto::merkle_key::MerkleSigner;
use identity_core::crypto::merkle_key::Sha256;
use identity_core::crypto::merkle_key::SigningKey;
use identity_core::crypto::merkle_tree::Proof;
use identity_core::crypto::Ed25519;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::ProofPurpose;
use identity_core::crypto::PublicKey;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Sign;
use identity_core::crypto::SignatureOptions;
use identity_core::crypto::Signer;
use identity_core::Error as CoreError;

use crate::document::CoreDocument;
use crate::verification::MethodQuery;
use crate::verification::MethodType;
use crate::verification::TryMethod;
use crate::verification::VerificationMethod;
use crate::Error;
use crate::Result;

// =============================================================================
// Document Signer - Simplifying Digital Signature Creation Since 2021
// =============================================================================

pub struct DocumentSigner<'base, 'query, 'proof, T = Object, U = Object, V = Object> {
  document: &'base CoreDocument<T, U, V>,
  private: &'base PrivateKey,
  method: Option<MethodQuery<'query>>,
  merkle_key: Option<(&'proof PublicKey, &'proof dyn Any)>,
  options: SignatureOptions,
}

impl<'base, T, U, V> DocumentSigner<'base, '_, '_, T, U, V> {
  pub fn new(document: &'base CoreDocument<T, U, V>, private: &'base PrivateKey) -> Self {
    Self {
      document,
      private,
      method: None,
      merkle_key: None,
      options: SignatureOptions::default(),
    }
  }

  /// Overwrites the [`SignatureOptions`].
  #[must_use]
  pub fn options(mut self, options: SignatureOptions) -> Self {
    self.options = options;
    self
  }

  /// Sets the [`Signature::created`](identity_core::crypto::Signature::created) field.
  #[must_use]
  pub fn created(mut self, created: Timestamp) -> Self {
    self.options = self.options.created(created);
    self
  }

  /// Sets the [`Signature::expires`](identity_core::crypto::Signature::expires) field.
  /// The signature will fail validation after the specified datetime.
  #[must_use]
  pub fn expires(mut self, expires: Timestamp) -> Self {
    self.options = self.options.expires(expires);
    self
  }

  /// Sets the [`Signature::challenge`](identity_core::crypto::Signature::challenge) field.
  #[must_use]
  pub fn challenge(mut self, challenge: String) -> Self {
    self.options = self.options.challenge(challenge);
    self
  }

  /// Sets the [`Signature::domain`](identity_core::crypto::Signature::domain) field.
  #[must_use]
  pub fn domain(mut self, domain: String) -> Self {
    self.options = self.options.domain(domain);
    self
  }

  /// Sets the [`Signature::purpose`](identity_core::crypto::Signature::purpose) field.
  #[must_use]
  pub fn purpose(mut self, purpose: ProofPurpose) -> Self {
    self.options = self.options.purpose(purpose);
    self
  }
}

impl<'base, 'query, T, U, V> DocumentSigner<'base, 'query, '_, T, U, V> {
  #[must_use]
  pub fn method<Q>(mut self, value: Q) -> Self
  where
    Q: Into<MethodQuery<'query>>,
  {
    self.method = Some(value.into());
    self
  }
}

impl<'proof, T, U, V> DocumentSigner<'_, '_, 'proof, T, U, V> {
  #[must_use]
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
        JcsEd25519::<Ed25519>::create_signature(that, method_uri, self.private.as_ref(), self.options.clone())?;
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

        MerkleSigner::<D, S>::create_signature(that, method, &skey, self.options.clone())?;

        Ok(())
      }
      None => Err(Error::CoreError(CoreError::InvalidKeyFormat)),
    }
  }
}
