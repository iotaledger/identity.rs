// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;

use crate::error::Result;
use crate::verification_method::MethodData;
use crate::verification_method::MethodType;
use crate::verification_method::VerificationMethod;
use identity_did::CoreDID;
use identity_did::DIDUrl;

/// A `MethodBuilder` is used to generate a customized `Method`.
#[derive(Clone, Debug, Default)]
pub struct MethodBuilder {
  pub(crate) id: Option<DIDUrl>,
  pub(crate) controller: Option<CoreDID>,
  pub(crate) type_: Option<MethodType>,
  pub(crate) data: Option<MethodData>,
  pub(crate) properties: Object,
}

impl MethodBuilder {
  /// Creates a new `MethodBuilder`.
  pub fn new(properties: Object) -> Self {
    Self {
      id: None,
      controller: None,
      type_: None,
      data: None,
      properties,
    }
  }

  /// Sets the `id` value of the generated `VerificationMethod`.
  #[must_use]
  pub fn id(mut self, value: DIDUrl) -> Self {
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
  pub fn type_(mut self, value: MethodType) -> Self {
    self.type_ = Some(value);
    self
  }

  /// Sets the `data` value of the generated `VerificationMethod`.
  #[must_use]
  pub fn data(mut self, value: MethodData) -> Self {
    self.data = Some(value);
    self
  }

  /// Returns a new `VerificationMethod` based on the `MethodBuilder` configuration.
  pub fn build(self) -> Result<VerificationMethod> {
    VerificationMethod::from_builder(self)
  }
}

#[cfg(test)]
mod tests {
  use crate::Error;

  use super::*;

  #[test]
  fn test_method_builder_success() {
    for method_data_fn in [MethodData::new_base58, MethodData::new_multibase] {
      let result: Result<VerificationMethod> = MethodBuilder::default()
        .id("did:example:123#key".parse().unwrap())
        .controller("did:example:123".parse().unwrap())
        .type_(MethodType::Ed25519VerificationKey2018)
        .data(method_data_fn(""))
        .build();
      assert!(result.is_ok());
    }
  }

  #[test]
  fn test_missing_id_fragment() {
    let result: Result<VerificationMethod> = MethodBuilder::default()
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::PublicKeyMultibase("".into()))
      .build();
    assert!(matches!(result.unwrap_err(), Error::InvalidMethod(_)));
  }

  #[test]
  fn test_missing_id() {
    let result: Result<VerificationMethod> = MethodBuilder::default()
      .controller("did:example:123".parse().unwrap())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::PublicKeyMultibase("".into()))
      .build();
    assert!(matches!(result.unwrap_err(), Error::InvalidMethod(_)));
  }

  #[test]
  fn test_missing_type() {
    let result: Result<VerificationMethod> = MethodBuilder::default()
      .id("did:example:123#key".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .data(MethodData::PublicKeyMultibase("".into()))
      .build();
    assert!(matches!(result.unwrap_err(), Error::InvalidMethod(_)));
  }

  #[test]
  fn test_missing_data() {
    let result: Result<VerificationMethod> = MethodBuilder::default()
      .id("did:example:123#key".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .type_(MethodType::Ed25519VerificationKey2018)
      .build();
    assert!(matches!(result.unwrap_err(), Error::InvalidMethod(_)));
  }

  #[test]
  fn test_missing_controller() {
    let result: Result<VerificationMethod> = MethodBuilder::default()
      .id("did:example:123#key".parse().unwrap())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::PublicKeyMultibase("".into()))
      .build();
    assert!(matches!(result.unwrap_err(), Error::InvalidMethod(_)));
  }
}
