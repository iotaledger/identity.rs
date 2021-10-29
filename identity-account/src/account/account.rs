// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::executor;
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
  did: IotaDID, // TODO: Get from state field
  state: IdentityState,
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
    let state = setup.storage.state(&did).await?.ok_or(Error::IdentityNotFound)?;

    let did_lease = setup.storage.lease_did(&did).await?;

    Self::with_setup(setup, did, state, did_lease).await
  }

  /// Creates a new `Account` instance with the given `config`.
  async fn with_setup(setup: AccountSetup, did: IotaDID, state: IdentityState, did_lease: DIDLease) -> Result<Self> {
    Ok(Self {
      config: setup.config,
      storage: setup.storage,
      client_map: setup.client_map,
      actions: AtomicUsize::new(0),
      state,
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
  pub fn state(&self) -> &IdentityState {
    &self.state
  }

  /// Resolves the DID Document associated with this `Account` from the Tangle.
  pub async fn resolve_identity(&self) -> Result<IotaDocument> {
    // Fetch the DID Document from the Tangle
    self.client_map.resolve(self.did()).await.map_err(Into::into)
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

    let state = IdentityState::new();

    // TODO: Pass state mutably and let process work on it
    let (did, did_lease, events): (IotaDID, DIDLease, Vec<Event>) = command
      .process(state.integration_generation(), setup.storage.as_ref())
      .await?;

    let account = Self::with_setup(setup, did, state, did_lease).await?;

    // TODO: This should also write state to storage (remove me)
    account.publish(true).await?;

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
    let state: &IdentityState = self.state();

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
    let state: &IdentityState = self.state();

    debug!("[Account::process] Root = {:#?}", state);

    // Process the command with a read-only view of the state
    let context: Context<'_> = Context::new(self.did(), state, self.storage());
    let events: Option<Vec<Event>> = command.process(context).await?;

    debug!("[Account::process] Events = {:#?}", events);

    // TODO: publish

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

  async fn process_integration_change(&self) -> Result<()> {
    let old_state: IdentityState = self.load_state().await?;
    let new_state: &IdentityState = self.state();

    let mut new_doc: IotaDocument = new_state.to_document()?;

    self.sign_document(&old_state, new_state, &mut new_doc).await?;

    let message: MessageId = if self.config.testmode {
      MessageId::null()
    } else {
      self.client_map.publish_document(&new_doc).await?.into()
    };

    Ok(())
  }

  async fn process_diff_change(&self) -> Result<()> {
    let old_state: IdentityState = self.load_state().await?;
    let new_state: &IdentityState = self.state();

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

    Ok(())
  }

  #[doc(hidden)]
  pub async fn load_state(&self) -> Result<IdentityState> {
    // Retrieve the state from storage or create a new one.
    Ok(
      self
        .storage()
        .state(self.did())
        .await?
        .unwrap_or_else(|| IdentityState::new()),
    )
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
    self.publish(true).await?;

    Ok(())
  }

  /// Publishes according to the autopublish configuration.
  async fn publish(&self, force: bool) -> Result<()> {
    if !force && !self.config.autopublish {
      return Ok(());
    }

    // TODO: Figure out if int or diff without commits
    // match Publish::new(&commits) {
    //   Publish::Integration => self.process_integration_change().await?,
    //   Publish::Diff => self.process_diff_change().await?,
    //   Publish::None => {}
    // }

    self.storage.set_state(self.did(), self.state()).await?;

    // TODO: true?
    self.save(false).await?;

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
