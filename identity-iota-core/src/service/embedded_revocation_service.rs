// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use identity_core::common::KeyComparable;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use identity_did::did::DIDUrl;
use identity_did::service::deserialize_id_with_fragment;
use identity_did::service::Service;
use identity_did::service::ServiceEndpoint;
use serde::Deserialize;
use serde::Serialize;

use super::error::Result;
use super::error::ServiceError;
use crate::did::IotaDID;
use crate::did::IotaDIDUrl;
use crate::revocation::EmbeddedRevocationList;

/// A DID Document Service used to enable validators to check the status of a credential.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmbeddedRevocationService {
  #[serde(deserialize_with = "deserialize_id_with_fragment")]
  pub(crate) id: IotaDIDUrl,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  #[serde(rename = "serviceEndpoint")]
  pub(crate) service_endpoint: Url,
}

impl EmbeddedRevocationService {
  pub fn new(id: IotaDIDUrl, service_endpoint: Url) -> Result<Self> {
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(ServiceError::InvalidServiceId("missing id fragment".to_owned()));
    }
    Ok(Self {
      id,
      type_: EmbeddedRevocationList::name().to_owned(),
      service_endpoint,
    })
  }

  /// Returns a reference to the `EmbeddedRevocationService` id.
  pub fn id(&self) -> &IotaDIDUrl {
    &self.id
  }

  /// Returns a reference to the `EmbeddedRevocationService` type.
  pub fn type_(&self) -> &str {
    &*self.type_
  }

  /// Returns a reference to the `EmbeddedRevocationService` endpoint.
  pub fn service_endpoint(&self) -> &Url {
    &self.service_endpoint
  }

  /// Sets the `EmbeddedRevocationService` id.
  ///
  /// # Errors
  /// [`Error::InvalidMethodFragment`] if there is no fragment on the [`DIDUrl`].
  pub fn set_id(&mut self, id: IotaDIDUrl) -> Result<()> {
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(ServiceError::InvalidServiceId("missing id fragment".to_owned()));
    }
    self.id = id;
    Ok(())
  }

  /// Updates the `EmbeddedRevocationList` contained in the `service_endpoint` by revoking all given credentials.
  pub fn revoke_credentials(&mut self, credentials: &[u32]) -> Result<()> {
    // Gets the current list and revokes all given credentials
    let mut embedded_revocation_list: EmbeddedRevocationList =
      EmbeddedRevocationList::deserialize_compressed_b64(&self.service_endpoint().clone().into_string())
        .map_err(|e| ServiceError::RevocationMethodError("invalid revocation list".to_owned(), e))?;
    embedded_revocation_list.revoke_multiple(credentials);
    // Sets the service endpoint with the updated revocation list
    let service_endpoint: String = embedded_revocation_list
      .serialize_compressed_b64()
      .map_err(|e| ServiceError::RevocationMethodError("invalid updated revocation list".to_owned(), e))?;
    self.set_service_endpoint(service_endpoint)
  }

  /// Sets the `EmbeddedRevocationService` service endpoint.
  fn set_service_endpoint(&mut self, service_enpoint: impl AsRef<str>) -> Result<()> {
    let url: Url =
      Url::parse(service_enpoint).map_err(|_| ServiceError::InvalidServiceEndpoint("invalid url".to_owned()))?;
    self.service_endpoint = url;
    Ok(())
  }
}

impl TryFrom<Service<IotaDID>> for EmbeddedRevocationService {
  type Error = ServiceError;

  fn try_from(service: Service<IotaDID>) -> Result<Self> {
    if service.type_() != EmbeddedRevocationList::name() {
      return Err(ServiceError::InvalidServiceType(format!(
        "expected {}",
        EmbeddedRevocationList::name()
      )));
    }
    match service.service_endpoint() {
      ServiceEndpoint::One(url) => Ok(EmbeddedRevocationService {
        id: service.id().clone(),
        type_: service.type_().to_owned(),
        service_endpoint: url.clone(),
      }),
      ServiceEndpoint::Map(_) | ServiceEndpoint::Set(_) => Err(ServiceError::InvalidServiceEndpoint(
        "expected single url as value".to_owned(),
      )),
    }
  }
}

impl TryFrom<EmbeddedRevocationService> for Service<IotaDID> {
  type Error = ServiceError;

  fn try_from(service: EmbeddedRevocationService) -> Result<Self> {
    Service::builder(Object::new())
      .id(service.id)
      .type_(service.type_)
      .service_endpoint(ServiceEndpoint::One(service.service_endpoint))
      .build()
      .map_err(ServiceError::InvalidService)
  }
}

impl AsRef<DIDUrl<IotaDID>> for EmbeddedRevocationService {
  fn as_ref(&self) -> &DIDUrl<IotaDID> {
    self.id()
  }
}

impl Display for EmbeddedRevocationService {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl KeyComparable for EmbeddedRevocationService {
  type Key = IotaDIDUrl;

  #[inline]
  fn key(&self) -> &Self::Key {
    self.as_ref()
  }
}

impl Default for EmbeddedRevocationList {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::Url;
  use identity_did::service::Service;
  use identity_did::service::ServiceEndpoint;

  use super::EmbeddedRevocationService;
  use crate::did::IotaDID;
  use crate::did::IotaDIDUrl;
  use crate::revocation::EmbeddedRevocationList;

  const TAG: &str = "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";
  const SERVICE: &str = "revocation";

  #[test]
  fn test_embedded_service_invariants() {
    let iota_did_url: IotaDIDUrl = IotaDIDUrl::parse(format!("did:iota:{}#{}", TAG, SERVICE)).unwrap();
    let service_endpoint: Url = Url::parse("data:,eJyzMmAAAwADKABr").unwrap();
    let embedded_revocation_service =
      EmbeddedRevocationService::new(iota_did_url.clone(), service_endpoint.clone()).unwrap();

    let service: Service<IotaDID> = Service::builder(Object::new())
      .id(iota_did_url.clone())
      .type_(EmbeddedRevocationList::name())
      .service_endpoint(ServiceEndpoint::One(service_endpoint.clone()))
      .build()
      .unwrap();
    assert_eq!(embedded_revocation_service, service.try_into().unwrap());

    let invalid_service_type: Service<IotaDID> = Service::builder(Object::new())
      .id(iota_did_url.clone())
      .type_("DifferentRevocationType")
      .service_endpoint(ServiceEndpoint::One(service_endpoint.clone()))
      .build()
      .unwrap();
    assert!(EmbeddedRevocationService::try_from(invalid_service_type).is_err());
  }

  #[test]
  fn test_embedded_service_update() {
    let iota_did_url: IotaDIDUrl = IotaDIDUrl::parse(format!("did:iota:{}#{}", TAG, SERVICE)).unwrap();
    let service_endpoint: Url = Url::parse("data:,eJyzMmAAAwADKABr").unwrap();
    let mut embedded_revocation_service =
      EmbeddedRevocationService::new(iota_did_url.clone(), service_endpoint.clone()).unwrap();

    embedded_revocation_service.revoke_credentials(&[0, 5, 6, 8]).unwrap();
    assert_eq!(
      embedded_revocation_service.service_endpoint(),
      &Url::parse("data:,eJyzMmBgYGQAAWYGATDNysDGwMEAAAscAJI").unwrap()
    );
  }
}
