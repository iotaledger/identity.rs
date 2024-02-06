// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Url;

use crate::document::CoreDocument;
use crate::error::Result;
use crate::service::Service;
use identity_did::CoreDID;
use identity_verification::MethodRef;
use identity_verification::VerificationMethod;

/// A `DocumentBuilder` is used to generate a customized [`Document`](crate::document::CoreDocument).
#[derive(Clone, Debug)]
pub struct DocumentBuilder {
  pub(crate) id: Option<CoreDID>,
  pub(crate) controller: Vec<CoreDID>,
  pub(crate) also_known_as: Vec<Url>,
  pub(crate) verification_method: Vec<VerificationMethod>,
  pub(crate) authentication: Vec<MethodRef>,
  pub(crate) assertion_method: Vec<MethodRef>,
  pub(crate) key_agreement: Vec<MethodRef>,
  pub(crate) capability_delegation: Vec<MethodRef>,
  pub(crate) capability_invocation: Vec<MethodRef>,
  pub(crate) service: Vec<Service>,
  pub(crate) properties: Object,
}

impl DocumentBuilder {
  /// Creates a new `DocumentBuilder`.
  pub fn new(properties: Object) -> Self {
    Self {
      id: None,
      controller: Vec::new(),
      also_known_as: Vec::new(),
      verification_method: Vec::new(),
      authentication: Vec::new(),
      assertion_method: Vec::new(),
      key_agreement: Vec::new(),
      capability_delegation: Vec::new(),
      capability_invocation: Vec::new(),
      service: Vec::new(),
      properties,
    }
  }

  /// Sets the `id` value.
  #[must_use]
  pub fn id(mut self, value: CoreDID) -> Self {
    self.id = Some(value);
    self
  }

  /// Adds a value to the `controller` set.
  #[must_use]
  pub fn controller(mut self, value: CoreDID) -> Self {
    self.controller.push(value);
    self
  }

  /// Adds a value to the `alsoKnownAs` set.
  #[must_use]
  pub fn also_known_as(mut self, value: Url) -> Self {
    self.also_known_as.push(value);
    self
  }

  /// Adds a value to the `verificationMethod` set.
  #[must_use]
  pub fn verification_method(mut self, value: VerificationMethod) -> Self {
    self.verification_method.push(value);
    self
  }

  /// Adds a value to the `authentication` set.
  #[must_use]
  pub fn authentication(mut self, value: impl Into<MethodRef>) -> Self {
    self.authentication.push(value.into());
    self
  }

  /// Adds a value to the `assertionMethod` set.
  #[must_use]
  pub fn assertion_method(mut self, value: impl Into<MethodRef>) -> Self {
    self.assertion_method.push(value.into());
    self
  }

  /// Adds a value to the `keyAgreement` set.
  #[must_use]
  pub fn key_agreement(mut self, value: impl Into<MethodRef>) -> Self {
    self.key_agreement.push(value.into());
    self
  }

  /// Adds a value to the `capabilityDelegation` set.
  #[must_use]
  pub fn capability_delegation(mut self, value: impl Into<MethodRef>) -> Self {
    self.capability_delegation.push(value.into());
    self
  }

  /// Adds a value to the `capabilityInvocation` set.
  #[must_use]
  pub fn capability_invocation(mut self, value: impl Into<MethodRef>) -> Self {
    self.capability_invocation.push(value.into());
    self
  }

  /// Adds a value to the `service` set.
  #[must_use]
  pub fn service(mut self, value: Service) -> Self {
    self.service.push(value);
    self
  }

  /// Returns a new `Document` based on the `DocumentBuilder` configuration.
  pub fn build(self) -> Result<CoreDocument> {
    CoreDocument::from_builder(self)
  }
}

impl Default for DocumentBuilder {
  fn default() -> Self {
    Self::new(Object::default())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Error;
  use identity_did::DID;
  use identity_verification::MethodData;
  use identity_verification::MethodType;

  #[test]
  fn test_missing_id() {
    let result: Result<CoreDocument> = DocumentBuilder::default().build();
    assert!(matches!(result.unwrap_err(), Error::InvalidDocument(_, None)));
  }

  #[test]
  fn duplicate_id_different_scopes() {
    let did: CoreDID = "did:example:1234".parse().unwrap();
    let fragment = "#key1";
    let id = did.to_url().join(fragment).unwrap();

    let method1: VerificationMethod = VerificationMethod::builder(Default::default())
      .id(id.clone())
      .controller(did.clone())
      .type_(MethodType::ED25519_VERIFICATION_KEY_2018)
      .data(MethodData::PublicKeyBase58(
        "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J".into(),
      ))
      .build()
      .unwrap();

    let method2: VerificationMethod = VerificationMethod::builder(Default::default())
      .id(id)
      .controller(did.clone())
      .type_(MethodType::X25519_KEY_AGREEMENT_KEY_2019)
      .data(MethodData::PublicKeyBase58(
        "FbQWLPRhTH95MCkQUeFYdiSoQt8zMwetqfWoxqPgaq7x".into(),
      ))
      .build()
      .unwrap();

    let result: Result<CoreDocument> = DocumentBuilder::default()
      .id(did)
      .verification_method(method1)
      .key_agreement(method2)
      .build();
    assert!(result.is_err());
  }
}
