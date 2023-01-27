// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;

use crate::error::Result;
use crate::service::Service;
use crate::service::ServiceEndpoint;
use identity_did::DIDUrl;

/// A `ServiceBuilder` is used to generate a customized `Service`.
#[derive(Clone, Debug, Default)]
pub struct ServiceBuilder {
  pub(crate) id: Option<DIDUrl>,
  pub(crate) type_: Vec<String>,
  pub(crate) service_endpoint: Option<ServiceEndpoint>,
  pub(crate) properties: Object,
}

impl ServiceBuilder {
  /// Creates a new `ServiceBuilder`.
  pub fn new(properties: Object) -> Self {
    Self {
      id: None,
      type_: Vec::new(),
      service_endpoint: None,
      properties,
    }
  }

  /// Sets the `id` value of the generated `Service`.
  #[must_use]
  pub fn id(mut self, value: DIDUrl) -> Self {
    self.id = Some(value);
    self
  }

  /// Appends a `type` value to the generated `Service`.
  #[must_use]
  pub fn type_(mut self, value: impl Into<String>) -> Self {
    self.type_.push(value.into());
    self
  }

  /// Sets the `type` value of the generated `Service`, overwriting any previous `type` entries.
  ///
  /// NOTE: the entries must be unique.
  #[must_use]
  pub fn types(mut self, values: impl Into<Vec<String>>) -> Self {
    self.type_ = values.into();
    self
  }

  /// Sets the `serviceEndpoint` value of the generated `Service`.
  #[must_use]
  pub fn service_endpoint(mut self, value: impl Into<ServiceEndpoint>) -> Self {
    self.service_endpoint = Some(value.into());
    self
  }

  /// Returns a new `Service` based on the `ServiceBuilder` configuration.
  pub fn build(self) -> Result<Service> {
    Service::from_builder(self)
  }
}

#[cfg(test)]
mod tests {
  use crate::Error;
  use identity_core::common::Url;

  use super::*;

  #[test]
  fn test_success() {
    let _: Service = ServiceBuilder::default()
      .id("did:example:123#service".parse().unwrap())
      .type_("ServiceType")
      .service_endpoint(Url::parse("https://example.com").unwrap())
      .build()
      .unwrap();
  }

  #[test]
  fn test_missing_id() {
    let result: Result<Service> = ServiceBuilder::default()
      .type_("ServiceType")
      .service_endpoint(Url::parse("https://example.com").unwrap())
      .build();
    assert!(matches!(result.unwrap_err(), Error::InvalidService(_)));
  }

  #[test]
  fn test_missing_id_fragment() {
    let result: Result<Service> = ServiceBuilder::default()
      .id("did:example:123".parse().unwrap())
      .type_("ServiceType")
      .service_endpoint(Url::parse("https://example.com").unwrap())
      .build();
    assert!(matches!(result.unwrap_err(), Error::InvalidService(_)));
  }

  #[test]
  fn test_missing_type_() {
    let result: Result<Service> = ServiceBuilder::default()
      .id("did:example:123#service".parse().unwrap())
      .service_endpoint(Url::parse("https://example.com").unwrap())
      .build();
    assert!(matches!(result.unwrap_err(), Error::InvalidService(_)));
  }

  #[test]
  fn test_missing_service_endpoint() {
    let result: Result<Service> = ServiceBuilder::default()
      .id("did:example:123#service".parse().unwrap())
      .type_("ServiceType")
      .build();
    assert!(matches!(result.unwrap_err(), Error::InvalidService(_)));
  }
}
