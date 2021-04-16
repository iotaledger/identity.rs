// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::chain::ChainData;
use crate::error::Result;
use crate::events::Event;
use crate::types::ChainId;
use crate::types::Index;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Commit {
  chain: ChainId,
  index: Index,
  #[serde(flatten)]
  pub(crate) event: Event,
}

impl Commit {
  pub fn new(chain: ChainId, index: Index, event: Event) -> Self {
    Self { chain, index, event }
  }

  pub fn chain(&self) -> ChainId {
    self.chain
  }

  pub fn index(&self) -> Index {
    self.index
  }

  pub fn event(&self) -> &Event {
    &self.event
  }

  pub fn into_event(self) -> Event {
    self.event
  }

  pub async fn apply(self, state: ChainData) -> Result<ChainData> {
    self.event.apply(state).await
  }
}
