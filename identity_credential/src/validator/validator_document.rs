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

  /// Returns the string identifier of the DID Document.
  fn did_str(&self) -> &str;

  /// Helper method to upcast to an [`Any`] trait object.
  /// The intended use case is to enable downcasting to a concrete [`Document`].
  fn into_any(self: Box<Self>) -> Box<dyn Any>
  where
    Self: 'static;

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

mod private {
  use super::*;

  pub trait Sealed {}

  impl<T> Sealed for T where T: Document {}
  impl Sealed for &dyn ValidatorDocument {}
  impl Sealed for Box<dyn ValidatorDocument> {}

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

/*
impl ValidatorDocument for &dyn ValidatorDocument {
  fn did_str(&self) -> &str {
    (*self).did_str()
  }

  fn verify_data(&self, data: &dyn Verifiable, options: &VerifierOptions) -> identity_did::Result<()> {
    (*self).verify_data(data, options)
  }

  fn into_any(self: Box<Self>) -> Box<dyn Any>
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
*/

/* 
impl ValidatorDocument for Box<dyn ValidatorDocument> {
  fn did_str(&self) -> &str {
    let reference: &dyn ValidatorDocument = self.as_ref();
    reference.did_str()
  }

  fn into_any(self: Box<Self>) -> Box<dyn Any>
  where
    Self: 'static,
  {
    self
  }

  fn verify_data(&self, data: &dyn Verifiable, options: &VerifierOptions) -> identity_did::Result<()> {
    let reference: &dyn ValidatorDocument = self.as_ref();
    reference.verify_data(data, options)
  }

  #[cfg(feature = "revocation-bitmap")]
  fn resolve_revocation_bitmap(
    &self,
    query: identity_did::utils::DIDUrlQuery<'_>,
  ) -> identity_did::Result<RevocationBitmap> {
    let reference: &dyn ValidatorDocument = self.as_ref();
    reference.resolve_revocation_bitmap(query)
  }
}

*/
impl<DOC> From<DOC> for Box<dyn ValidatorDocument> where DOC: Document + 'static {
  fn from(document: DOC) -> Self {
      Box::new(document)
  }
}

trait ValidatorRef {
  type Ref: ValidatorDocument + ?Sized;

  fn validator_ref(&self) -> &Self::Ref; 
}

impl<DOC> ValidatorRef for DOC where DOC: Document {
  type Ref = DOC;
  fn validator_ref(&self) -> &Self::Ref {
      &self
  }
}

impl ValidatorRef for Box<dyn ValidatorDocument> {
  type Ref = dyn ValidatorDocument;
  fn validator_ref(&self) -> &Self::Ref {
      self.as_ref()
  }
}

impl<'a> ValidatorRef for &'a dyn ValidatorDocument {
  type Ref = dyn ValidatorDocument + 'a;
  fn validator_ref(&self) -> &Self::Ref {
      self.borrow()
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

  fn into_any(self: Box<Self>) -> Box<dyn Any>
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
