// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use identity_core::crypto::GetSignature;
use identity_did::did::DID;
use identity_did::document::Document;
#[cfg(feature = "revocation-bitmap")]
use identity_did::revocation::RevocationBitmap;
use identity_did::verifiable::VerifierOptions;

use self::private::Sealed;
use self::private::Verifiable;

/// Abstraction over DID Documents for validating presentations and credentials.
///
/// NOTE: this is a sealed trait and not intended to be used externally or implemented manually.
/// A blanket implementation is provided for all types implementing the [`Document`] and [`Debug`](core::fmt::Debug)
/// traits, which is recommended to be implemented instead to be compatible.
/// 
/// # Warning
///
/// Any changes to this trait will be considered non-breaking.
pub trait ValidatorDocument: Sealed + core::fmt::Debug {
  /// Convenience function for casting self to the trait.
  ///
  /// Equivalent to `self as &dyn ValidatorDocument`.
  fn as_validator(&self) -> &dyn ValidatorDocument
  where
    Self: Sized,
  {
    self as &dyn ValidatorDocument
  }

  #[doc(hidden)]
  fn upcast(self: Box<Self>) -> Box<dyn Any>
  where
    Self: 'static;

  #[doc(hidden)]
  /// Returns the string identifier of the DID Document.
  fn did_str(&self) -> &str;

  #[doc(hidden)]
  /// Verifies the signature of the provided data against the DID Document.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, data
  /// serialization fails, or the verification operation fails.
  fn verify_data(&self, data: &dyn Verifiable, options: &VerifierOptions) -> identity_did::Result<()>;

  #[doc(hidden)]
  /// Extracts the `RevocationBitmap` from the referenced service in the DID Document.
  ///
  /// # Errors
  ///
  /// Fails if the referenced service is not found, or is not a
  /// valid `RevocationBitmap2022` service.
  #[cfg(feature = "revocation-bitmap")]
  fn resolve_revocation_bitmap(
    &self,
    query: identity_did::utils::DIDUrlQuery<'_>,
  ) -> identity_did::Result<RevocationBitmap>;
}

mod private {
  use super::*;

  pub trait Sealed {}

  impl<T> Sealed for T where T: Document {}
  impl Sealed for &dyn ValidatorDocument {}
  impl Sealed for AbstractValidatorDocument {}
  impl Sealed for AbstractThreadSafeValidatorDocument {}

  /// Object-safe trait workaround to satisfy the trait bounds
  /// [`serde::Serialize`] + [`GetSignature`].
  pub trait Verifiable: erased_serde::Serialize + GetSignature {}

  impl<T> Verifiable for T where T: erased_serde::Serialize + GetSignature {}

  impl<'a> serde::Serialize for dyn Verifiable + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      erased_serde::serialize(self, serializer)
    }
  }
}

impl ValidatorDocument for &dyn ValidatorDocument {
  fn did_str(&self) -> &str {
    (*self).did_str()
  }

  fn verify_data(&self, data: &dyn Verifiable, options: &VerifierOptions) -> identity_did::Result<()> {
    (*self).verify_data(data, options)
  }
  fn upcast(self: Box<Self>) -> Box<dyn Any>
  where
    Self: 'static,
  {
    self
  }

  #[cfg(feature = "revocation-bitmap")]
  fn resolve_revocation_bitmap(
    &self,
    query: identity_did::utils::DIDUrlQuery<'_>,
  ) -> identity_did::Result<RevocationBitmap> {
    (*self).resolve_revocation_bitmap(query)
  }
}

impl<DOC> ValidatorDocument for DOC
where
  DOC: Document + core::fmt::Debug,
{
  fn did_str(&self) -> &str {
    self.id().as_str()
  }

  fn verify_data(&self, data: &dyn Verifiable, options: &VerifierOptions) -> identity_did::Result<()> {
    self.verify_data(data, options).map_err(Into::into)
  }

  fn upcast(self: Box<Self>) -> Box<dyn Any>
  where
    Self: 'static,
  {
    self
  }

  #[cfg(feature = "revocation-bitmap")]
  fn resolve_revocation_bitmap(
    &self,
    query: identity_did::utils::DIDUrlQuery<'_>,
  ) -> identity_did::Result<RevocationBitmap> {
    self
      .resolve_service(query)
      .ok_or(identity_did::Error::InvalidService(
        "revocation bitmap service not found",
      ))
      .and_then(RevocationBitmap::try_from)
  }
}

/// An abstract implementer of [`ValidatorDocument`] that all implementers of [`Document`] can be converted to.
///
/// By calling [`Self::into_any`](Self::into_any()) one obtains a type that one may
/// attempt to convert to a concrete DID Document representation.
#[derive(Debug)]
pub struct AbstractValidatorDocument(Box<dyn ValidatorDocument>);

impl AbstractValidatorDocument {
  /// See [AbstractThreadSafeValidatorDocument::into_any](AbstractThreadSafeValidatorDocument::into_any()).
  pub fn into_any(self) -> Box<dyn Any> {
    self.0.upcast()
  }
}

impl<DOC: Document + core::fmt::Debug + 'static> From<DOC> for AbstractValidatorDocument {
  fn from(doc: DOC) -> Self {
    AbstractValidatorDocument(Box::new(doc) as Box<dyn ValidatorDocument>)
  }
}

impl ValidatorDocument for AbstractValidatorDocument {
  fn did_str(&self) -> &str {
    self.0.did_str()
  }

  fn verify_data(&self, data: &dyn Verifiable, options: &VerifierOptions) -> identity_did::Result<()> {
    self.0.verify_data(data, options)
  }

  fn upcast(self: Box<Self>) -> Box<dyn Any>
  where
    Self: 'static,
  {
    self.into_any()
  }

  #[cfg(feature = "revocation-bitmap")]
  fn resolve_revocation_bitmap(
    &self,
    query: identity_did::utils::DIDUrlQuery<'_>,
  ) -> identity_did::Result<RevocationBitmap> {
    self.0.resolve_revocation_bitmap(query)
  }
}

trait ThreadSafeValidatorDocument: ValidatorDocument + Send + Sync {
  fn thread_safe_upcast(self: Box<Self>) -> Box<dyn Any + Send + Sync>
  where
    Self: 'static;
}

impl<DOC> ThreadSafeValidatorDocument for DOC
where
  DOC: ValidatorDocument + Send + Sync + 'static,
{
  fn thread_safe_upcast(self: Box<Self>) -> Box<dyn Any + Send + Sync>
  where
    Self: 'static,
  {
    self
  }
}

/// Thread safe variant of [`AbstractValidatorDocument`].
///
/// By calling [`Self::into_any`](Self::into_any()) one obtains a type that one may
/// attempt to downcast to a concrete DID Document representation.
#[derive(Debug)]
pub struct AbstractThreadSafeValidatorDocument(Box<dyn ThreadSafeValidatorDocument>);

impl<DOC> From<DOC> for AbstractThreadSafeValidatorDocument
where
  DOC: Document + core::fmt::Debug + Send + Sync + 'static,
{
  fn from(doc: DOC) -> Self {
    Self(Box::new(doc) as Box<dyn ThreadSafeValidatorDocument>)
  }
}

impl AbstractThreadSafeValidatorDocument {
  /// Convert the abstract document into [`Any`] which one may then attempt to cast to a concrete type.
  ///
  /// # Example
  /// ```
  /// # use identity_did::document::CoreDocument;
  /// # use identity_credential::validator::AbstractValidatorDocument;
  ///
  /// fn round_trip(doc: CoreDocument) -> CoreDocument {
  ///   let abstract_doc = AbstractValidatorDocument::from(doc);
  ///   *abstract_doc.into_any().downcast::<CoreDocument>().unwrap()
  /// }
  /// ```
  pub fn into_any(self) -> Box<dyn Any + Send + Sync> {
    self.0.thread_safe_upcast()
  }
}

impl ValidatorDocument for AbstractThreadSafeValidatorDocument {
  fn verify_data(&self, data: &dyn Verifiable, options: &VerifierOptions) -> identity_did::Result<()> {
    self.0.verify_data(data, options)
  }

  fn upcast(self: Box<Self>) -> Box<dyn Any>
  where
    Self: 'static,
  {
    self.0.upcast()
  }

  fn did_str(&self) -> &str {
    self.0.did_str()
  }

  #[cfg(feature = "revocation-bitmap")]
  fn resolve_revocation_bitmap(
    &self,
    query: identity_did::utils::DIDUrlQuery<'_>,
  ) -> identity_did::Result<RevocationBitmap> {
    self.0.resolve_revocation_bitmap(query)
  }
}
