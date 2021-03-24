// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::Document;
use std::borrow::Cow;

use crate::storage::ResourceType;
use crate::types::MetadataItem;
use crate::types::Identifier;
use crate::types::Timestamps;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityMetadata {
  pub(crate) identifier: Identifier,
  pub(crate) timestamps: Timestamps,
  pub(crate) document_id: String,
  pub(crate) latest_auth_id: Option<Identifier>,
  pub(crate) latest_diff_id: Option<Identifier>,
}

impl IdentityMetadata {
  pub fn new(
    ident: String,
    index: u32,
    document: &Document,
  ) -> Self {
    Self {
      timestamps: Timestamps::now(),
      identifier: Identifier::new(ident, index),
      document_id: document.id().to_string(),
      latest_auth_id: None,
      latest_diff_id: None,
    }
  }

  // pub fn append_document(&mut self) {}

  // pub fn append_diff(&mut self) {}
}

impl MetadataItem for IdentityMetadata {
  const METADATA: ResourceType = ResourceType::IdentityMeta;
  const RESOURCE: ResourceType = ResourceType::Identity;

  fn resource(&self) -> &[u8] {
    self.document_id.as_bytes()
  }

  fn identifier(&self) -> &Identifier {
    &self.identifier
  }
}
