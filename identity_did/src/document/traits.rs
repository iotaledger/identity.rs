// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use identity_core::crypto::GetSignature;

use crate::did::DID;
use crate::service::Service;
use crate::utils::DIDUrlQuery;
use crate::verifiable::VerifierOptions;
use crate::verification::MethodScope;
use crate::verification::VerificationMethod;
use crate::Result;

// TODO: add sign_data, split sign/verify to separate trait as first step towards
//       supporting custom signature schemes. Replace DocumentSigner with trait?
//       Add DocumentMut for mutable function returns?
//       Blanket impl for &T, &mut T, Box<T> etc.?
/// Common operations for DID Documents.
pub trait Document {
  type D: DID;
  type U;
  type V;

  /// Returns a reference to the `Document` id.
  fn id(&self) -> &Self::D;

  /// Returns the first [`Service`] with an `id` property matching the provided `query`, if present.
  fn resolve_service<'query, 'me, Q>(&'me self, query: Q) -> Option<&Service<Self::D, Self::V>>
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

impl<DOC: Document> Document for &DOC {
  type D = DOC::D;
  type U = DOC::U;
  type V = DOC::V;

  fn id(&self) -> &Self::D {
    DOC::id(self)
  }

  fn resolve_service<'query, 'me, Q>(&'me self, query: Q) -> Option<&Service<Self::D, Self::V>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    DOC::resolve_service(self, query)
  }

  fn resolve_method<'query, 'me, Q>(
    &'me self,
    query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&VerificationMethod<Self::D, Self::U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    DOC::resolve_method(self, query, scope)
  }

  fn verify_data<X>(&self, data: &X, options: &VerifierOptions) -> Result<()>
  where
    X: Serialize + GetSignature + ?Sized,
  {
    DOC::verify_data(self, data, options)
  }
}
