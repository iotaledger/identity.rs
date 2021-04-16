// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::TryStreamExt;

use crate::chain::ChainData;
use crate::error::Result;
use crate::events::Commit;
use crate::events::Event;
use crate::storage::Storage;
use crate::types::ChainId;
use crate::types::Index;

// TODO: Make configurable
const MILESTONE: Index = Index::from_u32(10);

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Repository<'a, T>(&'a T);

impl<'a, T: Storage> Repository<'a, T> {
  pub fn new(store: &'a T) -> Self {
    Self(store)
  }

  pub fn store(&self) -> &T {
    self.0
  }

  pub async fn load(&self, chain: ChainId) -> Result<Snapshot> {
    // Retrieve the state snapshot from storage or create a new one.
    let initial: Snapshot = self
      .store()
      .get_snapshot(chain)
      .await?
      .unwrap_or_else(|| Snapshot::new(ChainData::new(chain)));

    // Get the initial version of the snapshot
    let version: Index = initial.version();

    // Apply all recent events to the state and create a new snapshot
    let snapshot: Snapshot = self
      .store()
      .stream(chain, version)
      .await?
      .try_fold(initial, fold)
      .await?;

    trace!("[Repository::load] Version = {} -> {}", version, snapshot.version());

    Ok(snapshot)
  }

  pub async fn commit(&self, events: &[Event], state: &Snapshot) -> Result<Vec<Commit>> {
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
    self.store().append(state.chain(), &commits).await?;

    // Store a snapshot every N events
    if version > Index::ZERO && version % MILESTONE == Index::ZERO {
      // Fold the new commits into the snapshot
      let mut state: Snapshot = state.clone();

      for commit in commits.iter() {
        state = fold(state, commit.clone()).await?;
      }

      self.store().set_snapshot(state.chain(), &state).await?;
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
