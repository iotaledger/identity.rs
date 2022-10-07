// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_did::document::CoreDocument;
use identity_did::verification::VerificationMethod;

use crate::key_storage;
use crate::KeyStorage;
use crate::MethodContent;
use crate::MethodType1;
use crate::StorageResult;

pub struct CreateMethodBuilder<'builder, K>
where
  K: KeyStorage,
  K::KeyType: TryFrom<MethodType1>,
  <K::KeyType as TryFrom<MethodType1>>::Error: std::error::Error + Send + Sync + 'static,
{
  document: &'builder mut CoreDocument,
  key_storage: Option<&'builder K>,
  content: Option<MethodContent>,
  fragment: Option<String>,
  mapping_strategy: fn(MethodType1) -> StorageResult<K::KeyType>,
}

impl<'builder, K> CreateMethodBuilder<'builder, K>
where
  K: KeyStorage,
  K::KeyType: TryFrom<MethodType1>,
  <K::KeyType as TryFrom<MethodType1>>::Error: std::error::Error + Send + Sync + 'static,
{
  pub fn new(document: &'builder mut CoreDocument) -> Self {
    Self {
      document,
      key_storage: None,
      content: None,
      fragment: None,
      mapping_strategy: TryFrom::<MethodType1>::try_from,
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

  pub fn key_storage(mut self, key_storage: &'builder K) -> Self {
    self.key_storage = Some(key_storage);
    self
  }

  pub async fn apply(self)
  // where
  //   K::KeyType: TryFrom<MethodType1>,
  //   <K::KeyType as TryFrom<MethodType1>>::Error: std::error::Error + Send + Sync + 'static,
  {
    let key_storage = self.key_storage.expect("TODO");
    let (key_alias, key_type) = if let Some(MethodContent::Generate(method_type)) = self.content {
      // TODO: Remove.
      let key_type = match &method_type {
        ty if ty == &MethodType1::ed25519_verification_key_2018() => KeyType::Ed25519,
        ty if ty == &MethodType1::x25519_verification_key_2018() => KeyType::X25519,
        _ => todo!("this will be gone after refactoring VerificationMethod"),
      };

      let ms = self.mapping_strategy.expect("TODO");

      let key_alias = key_storage
        .generate(ms(method_type).expect("TODO"))
        .await
        .expect("TODO");

      (key_alias, key_type)
    } else {
      unimplemented!()
    };

    let public_key = key_storage.public(&key_alias).await.expect("TODO");
    let method = VerificationMethod::new(
      self.document.id().to_owned(),
      key_type,
      &public_key,
      &self.fragment.expect("TODO"),
    )
    .expect("TODO");

    self.document.insert_method(method, Default::default()).expect("TODO");
  }
}
