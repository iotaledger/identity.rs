// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::borrow::Borrow;

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
/// A blanket implementation is provided for the [`Document`] trait, which can be implemented
/// instead to be compatible. Any changes to this trait will be considered non-breaking.
pub trait ValidatorDocument: Sealed {
  /// Convenience function for casting self to the trait.
  ///
  /// Equivalent to `self as &dyn ValidatorDocument`.
  fn as_validator(&self) -> &dyn ValidatorDocument
  where
    Self: Sized,
  {
    self as &dyn ValidatorDocument
  }

  /// Helper method to upcast to an [`Any`] trait object.
  /// The intended use case is to enable downcasting to a concrete [`Document`].
  fn upcast(self: Box<Self>) -> Box<dyn Any>
  where
    Self: 'static;

  /// Returns the string identifier of the DID Document.
  fn did_str(&self) -> &str;

  /// Verifies the signature of the provided data against the DID Document.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, data
  /// serialization fails, or the verification operation fails.
  fn verify_data(&self, data: &dyn Verifiable, options: &VerifierOptions) -> identity_did::Result<()>;

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

/// Specialization of [`ValidatorDocument`](ValidatorDocument) implemented by types that
/// are also [`Send`] and  [`Sync`].
///
/// NOTE: this is a sealed trait and not intended to be used externally or implemented manually.
/// Types which implement [`Document`], [`Send`] and [`Sync`] automatically implement this trait.
/// Any changes to this trait will be considered non-breaking.
pub trait ThreadSafeValidatorDocument: ValidatorDocument + Send + Sync {
  /// Thread safe variant of [`ValidatorDocument::upcast`](ValidatorDocument::upcast()).
  fn thread_safe_upcast(self: Box<Self>) -> Box<dyn Any + Send + Sync + 'static>
  where
    Self: 'static;
}

mod private {
  use super::*;

  pub trait Sealed {}

  impl<T> Sealed for T where T: Document {}
  impl Sealed for &dyn ValidatorDocument {}
  impl Sealed for Box<dyn ValidatorDocument> {}
  impl Sealed for Box<dyn ThreadSafeValidatorDocument> {}

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

impl<DOC> ValidatorDocument for DOC
where
  DOC: Document,
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

impl<DOC> ThreadSafeValidatorDocument for DOC
where
  DOC: ValidatorDocument + Send + Sync,
{
  fn thread_safe_upcast(self: Box<Self>) -> Box<dyn Any + Send + Sync + 'static>
  where
    Self: 'static,
  {
    self
  }
}

impl<DOC> From<DOC> for Box<dyn ValidatorDocument>
where
  DOC: Document + 'static,
{
  fn from(document: DOC) -> Self {
    Box::new(document)
  }
}

impl<DOC> From<DOC> for Box<dyn ThreadSafeValidatorDocument>
where
  DOC: Document + 'static + Send + Sync,
{
  fn from(document: DOC) -> Self {
    Box::new(document)
  }
}

/// Trait implemented by types capable of handing out a borrow implementing [`ValidatorDocument`].
///
/// NOTE: this is a sealed trait and its methods are not intended to be called externally or implemented manually.
/// A blanket implementation is provided for the [`Document`] trait, which can be implemented
/// instead to be compatible. Any changes to this trait will be considered non-breaking.
///
/// # Necessity (why not just use [`ValidatorDocument`]?)
///
/// This trait was introduced in order to achieve both of the following :
/// 1. Enable passing [`Box<dyn ValidatorDocument`] and `&`[Box<dyn ValidatorDocument>]` to the
/// [`PresentationValidator`](crate::validator::PresentationValidator). 2. Provide a blanket implementation for
/// converting any [`Document`] implementor into [`Box<dyn ValidatorDocument>`] via the [`Into`] trait.
///
/// The first of the two points can be achieved by implementing `ValidatorDocument` directly for [`Box<dyn
/// ValidatorDocument>`], but then the second point will not be achievable because of the [Orphan rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules).
///   
/// In terms of abstract functionality requiring this trait bound is essentially equivalent to the following
/// constraints: T: Borrow<U>,
/// U: ValidatorDocument  
/// thus reducing the number of necessary generic parameters in methods and frees the caller from
/// having to use turbofish to declare the concrete type of U. If the exact details of `U` (beyond equivalence of `Eq`,
/// `Ord` and `Hash`) are important then in that case one should not consider these trait bounds equivalent.
pub trait BorrowValidator: private::Sealed {
  /// The concrete ValidatorDocument one may borrow.  
  type BorrowedValidator: ValidatorDocument + ?Sized;
  /// Hands out a borrowed representative capable of calling the methods of [`ValidatorDocument`].
  fn borrow_validator(&self) -> &Self::BorrowedValidator;
}

impl<DOC> BorrowValidator for DOC
where
  DOC: Document + Send + Sync,
{
  type BorrowedValidator = DOC;
  /// Equivalent to: `<Self as Borrow<Self>>::borrow(self)`.
  fn borrow_validator(&self) -> &Self::BorrowedValidator {
    <Self as Borrow<Self>>::borrow(self)
  }
}

impl BorrowValidator for Box<dyn ValidatorDocument> {
  type BorrowedValidator = dyn ValidatorDocument;
  /// Equivalent to: ` <Self as Borrow<dyn ValidatorDocument>>::borrow(self)`.
  fn borrow_validator(&self) -> &Self::BorrowedValidator {
    <Self as Borrow<dyn ValidatorDocument>>::borrow(self)
  }
}

impl BorrowValidator for Box<dyn ThreadSafeValidatorDocument> {
  type BorrowedValidator = dyn ThreadSafeValidatorDocument;
  /// Equivalent to: ` <Self as Borrow<dyn ValidatorDocument>>::borrow(self)`.
  fn borrow_validator(&self) -> &Self::BorrowedValidator {
    <Self as Borrow<dyn ThreadSafeValidatorDocument>>::borrow(self)
  }
}

impl<'a> BorrowValidator for &'a dyn ValidatorDocument {
  type BorrowedValidator = dyn ValidatorDocument + 'a;
  /// Equivalent to: `<Self as Borrow<dyn ValidatorDocument>>::borrow(self)`.
  fn borrow_validator(&self) -> &Self::BorrowedValidator {
    <Self as Borrow<dyn ValidatorDocument>>::borrow(self)
  }
}
