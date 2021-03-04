// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_iota::did::Document;
use identity_iota::did::DID;

use crate::error::Result;
use crate::storage::StorageHandle;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identity {
  pub(crate) id: DID,
  pub(crate) index: u32,
  pub(crate) name: String,
  pub(crate) created_at: Timestamp,
  pub(crate) updated_at: Timestamp,
  pub(crate) document: Document, // TODO: Replace with DocumentChain
}

impl Identity {
  pub fn id(&self) -> &DID {
    &self.id
  }

  pub fn index(&self) -> u32 {
    self.index
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn created_at(&self) -> Timestamp {
    self.created_at
  }

  pub fn updated_at(&self) -> Timestamp {
    self.updated_at
  }

  pub fn document(&self) -> &Document {
    &self.document
  }

  pub(crate) async fn save(&self, storage: &StorageHandle) -> Result<()> {
    let id: Vec<u8> = self.id.to_string().into_bytes();
    let json: Vec<u8> = self.to_json_vec()?;

    storage.set(&id, &json).await?;

    Ok(())
  }
}
