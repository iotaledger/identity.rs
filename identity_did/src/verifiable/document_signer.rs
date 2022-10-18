// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::did::CoreDID;
use crate::did::DID;
use identity_core::common::KeyComparable;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::crypto::Ed25519;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::ProofOptions;
use identity_core::crypto::ProofPurpose;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signer;

use crate::document::CoreDocument;
use crate::utils::DIDUrlQuery;
use crate::verification::MethodType;
use crate::verification::TryMethod;
use crate::verification::VerificationMethod;
use crate::Error;
use crate::Result;

// =============================================================================
// Document Signer - Simplifying Digital Signature Creation Since 2021
// =============================================================================

pub struct DocumentSigner<'base, 'query, D = CoreDID, T = Object, U = Object, V = Object>
where
  D: DID + KeyComparable,
{
  document: &'base CoreDocument<D, T, U, V>,
  private: &'base PrivateKey,
  method: Option<DIDUrlQuery<'query>>,
  options: ProofOptions,
}

impl<'base, D, T, U, V> DocumentSigner<'base, '_, D, T, U, V>
where
  D: DID + KeyComparable,
{
  pub fn new(document: &'base CoreDocument<D, T, U, V>, private: &'base PrivateKey) -> Self {
    Self {
      document,
      private,
      method: None,
      options: ProofOptions::default(),
    }
  }

  /// Overwrites the [`ProofOptions`].
  #[must_use]
  pub fn options(mut self, options: ProofOptions) -> Self {
    self.options = options;
    self
  }

  /// Sets the [`Proof::created`](identity_core::crypto::Proof::created) field.
  #[must_use]
  pub fn created(mut self, created: Timestamp) -> Self {
    self.options = self.options.created(created);
    self
  }

  /// Sets the [`Proof::expires`](identity_core::crypto::Proof::expires) field.
  /// The signature will fail validation after the specified datetime.
  #[must_use]
  pub fn expires(mut self, expires: Timestamp) -> Self {
    self.options = self.options.expires(expires);
    self
  }

  /// Sets the [`Proof::challenge`](identity_core::crypto::Proof::challenge) field.
  #[must_use]
  pub fn challenge(mut self, challenge: String) -> Self {
    self.options = self.options.challenge(challenge);
    self
  }

  /// Sets the [`Proof::domain`](identity_core::crypto::Proof::domain) field.
  #[must_use]
  pub fn domain(mut self, domain: String) -> Self {
    self.options = self.options.domain(domain);
    self
  }

  /// Sets the [`Proof::purpose`](identity_core::crypto::Proof::purpose) field.
  #[must_use]
  pub fn purpose(mut self, purpose: ProofPurpose) -> Self {
    self.options = self.options.purpose(purpose);
    self
  }
}

impl<'base, 'query, D, T, U, V> DocumentSigner<'base, 'query, D, T, U, V>
where
  D: DID + KeyComparable,
{
  #[must_use]
  pub fn method<Q>(mut self, value: Q) -> Self
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.method = Some(value.into());
    self
  }
}

impl<D, T, U, V> DocumentSigner<'_, '_, D, T, U, V>
where
  D: DID + KeyComparable,
{
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
    // TODO: Update this method to reflect the new API.
    let query: DIDUrlQuery<'_> = self.method.clone().ok_or(Error::MethodNotFound)?;
    let method: &VerificationMethod<D, U> = self.document.resolve_method(query, None).ok_or(Error::MethodNotFound)?;
    let method_uri: String = X::try_method(method)?;

    if method.type_() == &MethodType::ED25519_VERIFICATION_KEY_2018 {
      JcsEd25519::<Ed25519>::create_signature(that, method_uri, self.private.as_ref(), self.options.clone())?;
    } else {
      return Err(Error::InvalidMethodType);
    }
    Ok(())
  }
}
