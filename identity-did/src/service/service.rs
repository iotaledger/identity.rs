// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::convert::ToJson;

use crate::did::CoreDIDUrl;
use crate::error::Error;
use crate::error::Result;
use crate::service::ServiceBuilder;
use crate::utils::OrderedSet;

/// A DID Document Service used to enable trusted interactions associated with a DID subject.
///
/// [Specification](https://www.w3.org/TR/did-core/#services)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Service<T = Object> {
  pub(crate) id: CoreDIDUrl,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  #[serde(rename = "serviceEndpoint")]
  pub(crate) service_endpoint: Url,
  #[serde(flatten)]
  pub(crate) properties: T,
}

/// An endpoint or set of endpoints specified in a [`Service`].
///
/// [Specification](https://www.w3.org/TR/did-core/#dfn-serviceendpoint)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServiceEndpoint {
  One(Url),
  Set(OrderedSet<Url>),
  // TODO: Enforce set is non-empty?
  // TODO: Should this support an ordered map and nested maps which the specification allows?
}

impl<T> Service<T> {
  /// Creates a `ServiceBuilder` to configure a new `Service`.
  ///
  /// This is the same as `ServiceBuilder::new()`.
  pub fn builder(properties: T) -> ServiceBuilder<T> {
    ServiceBuilder::new(properties)
  }

  /// Returns a new `Service` based on the `ServiceBuilder` configuration.
  pub fn from_builder(builder: ServiceBuilder<T>) -> Result<Self> {
    Ok(Self {
      id: builder.id.ok_or(Error::BuilderInvalidServiceId)?,
      type_: builder.type_.ok_or(Error::BuilderInvalidServiceType)?,
      service_endpoint: builder.service_endpoint.ok_or(Error::BuilderInvalidServiceEndpoint)?,
      properties: builder.properties,
    })
  }

  /// Returns a reference to the `Service` id.
  pub fn id(&self) -> &CoreDIDUrl {
    &self.id
  }

  /// Returns a mutable reference to the `Service` id.
  pub fn id_mut(&mut self) -> &mut CoreDIDUrl {
    &mut self.id
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
  pub fn service_endpoint(&self) -> &Url {
    &self.service_endpoint
  }

  /// Returns a mutable reference to the `Service` endpoint.
  pub fn service_endpoint_mut(&mut self) -> &mut Url {
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
}

impl<T> AsRef<CoreDIDUrl> for Service<T> {
  fn as_ref(&self) -> &CoreDIDUrl {
    self.id()
  }
}

impl<T> Display for Service<T>
where
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
  }
}

impl Display for ServiceEndpoint {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::{Object, Url};
  use identity_core::convert::{FromJson, ToJson};
  use crate::did::CoreDIDUrl;
  use crate::service::Service;

  use crate::service::service::ServiceEndpoint;
  use crate::utils::OrderedSet;

  // #[test]
  // fn test_service_serde() {
  //   let url1 = Url::parse("https://iota.org/").unwrap();
  //   let url2 = Url::parse("wss://www.example.com/socketserver/").unwrap();
  //   let url3 = Url::parse("did:abc:123#service").unwrap();
  //
  //   // VALID
  //   let mut set: OrderedSet<Url> = OrderedSet::try_from(vec![url1, url2, url3]).unwrap();
  //   let endpoint: ServiceEndpoint = ServiceEndpoint::Set(set);
  //   let service: Service = Service::builder(Object::new())
  //     .id(CoreDIDUrl::parse("did:example:123#service").unwrap())
  //     .service_endpoint(endpoint)
  //     .type_("LinkedDomains")
  //     .build();
  // }

  #[test]
  fn test_service_endpoint_serde() {
    let url1 = Url::parse("https://iota.org/").unwrap();
    let url2 = Url::parse("wss://www.example.com/socketserver/").unwrap();
    let url3 = Url::parse("did:abc:123#service").unwrap();

    // VALID: One.
    let endpoint1: ServiceEndpoint = ServiceEndpoint::One(url1.clone());
    let ser_endpoint1: String = endpoint1.to_json().unwrap();
    assert_eq!(ser_endpoint1, "\"https://iota.org/\"");
    assert_eq!(endpoint1, ServiceEndpoint::from_json(&ser_endpoint1).unwrap());

    let endpoint2: ServiceEndpoint = ServiceEndpoint::One(url2.clone());
    let ser_endpoint2: String = endpoint2.to_json().unwrap();
    assert_eq!(ser_endpoint2, "\"wss://www.example.com/socketserver/\"");
    assert_eq!(endpoint2, ServiceEndpoint::from_json(&ser_endpoint2).unwrap());

    let endpoint3: ServiceEndpoint = ServiceEndpoint::One(url3.clone());
    let ser_endpoint3: String = endpoint3.to_json().unwrap();
    assert_eq!(ser_endpoint3, "\"did:abc:123#service\"");
    assert_eq!(endpoint3, ServiceEndpoint::from_json(&ser_endpoint3).unwrap());

    // VALID: Set.
    let mut set: OrderedSet<Url> = OrderedSet::new();
    // One element.
    assert!(set.append(url1.clone()));
    let endpoint_set: ServiceEndpoint = ServiceEndpoint::Set(set.clone());
    let ser_endpoint_set: String = endpoint_set.to_json().unwrap();
    assert_eq!(ser_endpoint_set, "[\"https://iota.org/\"]");
    assert_eq!(endpoint_set, ServiceEndpoint::from_json(&ser_endpoint_set).unwrap());
    // Two elements.
    assert!(set.append(url2.clone()));
    let endpoint_set: ServiceEndpoint = ServiceEndpoint::Set(set.clone());
    let ser_endpoint_set: String = endpoint_set.to_json().unwrap();
    assert_eq!(ser_endpoint_set, "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\"]");
    assert_eq!(endpoint_set, ServiceEndpoint::from_json(&ser_endpoint_set).unwrap());
    // Three elements.
    assert!(set.append(url3.clone()));
    let endpoint_set: ServiceEndpoint = ServiceEndpoint::Set(set.clone());
    let ser_endpoint_set: String = endpoint_set.to_json().unwrap();
    assert_eq!(ser_endpoint_set, "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\",\"did:abc:123#service\"]");
    assert_eq!(endpoint_set, ServiceEndpoint::from_json(&ser_endpoint_set).unwrap());

    // VALID: Set ignore duplicates.
    let mut duplicates_set: OrderedSet<Url> = OrderedSet::new();
    duplicates_set.append(url1.clone());
    duplicates_set.append(url1.clone());
    assert_eq!(ServiceEndpoint::Set(duplicates_set.clone()).to_json().unwrap(), "[\"https://iota.org/\"]");
    duplicates_set.append(url2.clone());
    duplicates_set.append(url2.clone());
    duplicates_set.append(url1.clone());
    assert_eq!(ServiceEndpoint::Set(duplicates_set.clone()).to_json().unwrap(), "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\"]");
    assert!(duplicates_set.append(url3.clone()));
    duplicates_set.append(url3.clone());
    duplicates_set.append(url1.clone());
    duplicates_set.append(url2.clone());
    assert_eq!(ser_endpoint_set, "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\",\"did:abc:123#service\"]");
  }

  #[test]
  fn test_service_endpoint_serde_fails() {
    // INVALID: empty
    assert!(ServiceEndpoint::from_json("\"\"").is_err());
    assert!(ServiceEndpoint::from_json("").is_err());

    // INVALID: spaces
    assert!(ServiceEndpoint::from_json("\" \"").is_err());
    assert!(ServiceEndpoint::from_json("\"\t\"").is_err());
    assert!(ServiceEndpoint::from_json("\"https:// iota.org/\"").is_err());
    assert!(ServiceEndpoint::from_json("\"https://\tiota.org/\"").is_err());
    assert!(ServiceEndpoint::from_json("[\"https:// iota.org/\",\"wss://www.example.com/socketserver/\"]").is_err());
    assert!(ServiceEndpoint::from_json("[\"https:// iota.org/\",\"wss://www.example.com/socketserver/\"]").is_err());

    // INVALID: wrong map syntax (no keys)
    assert!(ServiceEndpoint::from_json("{\"https:// iota.org/\",\"wss://www.example.com/socketserver/\"}").is_err());
  }
}
