// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;

use crate::did::CoreDID;
use crate::did::CoreDIDUrl;
use crate::error::Result;
use crate::verification::MethodData;
use crate::verification::MethodType;
use crate::verification::VerificationMethod;

/// A `MethodBuilder` is used to generate a customized `Method`.
#[derive(Clone, Debug, Default)]
pub struct MethodBuilder<T = Object> {
  pub(crate) id: Option<CoreDIDUrl>,
  pub(crate) controller: Option<CoreDID>,
  pub(crate) key_type: Option<MethodType>,
  pub(crate) key_data: Option<MethodData>,
  pub(crate) properties: T,
}

impl<T> MethodBuilder<T> {
  /// Creates a new `MethodBuilder`.
  pub fn new(properties: T) -> Self {
    Self {
      id: None,
      controller: None,
      key_type: None,
      key_data: None,
      properties,
    }
  }

  /// Sets the `id` value of the generated `VerificationMethod`.
  #[must_use]
  pub fn id(mut self, value: CoreDIDUrl) -> Self {
    self.id = Some(value);
    self
  }

  /// Sets the `controller` value of the generated `VerificationMethod`.
  #[must_use]
  pub fn controller(mut self, value: CoreDID) -> Self {
    self.controller = Some(value);
    self
  }

  /// Sets the `type` value of the generated verification `VerificationMethod`.
  #[must_use]
  pub fn key_type(mut self, value: MethodType) -> Self {
    self.key_type = Some(value);
    self
  }

  /// Sets the `data` value of the generated `VerificationMethod`.
  #[must_use]
  pub fn key_data(mut self, value: MethodData) -> Self {
    self.key_data = Some(value);
    self
  }

  /// Returns a new `VerificationMethod` based on the `MethodBuilder` configuration.
  pub fn build(self) -> Result<VerificationMethod<T>> {
    VerificationMethod::from_builder(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_method_builder_success() {
    for method_data_fn in [MethodData::new_b58, MethodData::new_multibase] {
      let result: Result<VerificationMethod> = MethodBuilder::default()
        .id("did:example:123".parse().unwrap())
        .controller("did:example:123".parse().unwrap())
        .key_type(MethodType::Ed25519VerificationKey2018)
        .key_data(method_data_fn(""))
        .build();
      assert!(result.is_ok());
    }
  }

  #[test]
  #[should_panic = "InvalidMethodId"]
  fn test_missing_id() {
    let _: VerificationMethod = MethodBuilder::default()
      .controller("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyMultibase("".into()))
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "InvalidMethodType"]
  fn test_missing_key_type() {
    let _: VerificationMethod = MethodBuilder::default()
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .key_data(MethodData::PublicKeyMultibase("".into()))
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "InvalidMethodData"]
  fn test_missing_key_data() {
    let _: VerificationMethod = MethodBuilder::default()
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "InvalidMethodController"]
  fn test_missing_controller() {
    let _: VerificationMethod = MethodBuilder::default()
      .id("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyMultibase("".into()))
      .build()
      .unwrap();
  }
}
