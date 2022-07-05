// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::GetSignature;
use identity_did::did::DID;
use identity_did::document::Document;
use identity_did::revocation::RevocationBitmap;
use identity_did::utils::DIDUrlQuery;
use identity_did::verifiable::VerifierOptions;

use crate::credential::validator_document::private::Sealed;
use crate::credential::validator_document::private::Verifiable;

/// Abstraction over DID Documents for validating presentations and credentials.
///
/// NOTE: this is a sealed trait and not intended to be used externally or implemented manually.
/// A blanket implementation is provided for the [`Document`] trait, which can be implemented
/// instead to be compatible. Any changes to this trait will be considered non-breaking.
pub trait ValidatorDocument: Sealed {
  fn did_str(&self) -> &str;

  fn verify_data(
    &self,
    data: &dyn Verifiable,
    options: &VerifierOptions,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

  #[cfg(feature = "revocation-bitmap")]
  fn resolve_revocation_bitmap(&self, query: DIDUrlQuery<'_>) -> identity_did::Result<RevocationBitmap>;
}

mod private {
  use super::*;

  pub trait Sealed {}

  impl<T> Sealed for T where T: Document {}

  /// Object-safe trait workaround to satisfy the trait bounds
  /// [`serde::Serialize`] + [`GetSignature`] with dynamic dispatch.
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

  fn verify_data(
    &self,
    data: &dyn Verifiable,
    options: &VerifierOptions,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    self.verify_data(data, options).map_err(Into::into)
  }

  #[cfg(feature = "revocation-bitmap")]
  fn resolve_revocation_bitmap(&self, query: DIDUrlQuery<'_>) -> identity_did::Result<RevocationBitmap> {
    self
      .resolve_service(query)
      .ok_or(identity_did::Error::InvalidService(
        "revocation bitmap service not found",
      ))
      .and_then(RevocationBitmap::try_from)
  }
}
