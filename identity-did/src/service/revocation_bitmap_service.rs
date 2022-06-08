// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use dataurl::DataUrl;
use identity_core::common::Object;

use super::Service;
use super::ServiceEndpoint;
use crate::did::CoreDID;
use crate::did::DIDUrl;
use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::revocation::RevocationBitmap;

/// A DID Document Service used to enable validators to check the status of a credential.
#[derive(Clone, Debug, PartialEq)]
pub struct RevocationBitmapService<D: DID = CoreDID>(Service<D>);

impl<D: DID> RevocationBitmapService<D> {
  /// The name of the service.
  pub const TYPE: &'static str = "RevocationBitmap2022";

  /// Creates a new [`RevocationBitmapService`] from an identifier and a [`RevocationBitmap`].
  pub fn new(id: DIDUrl<D>, revocation_bitmap: &RevocationBitmap) -> Result<Self, Error> {
    let service: Service<D> = Service::builder(Object::new())
      .id(id)
      .type_(Self::TYPE)
      .service_endpoint(revocation_bitmap.to_endpoint()?)
      .build()?;

    Ok(Self(service))
  }

  /// Returns a reference to the `BitmapRevocationService` id.
  pub fn id(&self) -> &DIDUrl<D> {
    &self.0.id
  }

  /// Returns a reference to the `BitmapRevocationService` type.
  pub fn type_(&self) -> &str {
    &self.0.type_
  }

  /// Returns a reference to the `BitmapRevocationService` endpoint.
  pub fn service_endpoint(&self) -> &ServiceEndpoint {
    &self.0.service_endpoint
  }
}

impl<D: DID + Sized> TryFrom<Service<D>> for RevocationBitmapService<D> {
  type Error = Error;

  fn try_from(service: Service<D>) -> Result<Self> {
    if service.type_() != Self::TYPE {
      return Err(Error::InvalidService("invalid type - unexpected revocation method"));
    }
    match service.service_endpoint() {
      ServiceEndpoint::One(url) => {
        if service.type_() != Self::TYPE {
          Err(Error::InvalidService(
            "invalid service - expected a `RevocationBitmap2022`",
          ))
        } else if url.scheme() != "data" {
          Err(Error::InvalidService("invalid service - expected a data url"))
        } else {
          Ok(Self(service))
        }
      }
      ServiceEndpoint::Map(_) | ServiceEndpoint::Set(_) => {
        Err(Error::InvalidService("invalid endpoint - expected a single data url"))
      }
    }
  }
}

impl<D: DID + Sized> From<RevocationBitmapService<D>> for Service<D> {
  fn from(bitmap_service: RevocationBitmapService<D>) -> Self {
    bitmap_service.0
  }
}

impl<D: DID + Sized> TryFrom<RevocationBitmapService<D>> for RevocationBitmap {
  type Error = Error;

  fn try_from(service: RevocationBitmapService<D>) -> Result<Self, Self::Error> {
    match service.service_endpoint() {
      ServiceEndpoint::One(url) => {
        if url.scheme() == "data" {
          let data_url: DataUrl =
            DataUrl::parse(url.as_str()).map_err(|_| Error::InvalidService("invalid url - expected a data url"))?;

          RevocationBitmap::deserialize_compressed_b64(
            std::str::from_utf8(data_url.get_data())
              .map_err(|_| Error::InvalidService("invalid data url - expected valid utf-8"))?,
          )
        } else {
          Err(Error::InvalidService("invalid url - expected a data url"))
        }
      }
      ServiceEndpoint::Set(_) | ServiceEndpoint::Map(_) => {
        Err(Error::InvalidService("invalid endpoint - expected a single data url"))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::Url;

  use super::Service;
  use crate::did::CoreDID;
  use crate::did::DIDUrl;
  use crate::error::Error;
  use crate::revocation::RevocationBitmap;
  use crate::service::RevocationBitmapService;
  use crate::service::ServiceEndpoint;

  const TAG: &str = "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";
  const SERVICE: &str = "revocation";

  #[test]
  fn test_bitmap_revocation_service_invariants() {
    let service_url: DIDUrl<CoreDID> = DIDUrl::parse(format!("did:iota:{}#{}", TAG, SERVICE)).unwrap();

    let mut revocation_bitmap: RevocationBitmap = RevocationBitmap::new();
    for index in [1, 4, 9, 16000] {
      revocation_bitmap.revoke(index);
    }
    let revocation_bitmap_endpoint: ServiceEndpoint = revocation_bitmap.to_endpoint().unwrap();

    let revocation_bitmap_service = RevocationBitmapService::new(service_url.clone(), &revocation_bitmap).unwrap();

    let service: Service<CoreDID> = Service::builder(Object::new())
      .id(service_url.clone())
      .type_(RevocationBitmapService::<CoreDID>::TYPE)
      .service_endpoint(revocation_bitmap_endpoint.clone())
      .build()
      .unwrap();
    assert_eq!(revocation_bitmap_service, service.try_into().unwrap());

    let invalid_service_type: Service<CoreDID> = Service::builder(Object::new())
      .id(service_url.clone())
      .type_("DifferentRevocationType")
      .service_endpoint(revocation_bitmap_endpoint)
      .build()
      .unwrap();

    assert!(matches!(
      RevocationBitmapService::try_from(invalid_service_type).unwrap_err(),
      Error::InvalidService(_)
    ));

    let invalid_url: Service<CoreDID> = Service::builder(Object::new())
      .id(service_url)
      .type_(RevocationBitmapService::<CoreDID>::TYPE)
      .service_endpoint(ServiceEndpoint::One(Url::parse("http://example.com/").unwrap()))
      .build()
      .unwrap();

    assert!(matches!(
      RevocationBitmapService::try_from(invalid_url).unwrap_err(),
      Error::InvalidService(_)
    ));
  }
}
