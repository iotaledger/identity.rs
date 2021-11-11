// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use serde::Serialize;

use identity_core::common::Object;
use identity_core::convert::ToJson;

use crate::did::CoreDIDUrl;
use crate::error::Error;
use crate::error::Result;
use crate::service::ServiceBuilder;
use crate::service::ServiceEndpoint;

/// A DID Document Service used to enable trusted interactions associated with a DID subject.
///
/// [Specification](https://www.w3.org/TR/did-core/#services)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Service<T = Object> {
  pub(crate) id: CoreDIDUrl,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  #[serde(rename = "serviceEndpoint")]
  pub(crate) service_endpoint: ServiceEndpoint,
  #[serde(flatten)]
  pub(crate) properties: T,
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

#[cfg(test)]
mod tests {
  use crate::did::CoreDIDUrl;
  use crate::service::Service;
  use identity_core::common::Object;
  use identity_core::common::Url;

  use crate::service::service::ServiceEndpoint;
  use crate::utils::OrderedSet;
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
}
