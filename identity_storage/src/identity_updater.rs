// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::document::CoreDocument;
use identity_did::verification::MethodType;

use crate::create_method::CreateMethodBuilder;
use crate::KeyStorage;

pub struct IdentityUpdater<'updater> {
  pub document: &'updater mut CoreDocument,
}

impl<'updater> IdentityUpdater<'updater> {
  pub fn create_method<K>(&mut self) -> CreateMethodBuilder<'_, K>
  where
    K: KeyStorage,
    K::KeyType: TryFrom<MethodType>,
    <K::KeyType as TryFrom<MethodType>>::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    CreateMethodBuilder::new(self.document)
  }
}
