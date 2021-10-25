// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::executor;
use futures::StreamExt;
use futures::TryStreamExt;
use identity_core::common::Fragment;
use identity_core::crypto::KeyType;
use identity_core::crypto::SetSignature;
use identity_did::verification::MethodType;
use identity_iota::did::DocumentDiff;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;
use identity_iota::tangle::Client;
use identity_iota::tangle::ClientMap;
use identity_iota::tangle::MessageId;
use identity_iota::tangle::TangleResolve;
use serde::Serialize;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::account::AccountBuilder;
use crate::error::Result;
use crate::events::Commit;
use crate::events::Context;
use crate::events::CreateIdentity;
use crate::events::Event;
use crate::events::EventData;
use crate::events::Update;
use crate::identity::DIDLease;
use crate::identity::IdentitySetup;
use crate::identity::IdentitySnapshot;
use crate::identity::IdentityState;
use crate::identity::IdentityUpdater;
use crate::identity::TinyMethod;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::Error;

use super::config::AccountSetup;
use super::config::AutoSave;
use super::AccountConfig;

const OSC: Ordering = Ordering::SeqCst;

/// An account manages one identity.
///
/// It handles private keys, writing to storage and
/// publishing to the Tangle.
#[derive(Debug)]
pub struct Account {
  config: AccountConfig,
  storage: Arc<dyn Storage>,
  client_map: Arc<ClientMap>,
  actions: AtomicUsize,
  did: IotaDID,
  did_lease: DIDLease,
}

impl Account {
  /// Creates a new [AccountBuilder].
  pub fn builder() -> AccountBuilder {
    AccountBuilder::new()
  }

  /// Creates an [`Account`] for an existing identity, if it exists in the [`Storage`].
  pub(crate) async fn load_identity(setup: AccountSetup, did: IotaDID) -> Result<Self> {
    // Ensure the did exists in storage
    setup.storage.snapshot(&did).await?.ok_or(Error::IdentityNotFound)?;

    let did_lease = setup.storage.lease_did(&did).await?;

    Self::with_setup(setup, did, did_lease).await
  }

  /// Creates a new `Account` instance with the given `config`.
  async fn with_setup(setup: AccountSetup, did: IotaDID, did_lease: DIDLease) -> Result<Self> {
    Ok(Self {
      config: setup.config,
      storage: setup.storage,
      client_map: setup.client_map,
      actions: AtomicUsize::new(0),
      did,
      did_lease,
    })
  }

  /// Returns a reference to the [Storage] implementation.
  pub fn storage(&self) -> &dyn Storage {
    self.storage.as_ref()
  }

  /// Returns whether auto-publish is enabled.
  pub fn autopublish(&self) -> bool {
    self.config.autopublish
  }

  /// Returns the auto-save configuration value.
  pub fn autosave(&self) -> AutoSave {
    self.config.autosave
  }

  /// Returns whether save-on-drop is enabled.
  pub fn dropsave(&self) -> bool {
    self.config.dropsave
  }

  /// Returns the total number of actions executed by this instance.
  pub fn actions(&self) -> usize {
    self.actions.load(OSC)
  }

  /// Adds a pre-configured `Client` for Tangle interactions.
  pub fn set_client(&self, client: Client) {
    self.client_map.insert(client);
  }

  /// Returns the did of the managed identity.
  pub fn did(&self) -> &IotaDID {
    &self.did
  }

  // ===========================================================================
  // Identity
  // ===========================================================================

  /// Return a copy of the latest state of the identity
  pub async fn state(&self) -> Result<IdentityState> {
    Ok(self.load_snapshot().await?.into_identity())
  }

  /// Resolves the DID Document associated with this `Account` from the Tangle.
  pub async fn resolve_identity(&self) -> Result<IotaDocument> {
    let snapshot: IdentitySnapshot = self.load_snapshot().await?;
    let did: &IotaDID = snapshot.identity().try_did()?;

    // Fetch the DID Document from the Tangle
    self.client_map.resolve(did).await.map_err(Into::into)
  }

  /// Creates a new identity and returns an [`Account`] instance to manage it.
  /// The identity is stored locally in the [`Storage`] given in [`AccountSetup`], and published
  /// using the [`ClientMap`].
  ///
  /// See [`IdentityCreate`] to customize the identity creation.
  pub(crate) async fn create_identity(setup: AccountSetup, input: IdentitySetup) -> Result<Self> {
    let command = CreateIdentity {
      network: input.network,
      method_secret: input.method_secret,
      authentication: Self::key_to_method(input.key_type),
    };

    let snapshot = IdentitySnapshot::new(IdentityState::new());

    let (did, did_lease, events): (IotaDID, DIDLease, Vec<Event>) = command
      .process(snapshot.identity().integration_generation(), setup.storage.as_ref())
      .await?;

    let commits = Self::commit_events(&did, &setup.config, setup.storage.as_ref(), &snapshot, &events).await?;

    let account = Self::with_setup(setup, did, did_lease).await?;

    account.publish_commits(snapshot, commits, true).await?;

    Ok(account)
  }

  /// Returns the [`IdentityUpdater`] for this identity.
  ///
  /// On this type, various operations can be executed
  /// that modify an identity, such as creating services or methods.
  pub fn update_identity(&mut self) -> IdentityUpdater<'_> {
    IdentityUpdater::new(self)
  }

  /// Removes the identity from the local storage entirely.
  ///
  /// Note: This will remove all associated document updates and key material - recovery is NOT POSSIBLE!
  pub async fn delete_identity(self) -> Result<()> {
    // Remove all associated keys and events
    self.storage().purge(self.did()).await?;

    // Write the changes to disk
    self.save(false).await?;

    Ok(())
  }

  /// Signs `data` with the key specified by `fragment`.
  pub async fn sign<U>(&self, fragment: &str, target: &mut U) -> Result<()>
  where
    U: Serialize + SetSignature,
  {
    let snapshot: IdentitySnapshot = self.load_snapshot().await?;
    let state: &IdentityState = snapshot.identity();

    let fragment: Fragment = Fragment::new(fragment);
    let method: &TinyMethod = state.methods().fetch(fragment.name())?;
    let location: &KeyLocation = method.location();

    state.sign_data(self.did(), self.storage(), location, target).await?;

    Ok(())
  }

  // ===========================================================================
  // Misc. Private
  // ===========================================================================
  pub(crate) async fn process_update(&self, command: Update, persist: bool) -> Result<()> {
    // Load the latest state snapshot from storage
    let root: IdentitySnapshot = self.load_snapshot().await?;

    debug!("[Account::process] Root = {:#?}", root);

    // Process the command with a read-only view of the state
    let context: Context<'_> = Context::new(self.did(), root.identity(), self.storage());
    let events: Option<Vec<Event>> = command.process(context).await?;

    debug!("[Account::process] Events = {:#?}", events);

    if let Some(events) = events {
      let commits: Vec<Commit> = Self::commit_events(self.did(), &self.config, self.storage(), &root, &events).await?;
      self.publish_commits(root, commits, persist).await?;
    }

    Ok(())
  }

  /// Publishes the given commits to the tangle.
  pub(crate) async fn publish_commits(
    &self,
    root: IdentitySnapshot,
    commits: Vec<Commit>,
    persist: bool,
  ) -> Result<()> {
    debug!("[Account::process] Commits = {:#?}", commits);

    self.publish(root, commits, false).await?;

    // Update the total number of executions
    self.actions.fetch_add(1, OSC);

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
    if new_state.integration_generation() == Generation::new() {
      let method: &TinyMethod = new_state.authentication()?;
      let location: &KeyLocation = method.location();

      // Sign the DID Document with the current authentication method
      new_state
        .sign_data(self.did(), self.storage(), location, document)
        .await?;
    } else {
      let method: &TinyMethod = old_state.authentication()?;
      let location: &KeyLocation = method.location();

      // Sign the DID Document with the previous authentication method
      old_state
        .sign_data(self.did(), self.storage(), location, document)
        .await?;
    }

    Ok(())
  }

  async fn process_integration_change(&self, old_root: IdentitySnapshot) -> Result<()> {
    let new_root: IdentitySnapshot = self.load_snapshot().await?;

    let old_state: &IdentityState = old_root.identity();
    let new_state: &IdentityState = new_root.identity();

    let mut new_doc: IotaDocument = new_state.to_document()?;

    self.sign_document(old_state, new_state, &mut new_doc).await?;

    let message: MessageId = if self.config.testmode {
      MessageId::null()
    } else {
      self.client_map.publish_document(&new_doc).await?.into()
    };

    let events: [Event; 1] = [Event::new(EventData::IntegrationMessage(message))];

    Self::commit_events(self.did(), &self.config, self.storage(), &new_root, &events).await?;

    Ok(())
  }

  async fn process_diff_change(&self, old_root: IdentitySnapshot) -> Result<()> {
    let new_root: IdentitySnapshot = self.load_snapshot().await?;

    let old_state: &IdentityState = old_root.identity();
    let new_state: &IdentityState = new_root.identity();

    let old_doc: IotaDocument = old_state.to_document()?;
    let new_doc: IotaDocument = new_state.to_document()?;

    let diff_id: &MessageId = old_state.diff_message_id();

    let mut diff: DocumentDiff = DocumentDiff::new(&old_doc, &new_doc, *diff_id)?;

    let method: &TinyMethod = old_state.authentication()?;
    let location: &KeyLocation = method.location();

    old_state
      .sign_data(self.did(), self.storage(), location, &mut diff)
      .await?;

    let message: MessageId = if self.config.testmode {
      MessageId::null()
    } else {
      self
        .client_map
        .publish_diff(old_state.this_message_id(), &diff)
        .await?
        .into()
    };

    let events: [Event; 1] = [Event::new(EventData::DiffMessage(message))];

    Self::commit_events(self.did(), &self.config, self.storage(), &new_root, &events).await?;

    Ok(())
  }

  async fn fold_snapshot(snapshot: IdentitySnapshot, commit: Commit) -> Result<IdentitySnapshot> {
    Ok(IdentitySnapshot {
      sequence: commit.sequence().max(snapshot.sequence),
      identity: commit.into_event().apply(snapshot.identity).await?,
    })
  }

  #[doc(hidden)]
  pub async fn load_snapshot(&self) -> Result<IdentitySnapshot> {
    // Retrieve the state snapshot from storage or create a new one.
    let initial: IdentitySnapshot = self
      .storage()
      .snapshot(self.did())
      .await?
      .unwrap_or_else(|| IdentitySnapshot::new(IdentityState::new()));

    // Apply all recent events to the state and create a new snapshot
    self
      .storage()
      .stream(self.did(), initial.sequence())
      .await?
      .try_fold(initial, Self::fold_snapshot)
      .await
  }

  async fn load_snapshot_at(&self, generation: Generation) -> Result<IdentitySnapshot> {
    let initial: IdentitySnapshot = IdentitySnapshot::new(IdentityState::new());

    // Apply all events up to `generation`
    self
      .storage()
      .stream(self.did(), Generation::new())
      .await?
      .take(generation.to_u32() as usize)
      .try_fold(initial, Self::fold_snapshot)
      .await
  }

  async fn commit_events(
    did: &IotaDID,
    config: &AccountConfig,
    storage: &dyn Storage,
    state: &IdentitySnapshot,
    events: &[Event],
  ) -> Result<Vec<Commit>> {
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
      commits.push(Commit::new(did.clone(), sequence, event.clone()));
    }

    // Append the list of commits to the store
    storage.append(did, &commits).await?;

    // Store a snapshot every N events
    if sequence.to_u32() % config.milestone == 0 {
      let mut state: IdentitySnapshot = state.clone();

      // Fold the new commits into the snapshot
      for commit in commits.iter().cloned() {
        state = Self::fold_snapshot(state, commit).await?;
      }

      // Store the new snapshot
      storage.set_snapshot(did, &state).await?;
    }

    // Return the list of stored events
    Ok(commits)
  }

  async fn save(&self, force: bool) -> Result<()> {
    match self.config.autosave {
      AutoSave::Every => {
        self.storage().flush_changes().await?;
      }
      AutoSave::Batch(step) if force || (step != 0 && self.actions() % step == 0) => {
        self.storage().flush_changes().await?;
      }
      AutoSave::Batch(_) | AutoSave::Never => {}
    }

    Ok(())
  }

  /// Push all unpublished changes to the tangle in a single message.
  pub async fn publish_updates(&mut self) -> Result<()> {
    // Get the last commit generation that was published to the tangle.
    let last_published: Generation = self
      .storage()
      .published_generation(self.did())
      .await?
      .unwrap_or_default();

    // Get the commits that need to be published.
    let commits: Vec<Commit> = self.storage().collect(self.did(), last_published).await?;

    if commits.is_empty() {
      return Ok(());
    }

    // Load the snapshot that represents the state on the tangle.
    let snapshot: IdentitySnapshot = self.load_snapshot_at(last_published).await?;

    self.publish(snapshot, commits, true).await?;

    Ok(())
  }

  /// Publishes according to the autopublish configuration.
  async fn publish(&self, snapshot: IdentitySnapshot, commits: Vec<Commit>, force: bool) -> Result<()> {
    if !force && !self.config.autopublish {
      return Ok(());
    }

    match Publish::new(&commits) {
      Publish::Integration => self.process_integration_change(snapshot).await?,
      Publish::Diff => self.process_diff_change(snapshot).await?,
      Publish::None => {}
    }

    if !commits.is_empty() {
      let last_commit_generation: Generation = commits.last().unwrap().sequence();
      // Publishing adds an AuthMessage or DiffMessage event, that contains the message id
      // which is required to be set for subsequent updates.
      // The next snapshot that loads the tangle state will require this message id to be set.
      let generation: Generation = Generation::from_u32(last_commit_generation.to_u32() + 1);
      self.storage().set_published_generation(self.did(), generation).await?;
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
      let _ = executor::block_on(self.storage().flush_changes());
    }
  }
}

// =============================================================================
// Publish
// =============================================================================

#[derive(Clone, Copy, Debug)]
enum Publish {
  None,
  Integration,
  Diff,
}

impl Publish {
  fn new(commits: &[Commit]) -> Self {
    commits.iter().fold(Self::None, Self::apply)
  }

  const fn apply(self, commit: &Commit) -> Self {
    match (self, commit.event().data()) {
      (Self::Integration, _) => Self::Integration,
      (_, EventData::IdentityCreated(..)) => Self::Integration,
      (_, EventData::IntegrationMessage(_)) => self,
      (_, EventData::DiffMessage(_)) => self,
      (_, _) => Self::Diff,
    }
  }
}
