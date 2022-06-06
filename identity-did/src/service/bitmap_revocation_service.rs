// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;

use super::BitmapRevocationEndpoint;
use super::Service;
use super::ServiceEndpoint;
use crate::did::DIDUrl;
use crate::did::DID;
use crate::error::Error;
use crate::error::Result;

pub const BITMAP_REVOCATION_METHOD: &str = "RevocationBitmap2022";

/// A DID Document Service used to enable validators to check the status of a credential.
#[derive(Clone, Debug, PartialEq)]
pub struct BitmapRevocationService<D: DID + Sized> {
  pub(crate) id: DIDUrl<D>,
  pub(crate) type_: String,
  pub(crate) service_endpoint: BitmapRevocationEndpoint,
}

impl<D: DID + Sized> BitmapRevocationService<D> {
  /// Creates a new [`BitmapRevocationService`].
  pub fn new(id: DIDUrl<D>, service_endpoint: BitmapRevocationEndpoint) -> Result<Self> {
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(Error::InvalidService("invalid id - missing fragment"));
    }
    Ok(Self {
      id,
      type_: BITMAP_REVOCATION_METHOD.to_owned(),
      service_endpoint,
    })
  }

  /// Returns a reference to the `BitmapRevocationService` id.
  pub fn id(&self) -> &DIDUrl<D> {
    &self.id
  }

  /// Returns a reference to the `BitmapRevocationService` type.
  pub fn type_(&self) -> &str {
    &*self.type_
  }

  /// Returns a reference to the `BitmapRevocationService` endpoint.
  pub fn service_endpoint(&self) -> &BitmapRevocationEndpoint {
    &self.service_endpoint
  }

  /// Sets the `BitmapRevocationService` id.
  ///
  /// # Errors
  /// [`Error::InvalidService`] if there is no fragment on the [`DIDUrl`].
  pub fn set_id(&mut self, id: DIDUrl<D>) -> Result<()> {
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(Error::InvalidService("invalid id - missing fragment"));
    }
    self.id = id;
    Ok(())
  }

  /// Sets the `BitmapRevocationService` endpoint.
  pub fn set_service_endpoint(&mut self, service_endpoint: BitmapRevocationEndpoint) {
    self.service_endpoint = service_endpoint;
  }
}

impl<D: DID + Sized> TryFrom<Service<D>> for BitmapRevocationService<D> {
  type Error = Error;

  fn try_from(service: Service<D>) -> Result<Self> {
    if service.type_() != BITMAP_REVOCATION_METHOD {
      return Err(Error::InvalidService("invalid type - unexpected revocation method"));
    }
    match service.service_endpoint() {
      ServiceEndpoint::One(url) => Ok(BitmapRevocationService {
        id: service.id().clone(),
        type_: service.type_().to_owned(),
        service_endpoint: url.clone().try_into()?,
      }),
      ServiceEndpoint::Map(_) | ServiceEndpoint::Set(_) => {
        Err(Error::InvalidService("invalid endpoint - expected a single data url"))
      }
    }
  }
}

impl<D: DID + Sized> TryFrom<BitmapRevocationService<D>> for Service<D> {
  type Error = Error;

  fn try_from(bitmap_service: BitmapRevocationService<D>) -> Result<Self> {
    Service::builder(Object::new())
      .id(bitmap_service.id)
      .type_(bitmap_service.type_)
      .service_endpoint(ServiceEndpoint::One(bitmap_service.service_endpoint.try_into()?))
      .build()
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Object;

  use super::BitmapRevocationEndpoint;
  use super::BitmapRevocationService;
  use super::Service;
  use super::ServiceEndpoint;
  use super::BITMAP_REVOCATION_METHOD;
  use crate::did::CoreDID;
  use crate::did::DIDUrl;
  use crate::error::Error;

  const TAG: &str = "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";
  const SERVICE: &str = "revocation";

  #[test]
  fn test_bitmap_revocation_service_invariants() {
    let did_url: DIDUrl<CoreDID> = DIDUrl::parse(format!("did:iota:{}#{}", TAG, SERVICE)).unwrap();
    let service_endpoint: BitmapRevocationEndpoint = BitmapRevocationEndpoint::parse("data:,eJyzMmAAAwADKABr").unwrap();
    let embedded_revocation_service = BitmapRevocationService::new(did_url.clone(), service_endpoint.clone()).unwrap();

    let service: Service<CoreDID> = Service::builder(Object::new())
      .id(did_url.clone())
      .type_(BITMAP_REVOCATION_METHOD)
      .service_endpoint(ServiceEndpoint::One(service_endpoint.clone().try_into().unwrap()))
      .build()
      .unwrap();
    assert_eq!(embedded_revocation_service, service.try_into().unwrap());

    let invalid_service_type: Service<CoreDID> = Service::builder(Object::new())
      .id(did_url)
      .type_("DifferentRevocationType")
      .service_endpoint(ServiceEndpoint::One(service_endpoint.try_into().unwrap()))
      .build()
      .unwrap();

    match BitmapRevocationService::try_from(invalid_service_type).unwrap_err() {
      Error::InvalidService(_) => {}
      _ => panic!("expected invalid service error"),
    }
  }
}
