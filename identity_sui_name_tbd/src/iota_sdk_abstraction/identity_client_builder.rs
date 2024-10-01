// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file has been moved here from identity_iota_core/src/client_dummy.
// The file will be removed after the TS-Client-SDK is integrated.
// The file provides a POC for the wasm-bindgen glue code needed to
// implement the TS-Client-SDK integration.

use super::Error;
use super::IdentityClient;
use super::IotaClientTrait;
use super::types::base_types::{ObjectID};

pub struct IdentityClientBuilder<T: IotaClientTrait> {
  pub(crate) identity_iota_package_id: Option<ObjectID>,
  pub(crate) sender_public_key: Option<Vec<u8>>,
  pub(crate) iota_client: Option<T>,
  pub(crate) network_name: Option<String>,
}

impl<T> IdentityClientBuilder<T>
where
  T: IotaClientTrait<Error = Error>,
{
  /// Sets the `identity_iota_package_id` value.
  #[must_use]
  pub fn identity_iota_package_id(mut self, value: ObjectID) -> Self {
    self.identity_iota_package_id = Some(value);
    self
  }

  /// Sets the `sender_public_key` value.
  #[must_use]
  pub fn sender_public_key(mut self, value: &[u8]) -> Self {
    self.sender_public_key = Some(value.into());
    self
  }

  /// Sets the `iota_client` value.
  #[must_use]
  pub fn iota_client(mut self, value: T) -> Self {
    self.iota_client = Some(value);
    self
  }

  /// Sets the `network_name` value.
  #[must_use]
  pub fn network_name(mut self, value: &str) -> Self {
    self.network_name = Some(value.to_string());
    self
  }

  /// Returns a new `KinesisIdentityClientDummyBuilder` based on the `KinesisIdentityClientDummyBuilder` configuration.
  pub fn build(self) -> Result<IdentityClient<T>, Error> {
    IdentityClient::from_builder(self)
  }
}

impl<T> Default for IdentityClientBuilder<T>
where
  T: IotaClientTrait,
{
  fn default() -> Self {
    Self {
      identity_iota_package_id: None,
      sender_public_key: None,
      iota_client: None,
      network_name: None,
    }
  }
}
