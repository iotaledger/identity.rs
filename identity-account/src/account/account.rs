// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::num::NonZeroUsize;
use futures::executor;
use hashbrown::HashMap;
use identity_core::common::Url;
use identity_core::crypto::SetSignature;
use identity_core::crypto::TrySignatureMut;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::client::Network;
use identity_iota::did::Document;
use identity_iota::did::DocumentDiff;
use identity_iota::did::DID;
use identity_iota::tangle::MessageId;
use serde::Serialize;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use crate::account::Client;
use crate::chain::ChainData;
use crate::chain::ChainIndex;
use crate::chain::ChainKey;
use crate::chain::Key;
use crate::chain::TinyMethod;
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

  pub fn store(&self) -> &T {
    &self.store
  }

  pub fn autosave(&self) -> AutoSave {
    self.autosave
  }

  pub fn set_autosave(&mut self, value: AutoSave) {
    self.autosave = value;
  }

  pub fn commands(&self) -> usize {
    self.commands.load(OSC)
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
    // Read the chain id from the index
    let chain: ChainId = self.index.read()?.get(key).ok_or(Error::ChainIdNotFound)?;
    // Fetch the latest state snapshot
    let root: Snapshot = self.snapshot(chain).await?;

    // Create a DID Document from the snapshot
    root.state().to_document()
  }

  pub async fn create(&self, config: IdentityConfig) -> Result<ChainId> {
    // Acquire write access to the index.
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

  pub async fn sign<K, U>(&self, key: K, fragment: &str, target: &mut U) -> Result<()>
  where
    K: Key,
    U: Serialize + SetSignature,
  {
    // Acquire read access to the index
    let index: RwLockReadGuard<_> = self.index.read()?;

    // Read the chain id from the index
    let chain: ChainId = index.get(key).ok_or(Error::ChainIdNotFound)?;

    // Fetch the latest state snapshot
    let root: Snapshot = self.snapshot(chain).await?;

    let data: &ChainData = root.state();
    let method: &TinyMethod = data.methods().fetch(fragment)?;

    data.sign_data(&self.store, method.location(), target).await?;

    Ok(())
  }

  pub async fn create_method<K: Key>(
    &self,
    key: K,
    type_: MethodType,
    scope: MethodScope,
    fragment: &str,
  ) -> Result<()> {
    let command: Command = Command::create_method()
      .type_(type_)
      .scope(scope)
      .fragment(fragment)
      .finish()?;

    self.execute(key, command).await
  }

  pub async fn delete_method<K: Key>(&self, key: K, fragment: &str) -> Result<()> {
    let command: Command = Command::delete_method().fragment(fragment).finish()?;

    self.execute(key, command).await
  }

  pub async fn detach_method<K: Key>(&self, key: K, fragment: &str, scope: MethodScope) -> Result<()> {
    let command: Command = Command::delete_method().fragment(fragment).scope(scope).finish()?;

    self.execute(key, command).await
  }

  pub async fn create_service<K: Key>(&self, key: K, fragment: &str, type_: &str, endpoint: Url) -> Result<()> {
    let command: Command = Command::create_service()
      .fragment(fragment)
      .type_(type_)
      .endpoint(endpoint)
      .finish()?;

    self.execute(key, command).await
  }

  pub async fn delete_service<K: Key>(&self, key: K, fragment: &str) -> Result<()> {
    let command: Command = Command::delete_service().fragment(fragment).finish()?;

    self.execute(key, command).await
  }

  pub async fn snapshot(&self, chain: ChainId) -> Result<Snapshot> {
    Repository::new(&self.store).load(chain).await
  }

  pub async fn execute<K: Key>(&self, key: K, command: Command) -> Result<()> {
    // Acquire read access to the index
    let index: RwLockReadGuard<_> = self.index.read()?;

    // Read the chain id from the index
    let chain: ChainId = index.get(key).ok_or(Error::ChainIdNotFound)?;

    // Process the command
    self.process(chain, command, true).await?;

    Ok(())
  }

  #[doc(hidden)]
  pub async fn process(&self, chain: ChainId, command: Command, persist: bool) -> Result<()> {
    // Load chain snapshot from storage
    let repo: Repository<'_, T> = Repository::new(&self.store);
    let root: Snapshot = repo.load(chain).await?;

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
      trace!("[Account::process] Self = {:#?}", self);

      let change: Change = Change::new(&commits);

      trace!("[Account::process] Change = {:?}", change);

      match change {
        Change::Auth => self.process_auth_change(chain, repo).await?,
        Change::Diff => self.process_diff_change(chain, repo, root).await?,
        Change::None => {}
      }
    }

    // Update the command counter
    self.commands.fetch_add(1, OSC);

    if persist {
      self.save(false).await?;
    }

    Ok(())
  }

  async fn process_auth_change(&self, chain: ChainId, repo: Repository<'_, T>) -> Result<()> {
    // TODO: Sign with previous auth key
    let new_root: Snapshot = repo.load(chain).await?;
    let new_data: Document = new_root.state().to_signed_document(&self.store).await?;

    let client: Arc<Client> = self.client(Network::from_did(new_data.id().into())).await?;
    let message: MessageId = client.publish_document(&new_data).await?;

    debug!("[Account::process_auth_change] Message Id = {:?}", message);

    repo.commit(&[Event::AuthMessage { message }], &new_root).await?;

    Ok(())
  }

  async fn process_diff_change(&self, chain: ChainId, repo: Repository<'_, T>, old_root: Snapshot) -> Result<()> {
    let new_root: Snapshot = repo.load(chain).await?;

    let new_state: &ChainData = new_root.state();
    let old_state: &ChainData = old_root.state();

    let mut new_doc: Document = new_state.to_signed_document(&self.store).await?;
    let mut old_doc: Document = old_state.to_signed_document(&self.store).await?;

    // Should never fail - to_signed_document always adds signatures
    new_doc.try_signature_mut().unwrap().hide_value();
    old_doc.try_signature_mut().unwrap().hide_value();

    let diff_id: &MessageId = new_state.diff_message_id();
    let auth_id: &MessageId = new_state.this_message_id();

    let auth: &ChainKey = old_state.authentication()?.location();
    let mut diff: DocumentDiff = DocumentDiff::new(&old_doc, &new_doc, *diff_id)?;

    old_state.sign_data(&self.store, auth, &mut diff).await?;

    let client: Arc<Client> = self.client(Network::from_did(old_doc.id().into())).await?;
    let message: MessageId = client.publish_diff(auth_id, &diff).await?;

    debug!("[Account::process_diff_change] Message Id = {:?}", message);

    repo.commit(&[Event::DiffMessage { message }], &new_root).await?;

    Ok(())
  }

  async fn save(&self, force: bool) -> Result<()> {
    match self.autosave {
      AutoSave::Every => {
        self.store.flush_changes().await?;
      }
      AutoSave::Batch(step) if force || self.commands() % step.get() == 0 => {
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
    if self.commands() > 0 {
      // ...ensure we write any outstanding changes
      let _ = executor::block_on(self.store.flush_changes());
    }
  }
}

impl<T: Storage> Debug for Account<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let clients: Result<Vec<_>> = self.clients.read().map(|data| data.keys().cloned().collect());

    f.debug_struct("Account")
      .field("store", &self.store)
      .field("index", &self.index)
      .field("clients", &clients)
      .field("autosave", &self.autosave)
      .field("commands", &self.commands)
      .finish()
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
