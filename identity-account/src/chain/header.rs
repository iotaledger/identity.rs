// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::DID;
use identity_core::convert::ToJson;

use crate::chain::ChainId;
use crate::error::Result;
use crate::storage::StorageHandle;
use crate::types::Resource;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChainHeader {
  id: ChainId,
  ident: String,
  document: DID,
}

impl ChainHeader {
  pub fn new(id: ChainId, ident: String, document: DID) -> Self {
    Self {
      id,
      ident,
      document,
    }
  }

  pub fn id(&self) -> &ChainId {
    &self.id
  }

  pub fn ident(&self) -> &str {
    &self.ident
  }

  pub fn document(&self) -> &DID {
    &self.document
  }

  pub async fn write(&self, storage: &StorageHandle) -> Result<()> {
    let key: &[u8] = self.id().as_bytes();
    let json: Vec<u8> = self.to_json_vec()?;

    storage.json_set(Resource::Chain, key, &json).await?;

    Ok(())
  }
}
