// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_did::did::CoreDID;
use identity_did::did::DIDUrl;
use identity_did::did::DID;

use super::error::Result;
use super::EmbeddedRevocationService;
use crate::revocation::EMBEDDED_REVOCATION_METHOD_NAME;

/// A `ServiceBuilder` is used to generate a customized `Service`.
#[derive(Clone, Debug)]
pub struct EmbeddedServiceBuilder<D = CoreDID>
where
  D: DID,
{
  pub(crate) id: Option<DIDUrl<D>>,
  pub(crate) type_: String,
  pub(crate) service_endpoint: Option<Url>,
}

impl<D> EmbeddedServiceBuilder<D>
where
  D: DID,
{
  /// Creates a new `EmbeddedServiceBuilder`.
  pub fn new() -> Self {
    Self {
      id: None,
      type_: EMBEDDED_REVOCATION_METHOD_NAME.to_owned(),
      service_endpoint: None,
    }
  }

  /// Sets the `id` value of the generated `Service`.
  #[must_use]
  pub fn id(mut self, value: DIDUrl<D>) -> Self {
    self.id = Some(value);
    self
  }

  /// Sets the `serviceEndpoint` value of the generated `EmbeddedServiceBuilder`.
  #[must_use]
  pub fn service_endpoint(mut self, value: Url) -> Self {
    self.service_endpoint = Some(value);
    self
  }

  /// Returns a new `Service` based on the `ServiceBuilder` configuration.
  pub fn build(self) -> Result<EmbeddedRevocationService<D>> {
    EmbeddedRevocationService::from_builder(self)
  }
}
