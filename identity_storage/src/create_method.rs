// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_did::document::CoreDocument;
use identity_did::verification::VerificationMethod;

use crate::KeyStorage;
use crate::MethodContent;
use crate::NewMethodType;

pub struct CreateMethodBuilder<'builder> {
  document: &'builder mut CoreDocument,
  content: Option<MethodContent>,
  fragment: Option<String>,
}

impl<'builder> CreateMethodBuilder<'builder> {
  pub fn new(document: &'builder mut CoreDocument) -> Self {
    Self {
      document,
      content: None,
      fragment: None,
    }
  }

  pub fn content(mut self, content: MethodContent) -> Self {
    self.content = Some(content);
    self
  }

  pub fn fragment(mut self, fragment: &str) -> Self {
    self.fragment = Some(fragment.to_owned());
    self
  }

  pub async fn apply(self, key_storage: &impl KeyStorage) {
    let (key_alias, typ) = if let Some(MethodContent::Generate(typ)) = self.content {
      let typ = match typ {
        ty if ty == NewMethodType::ed25519_verification_key_2018() => KeyType::Ed25519,
        ty if ty == NewMethodType::x25519_verification_key_2018() => KeyType::X25519,
        _ => unimplemented!("unsupported method type"),
      };

      (key_storage.generate(typ).await.expect("TODO"), typ)
    } else {
      unimplemented!("")
    };

    let public_key = key_storage.public(&key_alias).await.expect("TODO");
    let method = VerificationMethod::new(
      self.document.id().to_owned(),
      typ,
      &public_key,
      &self.fragment.expect("TODO"),
    )
    .expect("TODO");

    self.document.insert_method(method, Default::default()).expect("TODO");
  }
}
