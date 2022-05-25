// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use serde::de;
use serde::Deserialize;
use serde::Serialize;

use identity_core::common::KeyComparable;
use identity_core::common::Object;
use identity_core::convert::FmtJson;

use crate::did::CoreDID;
use crate::did::DIDUrl;
use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::service::ServiceBuilder;
use crate::service::ServiceEndpoint;

/// A DID Document Service used to enable trusted interactions associated with a DID subject.
///
/// [Specification](https://www.w3.org/TR/did-core/#services)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(bound(deserialize = "D: DID + Deserialize<'de>, T: serde::Deserialize<'de>"))]
pub struct Service<D = CoreDID, T = Object>
where
  D: DID,
{
  #[serde(deserialize_with = "deserialize_id_with_fragment")]
  pub(crate) id: DIDUrl<D>,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  #[serde(rename = "serviceEndpoint")]
  pub(crate) service_endpoint: ServiceEndpoint,
  #[serde(flatten)]
  pub(crate) properties: T,
}

/// Deserializes an [`DIDUrl`] while enforcing that its fragment is non-empty.
pub fn deserialize_id_with_fragment<'de, D, T>(deserializer: D) -> Result<DIDUrl<T>, D::Error>
where
  D: de::Deserializer<'de>,
  T: DID + serde::Deserialize<'de>,
{
  let did_url: DIDUrl<T> = DIDUrl::deserialize(deserializer)?;
  if did_url.fragment().unwrap_or_default().is_empty() {
    return Err(de::Error::custom(Error::InvalidService("empty id fragment")));
  }
  Ok(did_url)
}

impl<D, T> Service<D, T>
where
  D: DID,
{
  /// Creates a `ServiceBuilder` to configure a new `Service`.
  ///
  /// This is the same as `ServiceBuilder::new()`.
  pub fn builder(properties: T) -> ServiceBuilder<D, T> {
    ServiceBuilder::new(properties)
  }

  /// Returns a new `Service` based on the `ServiceBuilder` configuration.
  pub fn from_builder(builder: ServiceBuilder<D, T>) -> Result<Self> {
    let id: DIDUrl<D> = builder.id.ok_or(Error::InvalidService("missing id"))?;
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(Error::InvalidService("empty id fragment"));
    }

    Ok(Self {
      id,
      type_: builder.type_.ok_or(Error::InvalidService("missing type"))?,
      service_endpoint: builder
        .service_endpoint
        .ok_or(Error::InvalidService("missing endpoint"))?,
      properties: builder.properties,
    })
  }

  /// Returns a reference to the `Service` id.
  pub fn id(&self) -> &DIDUrl<D> {
    &self.id
  }

  /// Sets the `Service` id.
  ///
  /// # Errors
  /// [`Error::InvalidMethodFragment`] if there is no fragment on the [`DIDUrl`].
  pub fn set_id(&mut self, id: DIDUrl<D>) -> Result<()> {
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(Error::MissingIdFragment);
    }
    self.id = id;
    Ok(())
  }

  /// Returns a reference to the `Service` type.
  pub fn type_(&self) -> &str {
    &*self.type_
  }

  /// Returns a mutable reference to the `Service` type.
  pub fn type_mut(&mut self) -> &mut String {
    &mut self.type_
  }

  /// Returns a reference to the `Service` endpoint.
  pub fn service_endpoint(&self) -> &ServiceEndpoint {
    &self.service_endpoint
  }

  /// Returns a mutable reference to the `Service` endpoint.
  pub fn service_endpoint_mut(&mut self) -> &mut ServiceEndpoint {
    &mut self.service_endpoint
  }

  /// Returns a reference to the custom `Service` properties.
  pub fn properties(&self) -> &T {
    &self.properties
  }

  /// Returns a mutable reference to the custom `Service` properties.
  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }

  /// Maps `Service<D,T>` to `Service<C,T>` by applying a function `f` to
  /// the id.
  pub fn map<C, F>(self, f: F) -> Service<C, T>
  where
    C: DID,
    F: FnMut(D) -> C,
  {
    Service {
      id: self.id.map(f),
      type_: self.type_,
      service_endpoint: self.service_endpoint,
      properties: self.properties,
    }
  }

  /// Fallible version of [`Service::map`].
  pub fn try_map<C, F, E>(self, f: F) -> Result<Service<C, T>, E>
  where
    C: DID,
    F: FnMut(D) -> Result<C, E>,
  {
    Ok(Service {
      id: self.id.try_map(f)?,
      type_: self.type_,
      service_endpoint: self.service_endpoint,
      properties: self.properties,
    })
  }
}

impl<D, T> AsRef<DIDUrl<D>> for Service<D, T>
where
  D: DID,
{
  fn as_ref(&self) -> &DIDUrl<D> {
    self.id()
  }
}

impl<D, T> Display for Service<D, T>
where
  D: DID + Serialize,
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl<D, T> KeyComparable for Service<D, T>
where
  D: DID,
{
  type Key = DIDUrl<D>;

  #[inline]
  fn key(&self) -> &Self::Key {
    self.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::did::CoreDIDUrl;
  use identity_core::common::OrderedSet;
  use identity_core::common::Url;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;

  #[test]
  fn test_service_serde() {
    // Single endpoint
    {
      let service: Service = Service::builder(Object::new())
        .id(CoreDIDUrl::parse("did:example:123#service").unwrap())
        .type_("LinkedDomains".to_owned())
        .service_endpoint(Url::parse("https://iota.org/").unwrap().into())
        .build()
        .unwrap();
      let expected = r#"{"id":"did:example:123#service","type":"LinkedDomains","serviceEndpoint":"https://iota.org/"}"#;
      assert_eq!(service.to_json().unwrap(), expected);
      assert_eq!(Service::from_json(expected).unwrap(), service);
    }

    // Set of endpoints
    {
      let endpoint: ServiceEndpoint = ServiceEndpoint::Set(
        OrderedSet::try_from(vec![
          Url::parse("https://iota.org/").unwrap(),
          Url::parse("wss://www.example.com/socketserver/").unwrap(),
          Url::parse("did:abc:123#service").unwrap(),
        ])
        .unwrap(),
      );
      let service: Service = Service::builder(Object::new())
        .id(CoreDIDUrl::parse("did:example:123#service").unwrap())
        .type_("LinkedDomains".to_owned())
        .service_endpoint(endpoint)
        .build()
        .unwrap();
      let expected = r#"{"id":"did:example:123#service","type":"LinkedDomains","serviceEndpoint":["https://iota.org/","wss://www.example.com/socketserver/","did:abc:123#service"]}"#;
      assert_eq!(service.to_json().unwrap(), expected);
      assert_eq!(Service::from_json(expected).unwrap(), service)
    }
  }

  #[test]
  fn test_service_deserialize_empty_fragment_fails() {
    let empty_id_fragment: &str =
      r#"{"id":"did:example:123","type":"LinkedDomains","serviceEndpoint":"https://iota.org/"}"#;
    let result: Result<Service, identity_core::Error> = Service::from_json(empty_id_fragment);
    assert!(result.is_err());
  }
}
