// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use hashbrown::HashMap;

use crate::error::Result;
use crate::storage::StorageHandle;
use crate::chain::ChainId;
use crate::chain::ChainHeader;
use crate::types::Resource;
use crate::utils;
use crate::utils::generate_unique_name;

const DEFAULT_NAME: &str = "Identity";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ChainIndex {
  chains: HashMap<ChainId, ChainHeader>,
}

impl ChainIndex {
  pub async fn load(storage: &StorageHandle) -> Result<Self> {
    println!("Loading Chain Index");

    let chains: HashMap<ChainId, ChainHeader> = storage
      .json_all(Resource::Chain)
      .await?
      .into_iter()
      .map(|chain: ChainHeader| (*chain.id(), chain))
      .collect();

    println!("Total Chains = {}", chains.len());

    Ok(Self { chains })
  }

  pub fn get(&self, chain: &ChainId) -> Option<&ChainHeader> {
    self.chains.get(chain)
  }

  pub fn get_mut(&mut self, chain: &ChainId) -> Option<&mut ChainHeader> {
    self.chains.get_mut(chain)
  }

  pub fn insert(&mut self, chain: ChainHeader) -> Option<ChainHeader> {
    self.chains.insert(*chain.id(), chain)
  }

  pub fn next_id(&self) -> ChainId {
    self.chains.keys().max().copied().unwrap_or_default().next()
  }

  pub fn unique_identifier(&self, chain: ChainId, base: Option<&str>) -> String {
    let default: String;

    let base: &str = match base {
      Some(base) => base,
      None => {
        default = format!("{} {}", DEFAULT_NAME, chain.to_u32());
        &default
      }
    };

    generate_unique_name(self.chains.values().map(ChainHeader::ident), base)
  }
}
