// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;
use identity_core::common::KeyComparable;
use identity_core::crypto::GetSignature;
use crate::did::DID;
use crate::document::CoreDocument;
use crate::service::Service;
use crate::utils::{DIDUrlQuery, Queryable};
use crate::verifiable::VerifierOptions;
use crate::verification::{MethodScope, VerificationMethod};
use crate::Result;

/// Common operations for DID Documents.
// TODO: add sign_data, split sign/verify to separate trait as first step towards
//       supporting custom signature schemes.
//       Remove DocumentSigner?
pub trait Document {
  type D: DID;
  type U;
  type V;

  /// Returns a reference to the `Document` id.
  fn id(&self) -> &Self::D;

  // /// Returns a reference to the `CoreDocument` controller.
  // pub fn controller(&self) -> Option<&OneOrSet<D>> {
  //   self.controller.as_ref()
  // }
  //
  // /// Returns a reference to the `CoreDocument` alsoKnownAs set.
  // pub fn also_known_as(&self) -> &OrderedSet<Url> {
  //   &self.also_known_as
  // }
  //
  // /// Returns a reference to the `CoreDocument` verificationMethod set.
  // pub fn verification_method(&self) -> &OrderedSet<VerificationMethod<D, U>> {
  //   &self.verification_method
  // }
  //
  // /// Returns a reference to the `CoreDocument` authentication set.
  // pub fn authentication(&self) -> &OrderedSet<MethodRef<D, U>> {
  //   &self.authentication
  // }
  //
  // /// Returns a reference to the `CoreDocument` assertionMethod set.
  // pub fn assertion_method(&self) -> &OrderedSet<MethodRef<D, U>> {
  //   &self.assertion_method
  // }
  //
  // /// Returns a reference to the `CoreDocument` keyAgreement set.
  // pub fn key_agreement(&self) -> &OrderedSet<MethodRef<D, U>> {
  //   &self.key_agreement
  // }
  //
  // /// Returns a reference to the `CoreDocument` capabilityDelegation set.
  // pub fn capability_delegation(&self) -> &OrderedSet<MethodRef<D, U>> {
  //   &self.capability_delegation
  // }
  //
  // /// Returns a reference to the `CoreDocument` capabilityInvocation set.
  // pub fn capability_invocation(&self) -> &OrderedSet<MethodRef<D, U>> {
  //   &self.capability_invocation
  // }
  //
  // /// Returns a reference to the `CoreDocument` service set.
  // pub fn service(&self) -> &OrderedSet<Service<D, V>> {
  //   &self.service
  // }

  /// Returns the first [`Service`] with an `id` property matching the provided `query`, if present.
  fn resolve_service<'query, 'me, Q>(
    &'me self,
    query: Q,
  ) -> Option<&Service<Self::D, Self::V>>
    where
      Q: Into<DIDUrlQuery<'query>>;

  /// Returns the first [`VerificationMethod`] with an `id` property matching the
  /// provided `query` and the verification relationship specified by `scope` if present.
  fn resolve_method<'query, 'me, Q>(
    &'me self,
    query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&VerificationMethod<Self::D, Self::U>>
    where
      Q: Into<DIDUrlQuery<'query>>;

  /// Verifies the signature of the provided data.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, data
  /// serialization fails, or the verification operation fails.
  fn verify_data<X>(&self, data: &X, options: &VerifierOptions) -> Result<()>
    where
      X: Serialize + GetSignature + ?Sized;
}
impl<D, T, U, V> Document for CoreDocument<D, T, U, V>
  where
    D: DID + KeyComparable,
{
  type D = D;
  type U = U;
  type V = V;

  fn id(&self) -> &D {
    CoreDocument::id(self)
  }

  fn resolve_service<'query, 'me, Q>(&'me self, query: Q) -> Option<&Service<Self::D, Self::V>> where Q: Into<DIDUrlQuery<'query>> {
    self
      .service()
      .query(query.into())
  }

  fn resolve_method<'query, 'me, Q>(&'me self, query: Q, scope: Option<MethodScope>) -> Option<&VerificationMethod<D, U>> where Q: Into<DIDUrlQuery<'query>> {
    CoreDocument::resolve_method(self, query, scope)
  }

  fn verify_data<X>(&self, data: &X, options: &VerifierOptions) -> Result<()> where X: Serialize + GetSignature + ?Sized {
    CoreDocument::verify_data(self, data, options)
  }
}
