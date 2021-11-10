// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::executor;

use identity_core::crypto::SetSignature;
use identity_iota::did::DocumentDiff;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;
use identity_iota::did::IotaVerificationMethod;
use identity_iota::tangle::Client;
use identity_iota::tangle::ClientMap;
use identity_iota::tangle::MessageId;
use identity_iota::tangle::Publish;
use identity_iota::tangle::TangleRef;
use identity_iota::tangle::TangleResolve;
use serde::Serialize;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::account::AccountBuilder;
use crate::error::Result;
use crate::events::create_identity;
use crate::events::Update;
use crate::identity::DIDLease;
use crate::identity::IdentitySetup;
use crate::identity::IdentityState;
use crate::identity::IdentityUpdater;
use crate::storage::Storage;
use crate::types::KeyLocation;
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
  state: IdentityState,
  did_lease: DIDLease,
}

impl Account {
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

  /// Returns whether save-on-drop is enabled.
  pub fn dropsave(&self) -> bool {
    self.config.dropsave
  }

  /// Returns the total number of actions executed by this instance.
  pub fn actions(&self) -> usize {
    self.actions.load(Ordering::SeqCst)
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

  // TODO: Make pub?
  pub(crate) fn state_mut_unchecked(&mut self) -> &mut IdentityState {
    &mut self.state
  }

  /// Returns the DID document of the identity, which this account manages,
  /// with all updates applied.
  pub fn document(&self) -> &IotaDocument {
    self.state.as_document()
  }

  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Creates a new [AccountBuilder].
  pub fn builder() -> AccountBuilder {
    AccountBuilder::new()
  }

  /// Creates a new `Account` instance with the given `config`.
  async fn with_setup(setup: AccountSetup, state: IdentityState, did_lease: DIDLease) -> Result<Self> {
    Ok(Self {
      config: setup.config,
      storage: setup.storage,
      client_map: setup.client_map,
      actions: AtomicUsize::new(0),
      state,
      did_lease,
    })
  }

  /// Creates a new identity and returns an [`Account`] instance to manage it.
  /// The identity is stored locally in the [`Storage`] given in [`AccountSetup`], and published
  /// using the [`ClientMap`].
  ///
  /// See [`IdentityCreate`] to customize the identity creation.
  pub(crate) async fn create_identity(setup: AccountSetup, input: IdentitySetup) -> Result<Self> {
    let (did_lease, state): (DIDLease, IdentityState) = create_identity(input, setup.storage.as_ref()).await?;

    let mut account = Self::with_setup(setup, state, did_lease).await?;

    account.store_state().await?;

    account.publish(false).await?;

    Ok(account)
  }

  /// Creates an [`Account`] for an existing identity, if it exists in the [`Storage`].
  pub(crate) async fn load_identity(setup: AccountSetup, did: IotaDID) -> Result<Self> {
    // Ensure the did exists in storage
    let state = setup.storage.state(&did).await?.ok_or(Error::IdentityNotFound)?;

    let did_lease = setup.storage.lease_did(&did).await?;

    Self::with_setup(setup, state, did_lease).await
  }

  // ===========================================================================
  // Identity
  // ===========================================================================

  /// Resolves the DID Document associated with this `Account` from the Tangle.
  pub async fn resolve_identity(&self) -> Result<IotaDocument> {
    // Fetch the DID Document from the Tangle
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
      .as_document()
      .resolve_method(fragment)
      .ok_or(Error::MethodNotFound)?;

    let location: KeyLocation = state
      .method_location(method.key_type(), fragment.to_owned())
      .expect("TODO: fatal error");

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
    // TODO: An account always holds a valid identity (after creation!),
    // so if None is returned, that's a broken invariant -> should be a fatal error.
    self.storage().state(self.did()).await?.ok_or(Error::IdentityNotFound)
  }

  pub(crate) async fn process_update(&mut self, update: Update, _persist: bool) -> Result<()> {
    let did = self.did().to_owned();
    let storage = Arc::clone(&self.storage);

    update
      .process(&did, self.state_mut_unchecked(), storage.as_ref())
      .await?;

    self.publish(false).await?;

    Ok(())
  }

  async fn sign_self(
    &self,
    old_state: &IdentityState,
    new_state: &IdentityState,
    document: &mut IotaDocument,
  ) -> Result<()> {
    if new_state.is_new_identity() {
      // TODO: Replace with: let method: &TinyMethod = new_state.capability_invocation()?;
      let method: &IotaVerificationMethod = new_state.as_document().default_signing_method()?;
      let location: KeyLocation = new_state
        .method_location(
          method.key_type(),
          method
            .id()
            .fragment()
            .expect("verification method did not have a fragment")
            .to_owned(),
        )
        .expect("TODO: fatal error");

      // Sign the DID Document with the current capability invocation method
      new_state
        .sign_data(self.did(), self.storage(), &location, document)
        .await?;
    } else {
      // TODO: Replace with: let method: &TinyMethod = new_state.capability_invocation()?;
      let method: &IotaVerificationMethod = old_state.as_document().default_signing_method()?;
      // TODO: Fatal error if not found
      let location: KeyLocation = new_state.method_location(
        method.key_type(),
        method
          .id()
          .fragment()
          .expect("verification method did not have a fragment")
          .to_owned(),
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

    let old_state: IdentityState = self.load_state().await?;
    let new_state: &IdentityState = self.state();

    if new_state.is_new_identity() {
      // New identity
      self.publish_integration_change(None).await?;
    } else {
      // Existing identity
      match Publish::new(old_state.as_document(), new_state.as_document()) {
        Publish::Integration => self.publish_integration_change(Some(&old_state)).await?,
        Publish::Diff => self.publish_diff_change(&old_state).await?,
        Publish::None => {}
      }
    }

    self.state_mut_unchecked().increment_generation()?;

    self.store_state().await?;

    Ok(())
  }

  async fn store_state(&self) -> Result<()> {
    self.storage.set_state(self.did(), self.state()).await?;

    self.save(false).await?;

    Ok(())
  }

  async fn publish_integration_change(&mut self, old_state: Option<&IdentityState>) -> Result<()> {
    let new_state: &IdentityState = self.state();

    let mut new_doc: IotaDocument = new_state.as_document().to_owned();

    self
      .sign_self(old_state.unwrap_or(new_state), new_state, &mut new_doc)
      .await?;

    let message_id: MessageId = if self.config.testmode {
      MessageId::null()
    } else {
      self.client_map.publish_document(&new_doc).await?.into()
    };

    self.state_mut_unchecked().set_integration_message_id(message_id);

    Ok(())
  }

  async fn publish_diff_change(&mut self, old_state: &IdentityState) -> Result<()> {
    let new_state: &IdentityState = self.state();

    let old_doc: IotaDocument = old_state.as_document().to_owned();
    let new_doc: IotaDocument = new_state.as_document().to_owned();

    let diff_id: &MessageId = old_state.diff_message_id();

    let mut diff: DocumentDiff = DocumentDiff::new(&old_doc, &new_doc, *diff_id)?;

    // TODO: Replace with: let method: &TinyMethod = new_state.capability_invocation()?;
    let method: &IotaVerificationMethod = old_state.as_document().default_signing_method()?;

    let location: KeyLocation = old_state.method_location(
      method.key_type(),
      method
        .id()
        .fragment()
        .expect("verification method did not have a fragment")
        .to_owned(),
    )?;

    old_state
      .sign_data(self.did(), self.storage(), &location, &mut diff)
      .await?;

    let message_id: MessageId = if self.config.testmode {
      MessageId::null()
    } else {
      self
        .client_map
        .publish_diff(old_state.this_message_id(), &diff)
        .await?
        .into()
    };

    self.state_mut_unchecked().set_diff_message_id(message_id);

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

impl Drop for Account {
  fn drop(&mut self) {
    if self.config.dropsave && self.actions() != 0 {
      // TODO: Handle Result (?)
      let _ = executor::block_on(self.storage().flush_changes());
    }
  }
}
