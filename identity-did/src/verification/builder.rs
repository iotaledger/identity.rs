// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use did_url::DID;

use crate::error::Result;
use crate::verification::Method;
use crate::verification::MethodData;
use crate::verification::MethodType;

/// A `Builder` is used to generate a customized `Method`.
#[derive(Clone, Debug, Default)]
pub struct Builder<T = ()> {
  pub(crate) id: Option<DID>,
  pub(crate) controller: Option<DID>,
  pub(crate) key_type: Option<MethodType>,
  pub(crate) key_data: Option<MethodData>,
  pub(crate) properties: T,
}

impl<T> Builder<T> {
  /// Creates a new `Builder`.
  pub fn new(properties: T) -> Self {
    Self {
      id: None,
      controller: None,
      key_type: None,
      key_data: None,
      properties,
    }
  }

  /// Sets the `id` value of the generated verification `Method`.
  #[must_use]
  pub fn id(mut self, value: DID) -> Self {
    self.id = Some(value);
    self
  }

  /// Sets the `controller` value of the generated verification `Method`.
  #[must_use]
  pub fn controller(mut self, value: DID) -> Self {
    self.controller = Some(value);
    self
  }

  /// Sets the `type` value of the generated verification `Method`.
  #[must_use]
  pub fn key_type(mut self, value: MethodType) -> Self {
    self.key_type = Some(value);
    self
  }

  /// Sets the `data` value of the generated verification `Method`.
  #[must_use]
  pub fn key_data(mut self, value: MethodData) -> Self {
    self.key_data = Some(value);
    self
  }

  /// Returns a new `Method` based on the `Builder` configuration.
  pub fn build(self) -> Result<Method<T>> {
    Method::from_builder(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic = "InvalidMethodId"]
  fn test_missing_id() {
    let _: Method = Builder::default()
      .controller("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "InvalidMethodType"]
  fn test_missing_key_type() {
    let _: Method = Builder::default()
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "InvalidMethodData"]
  fn test_missing_key_data() {
    let _: Method = Builder::default()
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "InvalidMethodController"]
  fn test_missing_controller() {
    let _: Method = Builder::default()
      .id("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap();
  }
}
