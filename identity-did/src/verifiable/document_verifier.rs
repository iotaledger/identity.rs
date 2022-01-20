// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use identity_core::common::BitSet;
use identity_core::common::Object;
use identity_core::common::Timestamp;
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
use identity_core::crypto::ProofPurpose;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::Verifier;
use identity_core::crypto::Verify;

use crate::document::CoreDocument;
use crate::verifiable::verifier_options::VerifierOptions;
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
  options: VerifierOptions,
}

impl<'base, T, U, V> DocumentVerifier<'base, T, U, V> {
  pub fn new(document: &'base CoreDocument<T, U, V>) -> Self {
    Self {
      document,
      options: VerifierOptions::default(),
    }
  }

  /// Overwrites the [`VerifierOptions`].
  #[must_use]
  pub fn options(mut self, options: VerifierOptions) -> Self {
    self.options = options;
    self
  }

  /// Verify the signing verification method relationship matches this.
  ///
  /// NOTE: `purpose` overrides the `method_scope` option.
  #[must_use]
  pub fn method_scope(mut self, method_scope: MethodScope) -> Self {
    self.options = self.options.method_scope(method_scope);
    self
  }

  /// Verify the signing verification method type matches one specified.
  #[must_use]
  pub fn method_type(mut self, method_type: Vec<MethodType>) -> Self {
    self.options = self.options.method_type(method_type);
    self
  }

  /// Verify the [`Signature::challenge`] field matches this.
  #[must_use]
  pub fn challenge(mut self, challenge: String) -> Self {
    self.options = self.options.challenge(challenge);
    self
  }

  /// Verify the [`Signature::domain`] field matches this.
  #[must_use]
  pub fn domain(mut self, domain: String) -> Self {
    self.options = self.options.domain(domain);
    self
  }

  /// Verify the [`Signature::purpose`] field matches this. Also verifies that the signing
  /// method has the corresponding verification method relationship.
  ///
  /// E.g. [`ProofPurpose::Authentication`] must be signed using a method
  /// with [`MethodRelationship::Authentication`].
  ///
  /// NOTE: `purpose` overrides the `method_scope` option.
  #[must_use]
  pub fn purpose(mut self, purpose: ProofPurpose) -> Self {
    self.options = self.options.purpose(purpose);
    self
  }

  /// Determines whether to error if the current time exceeds the [`Signature::expires`] field.
  ///
  /// Default: false (reject expired signatures).
  #[must_use]
  pub fn allow_expired(mut self, allow_expired: bool) -> Self {
    self.options = self.options.allow_expired(allow_expired);
    self
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
  /// Fails if an unsupported verification method is used, data
  /// serialization fails, or the verification operation fails.
  pub fn verify<X>(&self, data: &X) -> Result<()>
  where
    X: Serialize + TrySignature,
  {
    let signature: &Signature = data
      .try_signature()
      .map_err(|_| Error::InvalidSignature("missing signature"))?;

    // Retrieve the method used to create the signature and check it has the required verification
    // method relationship (purpose takes precedence over method_scope).
    let purpose_scope = self.options.purpose.map(|purpose| match purpose {
      ProofPurpose::AssertionMethod => MethodScope::assertion_method(),
      ProofPurpose::Authentication => MethodScope::authentication(),
    });
    let method: &VerificationMethod<U> = match (purpose_scope, self.options.method_scope) {
      (Some(purpose_scope), _) => self
        .document
        .try_resolve_method_with_scope(signature, purpose_scope)
        .map_err(|_| Error::InvalidSignature("method with purpose scope not found"))?,
      (None, Some(scope)) => self
        .document
        .try_resolve_method_with_scope(signature, scope)
        .map_err(|_| Error::InvalidSignature("method with specified scope not found"))?,
      (None, None) => self
        .document
        .try_resolve_method(signature)
        .map_err(|_| Error::InvalidSignature("method not found"))?,
    };

    // Check method type.
    if let Some(method_types) = &self.options.method_type {
      if !method_types.is_empty() && !method_types.contains(&method.key_type) {
        return Err(Error::InvalidSignature("invalid method type"));
      }
    }

    // Check challenge.
    if self.options.challenge.is_some() && self.options.challenge != signature.challenge {
      return Err(Error::InvalidSignature("invalid challenge"));
    }

    // Check domain.
    if self.options.domain.is_some() && self.options.domain != signature.domain {
      return Err(Error::InvalidSignature("invalid domain"));
    }

    // Check purpose.
    if self.options.purpose.is_some() && self.options.purpose != signature.purpose {
      return Err(Error::InvalidSignature("invalid purpose"));
    }

    // Check expired.
    if let Some(expires) = signature.expires {
      if !self.options.allow_expired.unwrap_or(false) && Timestamp::now_utc() > expires {
        return Err(Error::InvalidSignature("expired"));
      }
    }

    // Check signature.
    Self::do_verify(method, data)
  }

  /// Verifies the signature of the provided data matches the public key data from the given
  /// verification method.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, data
  /// serialization fails, or the verification operation fails.
  pub fn do_verify<X>(method: &VerificationMethod<U>, data: &X) -> Result<()>
  where
    X: Serialize + TrySignature,
  {
    let public_key: Vec<u8> = method.key_data().try_decode()?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        JcsEd25519::<Ed25519>::verify_signature(data, &public_key)?;
      }
      MethodType::MerkleKeyCollection2021 => match MerkleKey::extract_tags(&public_key)? {
        (MerkleSignatureTag::ED25519, MerkleDigestTag::SHA256) => {
          merkle_key_verify::<X, Sha256, Ed25519, U>(data, method, &public_key)?;
        }
        (MerkleSignatureTag::ED25519, MerkleDigestTag::BLAKE2B_256) => {
          merkle_key_verify::<X, Blake2b256, Ed25519, U>(data, method, &public_key)?;
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
