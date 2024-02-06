// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use serde::de;
use serde::Deserialize;
use serde::Serialize;

use identity_core::common::KeyComparable;
use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::convert::FmtJson;

use crate::error::Error;
use crate::error::Result;
use crate::service::ServiceBuilder;
use crate::service::ServiceEndpoint;
use identity_did::CoreDID;
use identity_did::DIDUrl;

/// A DID Document Service used to enable trusted interactions associated with a DID subject.
///
/// [Specification](https://www.w3.org/TR/did-core/#services)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Service {
  #[serde(deserialize_with = "deserialize_id_with_fragment")]
  pub(crate) id: DIDUrl,
  #[serde(rename = "type")]
  pub(crate) type_: OneOrSet<String>,
  #[serde(rename = "serviceEndpoint")]
  pub(crate) service_endpoint: ServiceEndpoint,
  #[serde(flatten)]
  pub(crate) properties: Object,
}

/// Deserializes an [`DIDUrl`] while enforcing that its fragment is non-empty.
fn deserialize_id_with_fragment<'de, D>(deserializer: D) -> Result<DIDUrl, D::Error>
where
  D: de::Deserializer<'de>,
{
  let did_url: DIDUrl = DIDUrl::deserialize(deserializer)?;
  if did_url.fragment().unwrap_or_default().is_empty() {
    return Err(de::Error::custom(Error::InvalidService("empty id fragment")));
  }
  Ok(did_url)
}

impl Service {
  /// Creates a `ServiceBuilder` to configure a new `Service`.
  ///
  /// This is the same as `ServiceBuilder::new()`.
  pub fn builder(properties: Object) -> ServiceBuilder {
    ServiceBuilder::new(properties)
  }

  /// Returns a new `Service` based on the `ServiceBuilder` configuration.
  pub fn from_builder(builder: ServiceBuilder) -> Result<Self> {
    let id: DIDUrl = builder.id.ok_or(Error::InvalidService("missing id"))?;
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(Error::InvalidService("empty id fragment"));
    }

    Ok(Self {
      id,
      type_: OneOrSet::try_from(builder.type_).map_err(|_| Error::InvalidService("invalid type"))?,
      service_endpoint: builder
        .service_endpoint
        .ok_or(Error::InvalidService("missing endpoint"))?,
      properties: builder.properties,
    })
  }

  /// Returns a reference to the `Service` id.
  pub fn id(&self) -> &DIDUrl {
    &self.id
  }

  /// Sets the `Service`'s id.
  ///
  /// # Errors
  ///
  /// [`Error::MissingIdFragment`] if there is no fragment on the [`DIDUrl`].
  pub fn set_id(&mut self, id: DIDUrl) -> Result<()> {
    if id.fragment().unwrap_or_default().is_empty() {
      return Err(Error::MissingIdFragment);
    }
    self.id = id;
    Ok(())
  }

  /// Returns a reference to the `Service` type.
  pub fn type_(&self) -> &OneOrSet<String> {
    &self.type_
  }

  /// Returns a mutable reference to the `Service` type.
  pub fn type_mut(&mut self) -> &mut OneOrSet<String> {
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
  pub fn properties(&self) -> &Object {
    &self.properties
  }

  /// Returns a mutable reference to the custom `Service` properties.
  pub fn properties_mut(&mut self) -> &mut Object {
    &mut self.properties
  }

  /// Maps `Service` by applying a function `f` to
  /// the id. This is useful when working with DID methods
  /// where the DID is not known prior to publishing.  
  pub fn map<F>(self, f: F) -> Service
  where
    F: FnMut(CoreDID) -> CoreDID,
  {
    Service {
      id: self.id.map(f),
      type_: self.type_,
      service_endpoint: self.service_endpoint,
      properties: self.properties,
    }
  }

  /// Fallible version of [`Service::map`].
  pub fn try_map<F, E>(self, f: F) -> Result<Service, E>
  where
    F: FnMut(CoreDID) -> Result<CoreDID, E>,
  {
    Ok(Service {
      id: self.id.try_map(f)?,
      type_: self.type_,
      service_endpoint: self.service_endpoint,
      properties: self.properties,
    })
  }
}

impl AsRef<DIDUrl> for Service {
  fn as_ref(&self) -> &DIDUrl {
    self.id()
  }
}

impl Display for Service {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl KeyComparable for Service {
  type Key = DIDUrl;

  #[inline]
  fn key(&self) -> &Self::Key {
    self.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use identity_core::common::OrderedSet;
  use identity_core::common::Url;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;

  #[test]
  fn test_service_types_serde() {
    // Single type.
    let service1: Service = Service::builder(Object::new())
      .id(DIDUrl::parse("did:example:123#service").unwrap())
      .type_("LinkedDomains")
      .service_endpoint(Url::parse("https://iota.org/").unwrap())
      .build()
      .unwrap();
    assert_eq!(service1.type_, OneOrSet::new_one("LinkedDomains".to_owned()));
    assert!(service1.type_.contains("LinkedDomains"));
    assert!(!service1.type_.contains("NotEntry"));
    assert!(!service1.type_.contains(""));

    let expected1 = r#"{"id":"did:example:123#service","type":"LinkedDomains","serviceEndpoint":"https://iota.org/"}"#;
    assert_eq!(service1.to_json().unwrap(), expected1);
    assert_eq!(Service::from_json(expected1).unwrap(), service1);

    // Set of types.
    let service2: Service = Service::builder(Object::new())
      .id(DIDUrl::parse("did:example:123#service").unwrap())
      .types(["LinkedDomains".to_owned(), "OtherService2022".to_owned()])
      .service_endpoint(Url::parse("https://iota.org/").unwrap())
      .build()
      .unwrap();
    assert_eq!(
      service2.type_,
      OneOrSet::try_from(vec!["LinkedDomains".to_owned(), "OtherService2022".to_owned()]).unwrap()
    );
    assert!(service2.type_.contains("LinkedDomains"));
    assert!(service2.type_.contains("OtherService2022"));
    assert!(!service2.type_.contains("NotEntry"));
    assert!(!service2.type_.contains(""));

    let expected2 = r#"{"id":"did:example:123#service","type":["LinkedDomains","OtherService2022"],"serviceEndpoint":"https://iota.org/"}"#;
    assert_eq!(service2.to_json().unwrap(), expected2);
    assert_eq!(Service::from_json(expected2).unwrap(), service2)
  }

  #[test]
  fn test_service_endpoint_serde() {
    // Single endpoint.
    {
      let service: Service = Service::builder(Object::new())
        .id(DIDUrl::parse("did:example:123#service").unwrap())
        .type_("LinkedDomains")
        .service_endpoint(Url::parse("https://iota.org/").unwrap())
        .build()
        .unwrap();
      let expected = r#"{"id":"did:example:123#service","type":"LinkedDomains","serviceEndpoint":"https://iota.org/"}"#;
      assert_eq!(service.to_json().unwrap(), expected);
      assert_eq!(Service::from_json(expected).unwrap(), service);
    }

    // Set of endpoints.
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
        .id(DIDUrl::parse("did:example:123#service").unwrap())
        .type_("LinkedDomains")
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
