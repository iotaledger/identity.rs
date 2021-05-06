// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use futures::executor;
use futures::TryStreamExt;
use hashbrown::HashMap;
use identity_core::crypto::KeyType;
use identity_core::crypto::SetSignature;
use identity_did::verification::MethodType;
use identity_iota::client::Client;
use identity_iota::client::Network;
use identity_iota::did::DocumentDiff;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;
use identity_iota::tangle::MessageId;
use serde::Serialize;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::RwLockWriteGuard;

use crate::account::AccountBuilder;
use crate::error::Error;
use crate::error::Result;
use crate::events::Command;
use crate::events::Commit;
use crate::events::Context;
use crate::events::Event;
use crate::events::EventData;
use crate::identity::IdentityCreate;
use crate::identity::IdentityId;
use crate::identity::IdentityIndex;
use crate::identity::IdentityKey;
use crate::identity::IdentitySnapshot;
use crate::identity::IdentityState;
use crate::identity::IdentityTag;
use crate::identity::TinyMethod;
use crate::storage::Storage;
use crate::types::Fragment;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::utils::Shared;

const OSC: Ordering = Ordering::SeqCst;

#[derive(Debug)]
pub struct Account {
  config: Arc<Config>,
  state: State,
  store: Box<dyn Storage>,
  index: RwLock<IdentityIndex>,
}

impl Account {
  /// Creates a new [AccountBuilder].
  pub fn builder() -> AccountBuilder {
    AccountBuilder::new()
  }

  /// Creates a new `Account` instance.
  pub async fn new(store: impl Storage) -> Result<Self> {
    Self::with_config(store, Config::new()).await
  }

  /// Creates a new `Account` instance with the given `config`.
  pub async fn with_config(store: impl Storage, config: Config) -> Result<Self> {
    let index: IdentityIndex = store.index().await?;

    Ok(Self {
      store: Box::new(store),
      state: State::new(),
      config: Arc::new(config),
      index: RwLock::new(index),
    })
  }

  /// Returns a reference to the [Storage] implementation.
  pub fn store(&self) -> &dyn Storage {
    &*self.store
  }

  /// Returns the auto-save configuration value.
  pub fn autosave(&self) -> AutoSave {
    self.config.autosave
  }

  /// Returns the save-on-drop configuration value.
  pub fn dropsave(&self) -> bool {
    self.config.dropsave
  }

  /// Returns the total number of actions executed by this instance.
  pub fn actions(&self) -> usize {
    self.state.actions.load(OSC)
  }

  // ===========================================================================
  // Identity
  // ===========================================================================

  /// Returns a list of tags identifying the identities in the account.
  pub async fn list_identities(&self) -> Vec<IdentityTag> {
    self.index.read().await.tags()
  }

  /// Finds and returns the state snapshot for the identity specified by given `key`.
  pub async fn find_identity<K: IdentityKey>(&self, key: K) -> Result<Option<IdentitySnapshot>> {
    match self.resolve_id(key).await {
      Some(identity) => self.load_snapshot(identity).await.map(Some),
      None => Ok(None),
    }
  }

  pub async fn create_identity(&self, input: IdentityCreate) -> Result<IdentitySnapshot> {
    // Acquire write access to the index.
    let mut index: RwLockWriteGuard<'_, _> = self.index.write().await;

    let identity: IdentityId = index.try_next_id()?;

    // Create the initialization command
    let command: Command = Command::CreateIdentity {
      network: input.network,
      shard: input.shard,
      authentication: Self::key_to_method(input.key_type),
    };

    // Process the command
    self.process(identity, command, false).await?;

    // Read the latest snapshot
    let snapshot: IdentitySnapshot = self.load_snapshot(identity).await?;
    let document: &IotaDID = snapshot.identity().try_did()?;

    // Add the identity to the index
    if let Some(name) = input.name {
      index.set_named(identity, document, name)?;
    } else {
      index.set(identity, document)?;
    }

    // Store the updated identity index
    self.store.set_index(&index).await?;

    // Write the changes to disk
    self.save(false).await?;

    // Return the state snapshot
    Ok(snapshot)
  }

  /// Updates the identity specified by the given `key` with the given `command`.
  pub async fn update_identity<K: IdentityKey>(&self, key: K, command: Command) -> Result<()> {
    let identity: IdentityId = self.try_resolve_id(key).await?;

    self.process(identity, command, true).await?;

    Ok(())
  }

  /// Removes the identity specified by the given `key`.
  ///
  /// Note: This will remove all associated events and key material - recovery is NOT POSSIBLE!
  pub async fn delete_identity<K: IdentityKey>(&self, key: K) -> Result<()> {
    // Acquire write access to the index.
    let mut index: RwLockWriteGuard<'_, _> = self.index.write().await;

    // Remove the identity from the index
    let identity: IdentityId = index.del(key)?.1;

    // Remove all associated keys and events
    self.store.purge(identity).await?;

    // Store the updated identity index
    self.store.set_index(&index).await?;

    // Write the changes to disk
    self.save(false).await?;

    Ok(())
  }

  /// Resolves the DID Document associated with the specified `key`.
  pub async fn resolve_identity<K: IdentityKey>(&self, key: K) -> Result<IotaDocument> {
    let identity: IdentityId = self.try_resolve_id(key).await?;
    let snapshot: IdentitySnapshot = self.load_snapshot(identity).await?;
    let document: &IotaDID = snapshot.identity().try_did()?;
    let network: Network = Network::from_did(document);

    // Fetch the DID Document from the Tangle
    self
      .state
      .clients
      .load(network)
      .await?
      .read_document(document)
      .await
      .map_err(Into::into)
  }

  /// Signs `data` with the key specified by `fragment`.
  pub async fn sign<K, U>(&self, key: K, fragment: &str, target: &mut U) -> Result<()>
  where
    K: IdentityKey,
    U: Serialize + SetSignature,
  {
    let identity: IdentityId = self.try_resolve_id(key).await?;
    let snapshot: IdentitySnapshot = self.load_snapshot(identity).await?;
    let state: &IdentityState = snapshot.identity();

    let fragment: Fragment = Fragment::new(fragment.into());
    let method: &TinyMethod = state.methods().fetch(fragment.name())?;
    let location: &KeyLocation = method.location();

    state.sign_data(&self.store, location, target).await?;

    Ok(())
  }

  async fn resolve_id<K: IdentityKey>(&self, key: K) -> Option<IdentityId> {
    self.index.read().await.get(key)
  }

  async fn try_resolve_id<K: IdentityKey>(&self, key: K) -> Result<IdentityId> {
    self.resolve_id(key).await.ok_or(Error::IdentityNotFound)
  }

  // ===========================================================================
  // Misc. Private
  // ===========================================================================

  #[doc(hidden)]
  pub async fn process(&self, id: IdentityId, command: Command, persist: bool) -> Result<()> {
    // Load the latest state snapshot from storage
    let root: IdentitySnapshot = self.load_snapshot(id).await?;

    debug!("[Account::process] Root = {:#?}", root);

    // Process the command with a read-only view of the state
    let context: Context<'_> = Context::new(root.identity(), self.store());
    let events: Option<Vec<Event>> = command.process(context).await?;

    debug!("[Account::process] Events = {:#?}", events);

    if let Some(events) = events {
      // Commit all events returned by the command
      let commits: Vec<Commit> = self.commit_events(&root, &events).await?;

      debug!("[Account::process] Commits = {:#?}", commits);

      match Publish::new(&commits) {
        Publish::Auth => self.process_auth_change(root).await?,
        Publish::Diff => self.process_diff_change(root).await?,
        Publish::None => {}
      }
    }

    // Update the total number of executions
    self.state.actions.fetch_add(1, OSC);

    if persist {
      self.save(false).await?;
    }

    Ok(())
  }

  async fn sign_document(
    &self,
    old_state: &IdentityState,
    new_state: &IdentityState,
    document: &mut IotaDocument,
  ) -> Result<()> {
    if new_state.auth_generation() == Generation::new() {
      let method: &TinyMethod = new_state.authentication()?;
      let location: &KeyLocation = method.location();

      // Sign the DID Document with the current authentication method
      new_state.sign_data(&self.store, location, document).await?;
    } else {
      let method: &TinyMethod = old_state.authentication()?;
      let location: &KeyLocation = method.location();

      // Sign the DID Document with the previous authentication method
      old_state.sign_data(&self.store, location, document).await?;
    }

    Ok(())
  }

  async fn process_auth_change(&self, old_root: IdentitySnapshot) -> Result<()> {
    let new_root: IdentitySnapshot = self.load_snapshot(old_root.id()).await?;

    let old_state: &IdentityState = old_root.identity();
    let new_state: &IdentityState = new_root.identity();

    let mut new_doc: IotaDocument = new_state.to_document()?;

    self.sign_document(old_state, new_state, &mut new_doc).await?;

    #[cfg(test)]
    let message: MessageId = MessageId::null();

    #[cfg(not(test))]
    let message: MessageId = {
      let network: Network = Network::from_did(new_doc.id());
      let client: Arc<Client> = self.state.clients.load(network).await?;

      client.publish_document(&new_doc).await?
    };

    let events: [Event; 1] = [Event::new(EventData::AuthMessage(message))];

    self.commit_events(&new_root, &events).await?;

    Ok(())
  }

  async fn process_diff_change(&self, old_root: IdentitySnapshot) -> Result<()> {
    let new_root: IdentitySnapshot = self.load_snapshot(old_root.id()).await?;

    let old_state: &IdentityState = old_root.identity();
    let new_state: &IdentityState = new_root.identity();

    let old_doc: IotaDocument = old_state.to_document()?;
    let new_doc: IotaDocument = new_state.to_document()?;

    let diff_id: &MessageId = old_state.diff_message_id();

    let mut diff: DocumentDiff = DocumentDiff::new(&old_doc, &new_doc, *diff_id)?;

    let method: &TinyMethod = old_state.authentication()?;
    let location: &KeyLocation = method.location();

    old_state.sign_data(&self.store, location, &mut diff).await?;

    #[cfg(test)]
    let message: MessageId = MessageId::null();

    #[cfg(not(test))]
    let message: MessageId = {
      let auth_id: &MessageId = old_state.this_message_id();
      let network: Network = Network::from_did(old_doc.id());
      let client: Arc<Client> = self.state.clients.load(network).await?;

      client.publish_diff(auth_id, &diff).await?
    };

    let events: [Event; 1] = [Event::new(EventData::DiffMessage(message))];

    self.commit_events(&new_root, &events).await?;

    Ok(())
  }

  async fn fold_snapshot(snapshot: IdentitySnapshot, commit: Commit) -> Result<IdentitySnapshot> {
    Ok(IdentitySnapshot {
      sequence: commit.sequence().max(snapshot.sequence),
      identity: commit.into_event().apply(snapshot.identity).await?,
    })
  }

  #[doc(hidden)]
  pub async fn load_snapshot(&self, id: IdentityId) -> Result<IdentitySnapshot> {
    // Retrieve the state snapshot from storage or create a new one.
    let initial: IdentitySnapshot = self
      .store
      .snapshot(id)
      .await?
      .unwrap_or_else(|| IdentitySnapshot::new(IdentityState::new(id)));

    // Apply all recent events to the state and create a new snapshot
    self
      .store
      .stream(id, initial.sequence())
      .await?
      .try_fold(initial, Self::fold_snapshot)
      .await
  }

  async fn commit_events(&self, state: &IdentitySnapshot, events: &[Event]) -> Result<Vec<Commit>> {
    // Bail early if there are no new events
    if events.is_empty() {
      return Ok(Vec::new());
    }

    // Get the current sequence index of the snapshot
    let mut sequence: Generation = state.sequence();
    let mut commits: Vec<Commit> = Vec::with_capacity(events.len());

    // Iterate over the events and create a new commit with the correct sequence
    for event in events {
      sequence = sequence.try_increment()?;
      commits.push(Commit::new(state.id(), sequence, event.clone()));
    }

    // Append the list of commits to the store
    self.store.append(state.id(), &commits).await?;

    // Store a snapshot every N events
    if sequence.to_u32() % self.config.milestone == 0 {
      let mut state: IdentitySnapshot = state.clone();

      // Fold the new commits into the snapshot
      for commit in commits.iter().cloned() {
        state = Self::fold_snapshot(state, commit).await?;
      }

      // Store the new snapshot
      self.store.set_snapshot(state.id(), &state).await?;
    }

    // Return the list of stored events
    Ok(commits)
  }

  async fn save(&self, force: bool) -> Result<()> {
    match self.config.autosave {
      AutoSave::Every => {
        self.store.flush_changes().await?;
      }
      AutoSave::Batch(step) if force || (step != 0 && self.actions() % step == 0) => {
        self.store.flush_changes().await?;
      }
      AutoSave::Batch(_) | AutoSave::Never => {}
    }

    Ok(())
  }

  fn key_to_method(type_: KeyType) -> MethodType {
    match type_ {
      KeyType::Ed25519 => MethodType::Ed25519VerificationKey2018,
    }
  }
}

impl Drop for Account {
  fn drop(&mut self) {
    if self.config.dropsave && self.actions() != 0 {
      // TODO: Handle Result (?)
      let _ = executor::block_on(self.store.flush_changes());
    }
  }
}

// =============================================================================
// Config
// =============================================================================

/// Top-level configuration for Identity [Account]s
#[derive(Clone, Debug)]
pub struct Config {
  autosave: AutoSave,
  dropsave: bool,
  milestone: u32,
}

impl Config {
  #[cfg(test)]
  const MILESTONE: u32 = 1;
  #[cfg(not(test))]
  const MILESTONE: u32 = 1; // 10

  /// Creates a new default `Config`.
  pub fn new() -> Self {
    Self {
      autosave: AutoSave::Every,
      dropsave: true,
      milestone: Self::MILESTONE,
    }
  }

  /// Sets the account auto-save behaviour.
  pub fn autosave(mut self, value: AutoSave) -> Self {
    self.autosave = value;
    self
  }

  /// Save the account state on drop.
  pub fn dropsave(mut self, value: bool) -> Self {
    self.dropsave = value;
    self
  }

  /// Save a state snapshot every N actions.
  pub fn milestone(mut self, value: u32) -> Self {
    self.milestone = value;
    self
  }
}

impl Default for Config {
  fn default() -> Self {
    Self::new()
  }
}

// =============================================================================
// AutoSave
// =============================================================================

/// Available auto-save behaviours.
#[derive(Clone, Copy, Debug)]
pub enum AutoSave {
  /// Never save
  Never,
  /// Save after every action
  Every,
  /// Save after every N actions
  Batch(usize),
}

// =============================================================================
// ClientMap
// =============================================================================

// A cache of clients to be used for publishing/resolving multiple identities
struct ClientMap {
  data: Shared<HashMap<Network, Arc<Client>>>,
}

impl ClientMap {
  /// Creates a new `ClientMap`.
  fn new() -> Self {
    Self {
      data: Shared::new(HashMap::new()),
    }
  }

  async fn load(&self, network: Network) -> Result<Arc<Client>> {
    // If we have a client for the given network, return it
    if let Some(client) = self.data.read()?.get(&network).map(Arc::clone) {
      return Ok(client);
    }

    // Initialize a new client for the given network
    let client: Arc<Client> = Client::from_network(network).await.map(Arc::new)?;

    // Store the newly created client
    self.data.write()?.insert(network, Arc::clone(&client));

    // Return the newly created client
    Ok(client)
  }
}

impl Debug for ClientMap {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("ClientMap")
  }
}

// =============================================================================
// State
// =============================================================================

// Internal account state
#[derive(Debug)]
struct State {
  actions: AtomicUsize,
  clients: ClientMap,
}

impl State {
  /// Creates a new `State` instance.
  fn new() -> Self {
    Self {
      actions: AtomicUsize::new(0),
      clients: ClientMap::new(),
    }
  }
}

// =============================================================================
// Publish
// =============================================================================

#[derive(Clone, Copy, Debug)]
enum Publish {
  None,
  Auth,
  Diff,
}

impl Publish {
  fn new(commits: &[Commit]) -> Self {
    commits.iter().fold(Self::None, Self::apply)
  }

  const fn apply(self, commit: &Commit) -> Self {
    match (self, commit.event().data()) {
      (Self::Auth, _) => Self::Auth,
      (_, EventData::IdentityCreated(..)) => Self::Auth,
      (_, EventData::AuthMessage(_)) => self,
      (_, EventData::DiffMessage(_)) => self,
      (_, _) => Self::Diff,
    }
  }
}
