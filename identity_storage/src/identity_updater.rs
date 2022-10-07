// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::document::CoreDocument;

use crate::create_method::CreateMethodBuilder;
use crate::KeyStorage;
use crate::MethodType1;

pub struct IdentityUpdater<'updater> {
  pub document: &'updater mut CoreDocument,
}

impl<'updater> IdentityUpdater<'updater> {
  pub fn create_method<K>(&mut self) -> CreateMethodBuilder<'_, K>
  where
    K: KeyStorage,
    K::KeyType: TryFrom<MethodType1>,
    <K::KeyType as TryFrom<MethodType1>>::Error: std::error::Error + Send + Sync + 'static,
  {
    CreateMethodBuilder::new(self.document)
  }
}
