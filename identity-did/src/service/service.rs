// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::convert::ToJson;
use serde::Serialize;

use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::service::ServiceBuilder;

/// A DID Document Service used to enable trusted interactions associated with a DID subject.
///
/// [Specification](https://www.w3.org/TR/did-core/#services)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Service<T = Object> {
  pub(crate) id: DID,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  #[serde(rename = "serviceEndpoint")]
  pub(crate) service_endpoint: Url,
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
  pub fn id(&self) -> &DID {
    &self.id
  }

  /// Returns a mutable reference to the `Service` id.
  pub fn id_mut(&mut self) -> &mut DID {
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

impl<T> AsRef<DID> for Service<T> {
  fn as_ref(&self) -> &DID {
    self.id()
  }
}
