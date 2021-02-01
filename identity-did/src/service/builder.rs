// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use did_url::DID;
use identity_core::common::Url;

use crate::error::Result;
use crate::service::Service;

/// A `Builder` is used to generate a customized `Service`.
#[derive(Clone, Debug, Default)]
pub struct Builder<T = ()> {
  pub(crate) id: Option<DID>,
  pub(crate) type_: Option<String>,
  pub(crate) service_endpoint: Option<Url>,
  pub(crate) properties: T,
}

impl<T> Builder<T> {
  /// Creates a new `Builder`.
  pub fn new(properties: T) -> Self {
    Self {
      id: None,
      type_: None,
      service_endpoint: None,
      properties,
    }
  }

  /// Sets the `id` value of the generated `Service`.
  #[must_use]
  pub fn id(mut self, value: DID) -> Self {
    self.id = Some(value);
    self
  }

  /// Sets the `type` value of the generated `Service`.
  #[must_use]
  pub fn type_(mut self, value: impl Into<String>) -> Self {
    self.type_ = Some(value.into());
    self
  }

  /// Sets the `serviceEndpoint` value of the generated `Service`.
  #[must_use]
  pub fn service_endpoint(mut self, value: Url) -> Self {
    self.service_endpoint = Some(value);
    self
  }

  /// Returns a new `Service` based on the `Builder` configuration.
  pub fn build(self) -> Result<Service<T>> {
    Service::from_builder(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic = "InvalidServiceId"]
  fn test_missing_id() {
    let _: Service = Builder::default()
      .type_("ServiceType")
      .service_endpoint("https://example.com".parse().unwrap())
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "InvalidServiceType"]
  fn test_missing_type_() {
    let _: Service = Builder::default()
      .id("did:example:123".parse().unwrap())
      .service_endpoint("https://example.com".parse().unwrap())
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "InvalidServiceEndpoint"]
  fn test_missing_service_endpoint() {
    let _: Service = Builder::default()
      .id("did:example:123".parse().unwrap())
      .type_("ServiceType")
      .build()
      .unwrap();
  }
}
