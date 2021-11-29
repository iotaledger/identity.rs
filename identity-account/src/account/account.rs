// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::SetSignature;
use identity_iota::did::IotaDID;
use identity_iota::document::DiffMessage;
use identity_iota::document::IotaDocument;
use identity_iota::document::IotaVerificationMethod;
use identity_iota::tangle::Client;
use identity_iota::tangle::ClientMap;
use identity_iota::tangle::MessageId;
use identity_iota::tangle::MessageIdExt;
use identity_iota::tangle::PublishType;
use identity_iota::tangle::TangleRef;
use identity_iota::tangle::TangleResolve;
use serde::Serialize;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::account::AccountBuilder;
use crate::error::Result;
use crate::identity::ChainState;
use crate::identity::DIDLease;
use crate::identity::IdentitySetup;
use crate::identity::IdentityState;
use crate::identity::IdentityUpdater;
use crate::storage::Storage;
use crate::types::KeyLocation;
use crate::updates::create_identity;
use crate::updates::Update;
use crate::Error;

use super::config::AccountSetup;
use super::config::AutoSave;
use super::AccountConfig;

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
  chain_state: ChainState,
  state: IdentityState,
  did_lease: DIDLease,
}

impl Account {
  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Creates a new [AccountBuilder].
  pub fn builder() -> AccountBuilder {
    AccountBuilder::new()
  }

  /// Creates a new `Account` instance with the given `config`.
  async fn with_setup(
    setup: AccountSetup,
    chain_state: ChainState,
    state: IdentityState,
    did_lease: DIDLease,
  ) -> Result<Self> {
    Ok(Self {
      config: setup.config,
      storage: setup.storage,
      client_map: setup.client_map,
      actions: AtomicUsize::new(0),
      chain_state,
      state,
      did_lease,
    })
  }

  /// Creates a new identity and returns an [`Account`] instance to manage it.
  /// The identity is stored locally in the [`Storage`] given in [`AccountSetup`], and published
  /// using the [`ClientMap`].
  ///
  /// See [`IdentitySetup`] to customize the identity creation.
  pub(crate) async fn create_identity(setup: AccountSetup, input: IdentitySetup) -> Result<Self> {
    let (did_lease, state): (DIDLease, IdentityState) = create_identity(input, setup.storage.as_ref()).await?;

    let mut account = Self::with_setup(setup, ChainState::new(), state, did_lease).await?;

    account.store_state().await?;

    account.publish(false).await?;

    Ok(account)
  }

  /// Creates an [`Account`] for an existing identity, if it exists in the [`Storage`].
  pub(crate) async fn load_identity(setup: AccountSetup, did: IotaDID) -> Result<Self> {
    // Ensure the did exists in storage
    let state = setup.storage.state(&did).await?.ok_or(Error::IdentityNotFound)?;
    let chain_state = setup.storage.chain_state(&did).await?.ok_or(Error::IdentityNotFound)?;

    let did_lease = setup.storage.lease_did(&did).await?;

    Self::with_setup(setup, chain_state, state, did_lease).await
  }

  // ===========================================================================
  // Getters & Setters
  // ===========================================================================

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

  /// Returns the total number of actions executed by this instance.
  pub fn actions(&self) -> usize {
    self.actions.load(Ordering::SeqCst)
  }

  /// Increments the total number of actions executed by this instance.
  fn increment_actions(&self) {
    self.actions.fetch_add(1, Ordering::SeqCst);
  }

  /// Adds a pre-configured `Client` for Tangle interactions.
  pub fn set_client(&self, client: Client) {
    self.client_map.insert(client);
  }

  /// Returns the did of the managed identity.
  pub fn did(&self) -> &IotaDID {
    self.document().did()
  }

  /// Return the latest state of the identity.
  pub fn state(&self) -> &IdentityState {
    &self.state
  }

  /// Return the chain state of the identity.
  pub fn chain_state(&self) -> &ChainState {
    &self.chain_state
  }

  /// Returns the DID document of the identity, which this account manages,
  /// with all updates applied.
  pub fn document(&self) -> &IotaDocument {
    self.state.document()
  }

  // ===========================================================================
  // Identity
  // ===========================================================================

  /// Resolves the DID Document associated with this `Account` from the Tangle.
  pub async fn resolve_identity(&self) -> Result<IotaDocument> {
    self.client_map.resolve(self.did()).await.map_err(Into::into)
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

    let method: &IotaVerificationMethod = state
      .document()
      .resolve_method(fragment)
      .ok_or(Error::DIDError(identity_did::Error::MethodNotFound))?;

    let location: KeyLocation = state.method_location(method.key_type(), fragment.to_owned())?;

    state.sign_data(self.did(), self.storage(), &location, target).await?;

    Ok(())
  }

  /// Push all unpublished changes to the tangle in a single message.
  pub async fn publish_updates(&mut self) -> Result<()> {
    self.publish(true).await?;

    Ok(())
  }

  // ===========================================================================
  // Misc. Private
  // ===========================================================================

  #[doc(hidden)]
  pub async fn load_state(&self) -> Result<IdentityState> {
    // TODO: An account always holds a valid identity,
    // so if None is returned, that's a broken invariant.
    // This should be mapped to a fatal error in the future.
    self.storage().state(self.did()).await?.ok_or(Error::IdentityNotFound)
  }

  pub(crate) async fn process_update(&mut self, update: Update) -> Result<()> {
    let did = self.did().to_owned();
    let storage = Arc::clone(&self.storage);

    update.process(&did, &mut self.state, storage.as_ref()).await?;

    self.increment_actions();

    self.publish(false).await?;

    Ok(())
  }

  async fn sign_self(
    &self,
    old_state: &IdentityState,
    new_state: &IdentityState,
    document: &mut IotaDocument,
  ) -> Result<()> {
    if self.chain_state().is_new_identity() {
      let method: &IotaVerificationMethod = new_state.document().default_signing_method()?;
      let location: KeyLocation = new_state.method_location(
        method.key_type(),
        // TODO: Should be a fatal error.
        method.id().fragment().ok_or(Error::MethodMissingFragment)?.to_owned(),
      )?;

      // Sign the DID Document with the current capability invocation method
      new_state
        .sign_data(self.did(), self.storage(), &location, document)
        .await?;
    } else {
      let method: &IotaVerificationMethod = old_state.document().default_signing_method()?;
      let location: KeyLocation = new_state.method_location(
        method.key_type(),
        // TODO: Should be a fatal error.
        method.id().fragment().ok_or(Error::MethodMissingFragment)?.to_owned(),
      )?;

      // Sign the DID Document with the previous capability invocation method
      old_state
        .sign_data(self.did(), self.storage(), &location, document)
        .await?;
    }

    Ok(())
  }

  /// Publishes according to the autopublish configuration.
  async fn publish(&mut self, force: bool) -> Result<()> {
    if !force && !self.config.autopublish {
      return Ok(());
    }

    if self.chain_state().is_new_identity() {
      // New identity
      self.publish_integration_change(None).await?;
    } else {
      // Existing identity
      let old_state: IdentityState = self.load_state().await?;
      let new_state: &IdentityState = self.state();

      match PublishType::new(old_state.document(), new_state.document()) {
        Some(PublishType::Integration) => self.publish_integration_change(Some(&old_state)).await?,
        Some(PublishType::Diff) => self.publish_diff_change(&old_state).await?,
        None => {
          // Can return early, as there is nothing new to publish or store.
          return Ok(());
        }
      }
    }

    self.state.increment_generation()?;

    self.store_state().await?;

    Ok(())
  }

  async fn store_state(&self) -> Result<()> {
    self.storage.set_state(self.did(), self.state()).await?;
    self.storage.set_chain_state(self.did(), self.chain_state()).await?;

    self.save(false).await?;

    Ok(())
  }

  async fn publish_integration_change(&mut self, old_state: Option<&IdentityState>) -> Result<()> {
    log::debug!("[publish_integration_change] publishing {:?}", self.document().did());

    let new_state: &IdentityState = self.state();

    let mut new_doc: IotaDocument = new_state.document().to_owned();

    new_doc.set_previous_message_id(*self.chain_state().last_integration_message_id());

    self
      .sign_self(old_state.unwrap_or(new_state), new_state, &mut new_doc)
      .await?;

    log::debug!(
      "[publish_integration_change] publishing on index {}",
      new_doc.integration_index()
    );

    let message_id: MessageId = if self.config.testmode {
      // Fake publishing by returning a random message id.
      MessageId::new(unsafe { crypto::utils::rand::gen::<[u8; 32]>().unwrap() })
    } else {
      self.client_map.publish_document(&new_doc).await?.into()
    };

    self.chain_state.set_last_integration_message_id(message_id);

    Ok(())
  }

  async fn publish_diff_change(&mut self, old_state: &IdentityState) -> Result<()> {
    log::debug!("[publish_diff_change] publishing {:?}", self.document().did());

    let old_doc: &IotaDocument = old_state.document();
    let new_doc: &IotaDocument = self.state().document();

    let mut previous_message_id: &MessageId = self.chain_state().last_diff_message_id();

    // If there was no previous diff message, use the previous int message.
    if previous_message_id.is_null() {
      if !self.chain_state.last_integration_message_id().is_null() {
        previous_message_id = self.chain_state.last_integration_message_id();
      } else {
        // TODO: Return a fatal error about the invalid chain state.
      }
    }

    let mut diff: DiffMessage = DiffMessage::new(old_doc, new_doc, *previous_message_id)?;

    let method: &IotaVerificationMethod = old_state.document().default_signing_method()?;

    let location: KeyLocation = old_state.method_location(
      method.key_type(),
      // TODO: Should be a fatal error.
      method.id().fragment().ok_or(Error::MethodMissingFragment)?.to_owned(),
    )?;

    old_state
      .sign_data(self.did(), self.storage(), &location, &mut diff)
      .await?;

    log::debug!(
      "[publish_diff_change] publishing on index {}",
      IotaDocument::diff_index(self.chain_state().last_integration_message_id())?
    );

    let message_id: MessageId = if self.config.testmode {
      // Fake publishing by returning a random message id.
      MessageId::new(unsafe { crypto::utils::rand::gen::<[u8; 32]>().unwrap() })
    } else {
      self
        .client_map
        .publish_diff(self.chain_state().last_integration_message_id(), &diff)
        .await?
        .into()
    };

    self.chain_state.set_last_diff_message_id(message_id);

    Ok(())
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
}
