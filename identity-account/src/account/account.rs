// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::num::NonZeroUsize;
use futures::executor;
use hashbrown::HashMap;
use identity_core::crypto::TrySignatureMut;
use identity_iota::client::Client;
use identity_iota::client::Network;
use identity_iota::did::Document;
use identity_iota::did::DocumentDiff;
use identity_iota::did::DID;
use identity_iota::tangle::MessageId;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use crate::chain::ChainData;
use crate::chain::ChainIndex;
use crate::chain::Key;
use crate::error::Error;
use crate::error::Result;
use crate::events::Command;
use crate::events::Commit;
use crate::events::Context;
use crate::events::Event;
use crate::events::Repository;
use crate::events::Snapshot;
use crate::storage::Storage;
use crate::types::ChainId;
use crate::types::IdentityConfig;
use crate::utils::key_to_method;
use crate::utils::Shared;

const OSC: Ordering = Ordering::SeqCst;

#[derive(Debug)]
pub struct Account<T: Storage> {
  store: T,
  index: Shared<ChainIndex>,
  clients: Shared<HashMap<Network, Arc<Client>>>,
  autosave: AutoSave,
  commands: AtomicUsize,
}

impl<T: Storage> Account<T> {
  pub async fn new(store: T) -> Result<Self> {
    let index: ChainIndex = store.get_chain_index().await?;

    Ok(Self {
      store,
      index: Shared::new(index),
      clients: Shared::new(HashMap::new()),
      autosave: AutoSave::Every,
      commands: AtomicUsize::new(0),
    })
  }

  pub fn autosave(&self) -> AutoSave {
    self.autosave
  }

  pub fn set_autosave(&mut self, value: AutoSave) {
    self.autosave = value;
  }

  pub fn store(&self) -> &T {
    &self.store
  }

  // ===========================================================================
  // Identity Index
  // ===========================================================================

  pub fn index(&self) -> Result<ChainIndex> {
    self.index.read().map(|index| index.clone())
  }

  pub fn with_index<U>(&self, f: impl Fn(&ChainIndex) -> U) -> Result<U> {
    self.index.read().map(|index| f(&index))
  }

  pub fn try_with_index<U>(&self, f: impl Fn(&ChainIndex) -> Result<U>) -> Result<U> {
    self.index.read().and_then(|index| f(&index))
  }

  // ===========================================================================
  // Identity Management
  // ===========================================================================

  pub async fn get<K: Key>(&self, key: K) -> Result<Document> {
    // Acquire read access to the index
    let index: RwLockReadGuard<_> = self.index.read()?;

    // Read the chain id from the index
    let chain: ChainId = index.get(key).ok_or(Error::ChainIdNotFound)?;

    // Fetch the latest state snapshot
    let root: Snapshot = self.snapshot(chain).await?;

    // Create a DID Document from the snapshot
    let data: Document = root.state().to_document(&self.store).await?;

    Ok(data)
  }

  pub async fn create(&self, config: IdentityConfig) -> Result<ChainId> {
    // Acquire write access to the index - at the moment we can't add multiple
    // identities concurrently.
    let mut index: RwLockWriteGuard<_> = self.index.write()?;

    let chain: ChainId = index.next_id();

    // Create the initialization command
    let command: Command = Command::CreateChain {
      network: config.network,
      shard: config.shard,
      authentication: key_to_method(config.key_type),
    };

    // Process the command
    self.process(chain, command, false).await?;

    // Read the latest snapshot
    let snapshot: Snapshot = self.snapshot(chain).await?;
    let document: &DID = snapshot.state().try_document()?;

    // Add the chain to the index
    if let Some(name) = config.name {
      index.set_named(chain, document, name)?;
    } else {
      index.set(chain, document)?;
    }

    // Store the updated index
    self.store.set_chain_index(&index).await?;
    self.save(false).await?;

    Ok(chain)
  }

  pub async fn snapshot(&self, chain: ChainId) -> Result<Snapshot> {
    Repository::new(chain, &self.store).load().await
  }

  pub async fn process(&self, chain: ChainId, command: Command, persist: bool) -> Result<()> {
    // Load chain snapshot from storage
    let repo: Repository<'_, T> = Repository::new(chain, &self.store);
    let root: Snapshot = repo.load().await?;

    trace!("[Account::process] Repo = {:#?}", repo);
    trace!("[Account::process] Root = {:#?}", root);

    // Process the command with a read-only view of the state
    let context: Context<'_, T> = Context::new(root.state(), &self.store);
    let events: Option<Vec<Event>> = command.process(context).await?;

    debug!("[Account::process] Events = {:#?}", events);

    if let Some(events) = events {
      // Commit all events returned by the command
      let commits: Vec<Commit> = repo.commit(&events, &root).await?;

      trace!("[Account::process] Commits = {:#?}", commits);
      trace!("[Account::process] Self    = {:#?}", self);

      let change: Change = Change::new(&commits);

      if matches!(change, Change::None) {
        return Ok(());
      }

      match change {
        Change::Auth => self.process_auth_change(repo).await?,
        Change::Diff => self.process_diff_change(repo, root).await?,
        Change::None => unreachable!(), // should never get here due to early return above
      }
    }

    // Update the command counter
    self.commands.fetch_add(1, OSC);

    if persist {
      self.save(false).await?;
    }

    Ok(())
  }

  async fn process_auth_change(&self, repo: Repository<'_, T>) -> Result<()> {
    let new_root: Snapshot = repo.load().await?;
    let new_data: Document = new_root.state().to_signed_document(&self.store).await?;

    let client: Arc<Client> = self.client(new_data.id().into()).await?;
    let message: MessageId = client.publish_document(&new_data).await?;

    debug!("[Account::process_auth_change] Message Id = {:?}", message);

    repo.commit(&[Event::AuthMessage { message }], &new_root).await?;

    Ok(())
  }

  async fn process_diff_change(&self, repo: Repository<'_, T>, old_root: Snapshot) -> Result<()> {
    let new_root: Snapshot = repo.load().await?;

    let new_state: &ChainData = new_root.state();
    let old_state: &ChainData = old_root.state();

    let mut new_data: Document = new_state.to_signed_document(&self.store).await?;
    let mut old_data: Document = old_state.to_signed_document(&self.store).await?;

    let client: Arc<Client> = self.client(old_data.id().into()).await?;

    // Should never fail - to_signed_document always adds signatures
    new_data.try_signature_mut().unwrap().hide_value();
    old_data.try_signature_mut().unwrap().hide_value();

    let previous: MessageId = old_state
      .diff_message_id()
      .copied()
      .ok_or(Error::DiffMessageIdNotFound)?;

    let mut diff: DocumentDiff = DocumentDiff::new(&old_data, &new_data, previous)?;

    old_state.sign_data(&old_state.current_auth(), &self.store, &mut diff).await?;

    let message: MessageId = client.publish_diff(&previous, &diff).await?;

    debug!("[Account::process_diff_change] Message Id = {:?}", message);

    repo.commit(&[Event::DiffMessage { message }], &new_root).await?;

    Ok(())
  }

  async fn save(&self, force: bool) -> Result<()> {
    let count: usize = self.commands.load(OSC);

    match self.autosave {
      AutoSave::Every => {
        self.store.flush_changes().await?;
      }
      AutoSave::Batch(step) if force || count % step.get() == 0 => {
        self.store.flush_changes().await?;
      }
      AutoSave::Batch(_) | AutoSave::Never => {}
    }

    Ok(())
  }

  // ===========================================================================
  // Tangle
  // ===========================================================================

  async fn client(&self, network: Network) -> Result<Arc<Client>> {
    // If we have a client for the given network, return it
    if let Some(client) = self.clients.read()?.get(&network).cloned() {
      return Ok(client);
    }

    // Initialize a new client for the given network
    let client: Client = Client::from_network(network).await?;
    let client: Arc<Client> = Arc::new(client);

    // Store the newly created client
    self.clients.write()?.insert(network, Arc::clone(&client));

    Ok(client)
  }
}

impl<T: Storage> Drop for Account<T> {
  fn drop(&mut self) {
    // if we've executed any commands...
    if self.commands.load(OSC) > 0 {
      // ...ensure we write any outstanding changes
      let _ = executor::block_on(self.store.flush_changes());
    }
  }
}

// =============================================================================
// AutoSave
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub enum AutoSave {
  /// Never save
  Never,
  /// Save after every action
  Every,
  /// Save after every N actions
  Batch(NonZeroUsize),
}

// =============================================================================
// Change
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub enum Change {
  None,
  Auth,
  Diff,
}

impl Change {
  pub fn new(commits: &[Commit]) -> Self {
    commits.iter().fold(Self::None, Self::apply)
  }

  pub const fn apply(self, commit: &Commit) -> Self {
    match commit.event {
      Event::ChainCreated { .. } => self.auth(),
      Event::AuthMessage { .. } | Event::DiffMessage { .. } => self,
      _ => self.diff(),
    }
  }

  pub const fn auth(self) -> Self {
    match self {
      Self::Diff | Self::None => Self::Auth,
      Self::Auth => self,
    }
  }

  pub const fn diff(self) -> Self {
    match self {
      Self::None => Self::Diff,
      Self::Auth | Self::Diff => self,
    }
  }
}
