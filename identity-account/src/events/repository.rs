// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::cell::Cell;
use futures::TryStreamExt;

use crate::chain::ChainData;
use crate::error::Result;
use crate::events::Commit;
use crate::events::Event;
use crate::storage::Storage;
use crate::types::ChainId;
use crate::types::Index;

const MILESTONE: Index = Index::from_u32(10);

#[derive(Clone, Debug)]
pub struct Repository<'a, T> {
  chain: ChainId,
  store: &'a T,
  version: Cell<Index>,
}

impl<'a, T: Storage> Repository<'a, T> {
  pub fn new(chain: ChainId, store: &'a T) -> Self {
    Self {
      chain,
      store,
      version: Cell::new(Index::ZERO),
    }
  }

  pub fn chain(&self) -> ChainId {
    self.chain
  }

  pub fn store(&self) -> &T {
    self.store
  }

  pub fn version(&self) -> Index {
    self.version.get()
  }

  pub async fn load(&self) -> Result<Snapshot> {
    // Retrieve the state snapshot from storage or create a new one.
    let initial: Snapshot = self
      .store
      .get_snapshot(self.chain)
      .await?
      .unwrap_or_else(|| Snapshot::new(ChainData::new(self.chain)));

    // Get the initial version of the snapshot
    let version: Index = initial.version();

    // Apply all recent events to the state and create a new snapshot
    let snapshot: Snapshot = self
      .store
      .stream(self.chain, version)
      .await?
      .try_fold(initial, fold)
      .await?;

    trace!("[Repository::load] Version = {} -> {}", version, snapshot.version());
    trace!("[Repository::load] Snapshot = {:#?}", snapshot);

    self.version.set(snapshot.version());

    Ok(snapshot)
  }

  pub async fn commit(&self, events: &[Event], state: &Snapshot) -> Result<Vec<Commit>> {
    // Sanity check - snapshot needs to be one retrieved by `load`
    assert_eq!(self.chain(), state.chain(), "Invalid Chain Id");
    assert_eq!(self.version(), state.version(), "Invalid Snapshot Version");

    trace!("[Repository::commit] Count = {}", events.len());

    // Bail early if there are no new events
    if events.is_empty() {
      return Ok(Vec::new());
    }

    // Get the current version of the snapshot
    let mut version: Index = state.version();
    let mut commits: Vec<Commit> = Vec::with_capacity(events.len());

    // Iterate over the events and create a new commit with the correct version
    for event in events {
      version = version.try_increment()?;
      commits.push(Commit::new(state.chain(), version, event.clone()));
    }

    // Append the list of commits to the store
    self.store.append(state.chain(), &commits).await?;

    // Store a snapshot every N events
    if version > Index::ZERO && version % MILESTONE == Index::ZERO {
      // Fold the new commits into the snapshot
      let mut state: Snapshot = state.clone();

      for commit in commits.iter() {
        state = fold(state, commit.clone()).await?;
      }

      self.store.set_snapshot(state.chain(), &state).await?;
    }

    Ok(commits)
  }
}

async fn fold(state: Snapshot, commit: Commit) -> Result<Snapshot> {
  Ok(Snapshot {
    version: commit.index().max(state.version),
    state: commit.apply(state.state).await?,
  })
}

// =============================================================================
// Snapshot
// =============================================================================

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Snapshot {
  #[serde(rename = "v")]
  version: Index,
  #[serde(rename = "s")]
  state: ChainData,
}

impl Snapshot {
  pub fn new(state: ChainData) -> Self {
    Self {
      version: Index::ZERO,
      state,
    }
  }

  pub fn chain(&self) -> ChainId {
    self.state.chain()
  }

  pub fn version(&self) -> Index {
    self.version
  }

  pub fn state(&self) -> &ChainData {
    &self.state
  }
}
