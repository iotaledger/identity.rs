// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::storage::ResourceType;
use crate::storage::ResourceId;
use crate::types::Identifier;
use crate::types::Key;

pub trait MetadataItem {
  const METADATA: ResourceType;
  const RESOURCE: ResourceType;

  fn identifier(&self) -> &Identifier;

  fn resource(&self) -> &[u8];

  fn resource_id(&self) -> ResourceId<'_> {
    ResourceId::new(Self::RESOURCE, self.resource())
  }

  fn metadata_id(&self) -> ResourceId<'_> {
    ResourceId::new(Self::METADATA, self.resource())
  }

  fn compare_key(&self, key: Key<'_>) -> bool {
    match key {
      Key::DID(value) => self.resource() == value.as_str().as_bytes(),
      Key::Ident(value) => self.identifier().ident == value,
      Key::Index(value) => self.identifier().index.get() == value,
    }
  }
}
