// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::Document;
use std::borrow::Cow;

use crate::storage::ResourceType;
use crate::types::MetadataItem;
use crate::types::Identifier;
use crate::types::Timestamps;
use crate::types::Index;
use crate::types::KeyMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityMetadata {
  pub(crate) identifier: Identifier,
  pub(crate) timestamps: Timestamps,
  pub(crate) document_id: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub(crate) latest_auth_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub(crate) latest_diff_id: Option<String>,
  pub(crate) auth: Index,
  #[serde(default, skip_serializing_if = "HashMap::is_empty")]
  pub(crate) diff: HashMap<Index, Index>,
  pub(crate) kmap: KeyMap,
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

  pub fn chain(&self) -> Index {
    self.identifier.index
  }
}

impl MetadataItem for IdentityMetadata {
  const METADATA: ResourceType = ResourceType::IdentityMetadata;
  const RESOURCE: ResourceType = ResourceType::IdentityDocument;

  fn resource(&self) -> &[u8] {
    self.document_id.as_bytes()
  }

  fn identifier(&self) -> &Identifier {
    &self.identifier
  }
}
