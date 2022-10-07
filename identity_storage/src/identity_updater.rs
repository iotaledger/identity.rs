// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::document::CoreDocument;

use crate::create_method::CreateMethodBuilder;

pub struct IdentityUpdater<'updater> {
  pub document: &'updater mut CoreDocument,
}

impl<'updater> IdentityUpdater<'updater> {
  pub fn create_method(&mut self) -> CreateMethodBuilder<'_> {
    CreateMethodBuilder::new(self.document)
  }
}
