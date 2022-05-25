// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use identity_core::common::KeyComparable;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use identity_did::did::CoreDID;
use identity_did::did::DIDUrl;
use identity_did::did::DID;
use identity_did::service::deserialize_id_with_fragment;
use identity_did::service::Service;
use identity_did::service::ServiceEndpoint;
use serde::Deserialize;
use serde::Serialize;

use super::error::Result;
use super::error::ServiceError;
use super::EmbeddedServiceBuilder;
use crate::revocation::EmbeddedRevocationList;

/// A DID Document Service used to enable validators to check the status of a credential.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(bound(deserialize = "D: DID + Deserialize<'de>"))]
pub struct EmbeddedRevocationService<D = CoreDID>
where
  D: DID,
{
  #[serde(deserialize_with = "deserialize_id_with_fragment")]
  pub(crate) id: DIDUrl<D>,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  #[serde(rename = "serviceEndpoint")]
  pub(crate) service_endpoint: Url,
}

impl<D> EmbeddedRevocationService<D>
where
  D: DID,
{
  pub fn new(id: DIDUrl<D>, service_endpoint: Url) -> Result<Self> {
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(ServiceError::InvalidServiceId("missing id fragment".to_owned()));
    }
    Ok(Self {
      id,
      type_: EmbeddedRevocationList::name().to_owned(),
      service_endpoint,
    })
  }

  /// Creates a `EmbeddedServiceBuilder` to configure a new `EmbeddedRevocationService`.
  pub fn builder() -> EmbeddedServiceBuilder<D> {
    EmbeddedServiceBuilder::new()
  }

  /// Returns a new `Service` based on the `ServiceBuilder` configuration.
  pub fn from_builder(builder: EmbeddedServiceBuilder<D>) -> Result<Self> {
    let id: DIDUrl<D> = builder
      .id
      .ok_or_else(|| ServiceError::InvalidServiceId("missing id property".to_owned()))?;
    let _ = id
      .fragment()
      .ok_or_else(|| ServiceError::InvalidServiceId("missing id fragment".to_owned()))?;
    let service_endpoint: Url = builder
      .service_endpoint
      .ok_or_else(|| ServiceError::InvalidServiceEndpoint("missing url".to_owned()))?;
    Ok(Self {
      id,
      type_: builder.type_,
      service_endpoint,
    })
  }
  /// Returns a reference to the `EmbeddedRevocationService` id.
  pub fn id(&self) -> &DIDUrl<D> {
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
  pub fn set_id(&mut self, id: DIDUrl<D>) -> Result<()> {
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(ServiceError::InvalidServiceId("missing id fragment".to_owned()));
    }
    self.id = id;
    Ok(())
  }

  /// Sets the `EmbeddedRevocationService` service endpoint.
  ///
  /// # Errors
  /// [`Error::`] if there is no fragment on the [`DIDUrl`].
  pub fn set_service_endpoint(&mut self, service_enpoint: impl AsRef<str>) -> Result<()> {
    let url: Url =
      Url::parse(service_enpoint).map_err(|_e| ServiceError::InvalidServiceEndpoint("invalid url".to_owned()))?;
    self.service_endpoint = url;
    Ok(())
  }
}

impl TryFrom<Service> for EmbeddedRevocationService {
  type Error = ServiceError;

  fn try_from(service: Service) -> Result<Self> {
    if service.type_() != EmbeddedRevocationList::name() {
      return Err(ServiceError::InvalidServiceType(format!(
        "expected {} type",
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

impl TryFrom<EmbeddedRevocationService> for Service {
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

impl<D> AsRef<DIDUrl<D>> for EmbeddedRevocationService<D>
where
  D: DID,
{
  fn as_ref(&self) -> &DIDUrl<D> {
    self.id()
  }
}

impl<D> Display for EmbeddedRevocationService<D>
where
  D: DID + Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl<D> KeyComparable for EmbeddedRevocationService<D>
where
  D: DID,
{
  type Key = DIDUrl<D>;

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
