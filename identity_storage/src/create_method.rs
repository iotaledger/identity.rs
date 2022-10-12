// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;

use crate::KeyStorage;
use crate::MethodContent;
use crate::MethodSuite;
use crate::MethodType1;

pub struct CreateMethodBuilder<'builder, K>
where
  K: KeyStorage,
{
  document: &'builder mut CoreDocument,
  method_suite: Option<&'builder MethodSuite<K>>,
  content: Option<MethodContent>,
  typ: Option<MethodType1>,
  fragment: Option<String>,
}

impl<'builder, K> CreateMethodBuilder<'builder, K>
where
  K: KeyStorage,
{
  pub fn new(document: &'builder mut CoreDocument) -> Self {
    Self {
      document,
      method_suite: None,
      content: None,
      fragment: None,
      typ: None,
    }
  }

  pub fn content(mut self, content: MethodContent) -> Self {
    self.content = Some(content);
    self
  }

  pub fn type_(mut self, typ: MethodType1) -> Self {
    self.typ = Some(typ);
    self
  }

  pub fn fragment(mut self, fragment: &str) -> Self {
    self.fragment = Some(fragment.to_owned());
    self
  }

  pub fn method_suite(mut self, method_suite: &'builder MethodSuite<K>) -> Self {
    self.method_suite = Some(method_suite);
    self
  }

  pub async fn apply(self) {
    let method_suite = self.method_suite.expect("TODO");
    let method_type = self.typ.expect("TODO");
    let method_content = self.content.expect("TODO");

    // TODO: This will be gone after refactoring VerificationMethod to use MethodType1.
    let legacy_method_type = match &method_type {
      ty if ty == &MethodType1::ed25519_verification_key_2018() => MethodType::Ed25519VerificationKey2018,
      ty if ty == &MethodType1::x25519_verification_key_2018() => MethodType::X25519KeyAgreementKey2019,
      _ => todo!("legacy"),
    };

    // TODO: Store key_alias mapping to method id.
    // TODO: Allow user or suite to also set method custom properties (?)
    let (_key_alias, method_data) = method_suite.create(&method_type, method_content).await;

    let method = VerificationMethod::builder(Default::default())
      .id(
        self
          .document
          .id()
          .to_owned()
          .join(self.fragment.expect("TODO"))
          .expect("TODO"),
      )
      .controller(self.document.id().to_owned())
      .data(method_data)
      .type_(legacy_method_type)
      .build()
      .expect("TODO");

    self.document.insert_method(method, Default::default()).expect("TODO");
  }
}
