// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;
use std::ops::Deref;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use identity_account_storage::identity::ChainState;
use identity_account_storage::identity::IdentityState;
use identity_account_storage::storage::Storage;
use identity_account_storage::types::KeyLocation;
use identity_core::crypto::SetSignature;
use identity_core::crypto::SignatureOptions;
use identity_iota::chain::DocumentChain;
use identity_iota::document::ResolvedIotaDocument;
use identity_iota::tangle::Client;
use identity_iota::tangle::PublishType;
use identity_iota::tangle::SharedPtr;
use identity_iota_core::did::IotaDID;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::document::IotaVerificationMethod;
use identity_iota_core::tangle::MessageId;
use identity_iota_core::tangle::MessageIdExt;
use serde::Serialize;

use crate::account::AccountBuilder;
use crate::account::PublishOptions;
use crate::identity::IdentitySetup;
use crate::identity::IdentityUpdater;
use crate::updates::create_identity;
use crate::updates::Update;
use crate::Error;
use crate::Result;

use super::config::AccountSetup;
use super::config::AutoSave;
use super::AccountConfig;

/// An account manages one identity.
///
/// It handles private keys, writing to storage and
/// publishing to the Tangle.
#[derive(Debug)]
pub struct Account<C = Arc<Client>>
where
  C: SharedPtr<Client>,
{
  config: AccountConfig,
  storage: Arc<dyn Storage>,
  client: C,
  actions: AtomicUsize,
  chain_state: ChainState,
  state: IdentityState,
}

impl<C> Account<C>
where
  C: SharedPtr<Client>,
{
  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Creates a new [AccountBuilder].
  pub fn builder() -> AccountBuilder<C> {
    AccountBuilder::new()
  }

  /// Creates a new `Account` instance with the given `config`.
  async fn with_setup(setup: AccountSetup<C>, chain_state: ChainState, state: IdentityState) -> Result<Self> {
    Ok(Self {
      config: setup.config,
      storage: setup.storage,
      client: setup.client,
      actions: AtomicUsize::new(0),
      chain_state,
      state,
    })
  }

  /// Creates a new identity and returns an [`Account`] instance to manage it.
  ///
  /// The identity is stored locally in the [`Storage`] given in [`AccountSetup`]. The DID network
  /// is automatically determined by the [`Client`] used to publish it.
  ///
  /// See [`IdentitySetup`] to customize the identity creation.
  pub(crate) async fn create_identity(account_setup: AccountSetup<C>, identity_setup: IdentitySetup) -> Result<Self> {
    let state: IdentityState = create_identity(
      identity_setup,
      account_setup.client.deref().network().name(),
      account_setup.storage.deref(),
    )
    .await?;

    let mut account = Self::with_setup(account_setup, ChainState::new(), state).await?;

    account.store_state().await?;

    account.publish_internal(false, PublishOptions::default()).await?;

    Ok(account)
  }

  /// Creates an [`Account`] for an existing identity, if it exists in the [`Storage`].
  ///
  /// # Warning
  ///
  /// Callers are expected **not** to load the same [`IotaDID`] into more than one account,
  /// as that would cause race conditions when updating the identity.
  pub(crate) async fn load_identity(setup: AccountSetup<C>, did: IotaDID) -> Result<Self> {
    // Ensure the DID matches the client network.
    if did.network_str() != setup.client.network().name_str() {
      return Err(Error::IotaError(identity_iota::Error::IncompatibleNetwork(format!(
        "DID network {} does not match account network {}",
        did.network_str(),
        setup.client.network().name_str()
      ))));
    }

    // Ensure the did exists in storage
    let state = setup.storage.state(&did).await?.ok_or(Error::IdentityNotFound)?;
    let chain_state = setup.storage.chain_state(&did).await?.ok_or(Error::IdentityNotFound)?;

    Self::with_setup(setup, chain_state, state).await
  }

  // ===========================================================================
  // Getters & Setters
  // ===========================================================================

  /// Returns a reference counter to the [Storage] implementation.
  pub fn storage(&self) -> &Arc<dyn Storage> {
    &self.storage
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

  /// Returns the did of the managed identity.
  pub fn did(&self) -> &IotaDID {
    self.document().id()
  }

  /// Return the latest state of the identity.
  pub(crate) fn state(&self) -> &IdentityState {
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

  /// Sets the [`ChainState`] for the identity this account manages, **without doing any validation**.
  ///
  /// # WARNING
  ///
  /// This method is dangerous and can easily corrupt the internal state,
  /// potentially making the identity unusable. Only call this if you fully
  /// understand the implications!
  pub fn set_chain_state_unchecked(&mut self, chain_state: ChainState) {
    self.chain_state = chain_state
  }

  // ===========================================================================
  // Identity
  // ===========================================================================

  /// Resolves the DID Document associated with this `Account` from the Tangle.
  pub async fn resolve_identity(&self) -> Result<ResolvedIotaDocument> {
    self.client.read_document(self.did()).await.map_err(Into::into)
  }

  /// Returns the [`IdentityUpdater`] for this identity.
  ///
  /// On this type, various operations can be executed
  /// that modify an identity, such as creating services or methods.
  pub fn update_identity(&mut self) -> IdentityUpdater<'_, C> {
    IdentityUpdater::new(self)
  }

  /// Overwrites the [`IotaDocument`] this account manages, **without doing any validation**.
  ///
  /// # WARNING
  ///
  /// This method is dangerous and can easily corrupt the internal state,
  /// potentially making the identity unusable. Only call this if you fully
  /// understand the implications!
  pub async fn update_document_unchecked(&mut self, document: IotaDocument) -> Result<()> {
    *self.state.document_mut() = document;

    self.increment_actions();

    self.publish_internal(false, PublishOptions::default()).await?;

    Ok(())
  }

  /// Removes the identity from the local storage entirely.
  ///
  /// Note: This will remove all associated document updates and key material - recovery is NOT POSSIBLE!
  pub async fn delete_identity(self) -> Result<()> {
    // Remove all associated keys and events
    self.storage().deref().purge(self.did()).await?;

    // Write the changes to disk
    self.save(false).await?;

    Ok(())
  }

  /// Signs `data` with the key specified by `fragment`.
  pub async fn sign<U>(&self, fragment: &str, data: &mut U, options: SignatureOptions) -> Result<()>
  where
    U: Serialize + SetSignature,
  {
    let state: &IdentityState = self.state();

    let method: &IotaVerificationMethod = state
      .document()
      .resolve_method(fragment, None)
      .ok_or(Error::DIDError(identity_did::Error::MethodNotFound))?;

    let location: KeyLocation = state.method_location(method.type_(), fragment.to_owned())?;

    state
      .sign_data(self.did(), self.storage().deref(), &location, data, options)
      .await?;

    Ok(())
  }

  /// Push all unpublished changes to the tangle in a single message.
  pub async fn publish(&mut self) -> Result<()> {
    self.publish_internal(true, PublishOptions::default()).await?;

    Ok(())
  }

  /// Push all unpublished changes to the Tangle in a single message, optionally choosing
  /// the signing key used or forcing an integration chain update.
  ///
  /// See [`PublishOptions`].
  pub async fn publish_with_options(&mut self, options: PublishOptions) -> Result<()> {
    self.publish_internal(true, options).await?;

    Ok(())
  }

  /// Fetches the latest changes from the tangle and **overwrites** the local document.
  ///
  /// If a DID is managed from distributed accounts, this should be called before making changes
  /// to the identity, to avoid publishing updates that would be ignored.
  pub async fn fetch_state(&mut self) -> Result<()> {
    let iota_did: &IotaDID = self.did();
    let mut document_chain: DocumentChain = self.client.read_document_chain(iota_did).await?;
    // Checks if the local document is up to date
    if document_chain.integration_message_id() == self.chain_state.last_integration_message_id()
      && (document_chain.diff().is_empty()
        || document_chain.diff_message_id() == self.chain_state.last_diff_message_id())
    {
      return Ok(());
    }
    // Overwrite the current state with the most recent document
    self
      .chain_state
      .set_last_integration_message_id(*document_chain.integration_message_id());
    self
      .chain_state
      .set_last_diff_message_id(*document_chain.diff_message_id());
    std::mem::swap(self.state.document_mut(), &mut document_chain.current_mut().document);
    self.increment_actions();
    self.store_state().await?;
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
    self
      .storage()
      .deref()
      .state(self.did())
      .await?
      .ok_or(Error::IdentityNotFound)
  }

  pub(crate) async fn process_update(&mut self, update: Update) -> Result<()> {
    let did = self.did().to_owned();
    update.process(&did, &mut self.state, self.storage.deref()).await?;

    self.increment_actions();

    self.publish_internal(false, PublishOptions::default()).await?;

    Ok(())
  }

  async fn sign_self(
    &self,
    old_state: &IdentityState,
    new_state: &IdentityState,
    signing_method_query: &Option<String>,
    document: &mut IotaDocument,
  ) -> Result<()> {
    let signing_state: &IdentityState = if self.chain_state().is_new_identity() {
      new_state
    } else {
      old_state
    };

    let signing_method: &IotaVerificationMethod = match signing_method_query {
      Some(fragment) => signing_state.document().try_resolve_signing_method(fragment)?,
      None => signing_state.document().default_signing_method()?,
    };

    let signing_key_location: KeyLocation = new_state.method_location(
      signing_method.type_(),
      // TODO: Should be a fatal error.
      signing_method
        .id()
        .fragment()
        .ok_or(Error::MethodMissingFragment)?
        .to_owned(),
    )?;

    signing_state
      .sign_data(
        self.did(),
        self.storage().deref(),
        &signing_key_location,
        document,
        SignatureOptions::default(),
      )
      .await?;

    Ok(())
  }

  /// Publishes according to the autopublish configuration.
  async fn publish_internal(&mut self, force: bool, options: PublishOptions) -> Result<()> {
    if !force && !self.config.autopublish {
      return Ok(());
    }

    if self.chain_state().is_new_identity() {
      // New identity
      self.publish_integration_change(None, &options.sign_with).await?;
    } else {
      // Existing identity
      let old_state: IdentityState = self.load_state().await?;
      let new_state: &IdentityState = self.state();

      let publish_type: Option<PublishType> = if options.force_integration_update {
        Some(PublishType::Integration)
      } else {
        PublishType::new(old_state.document(), new_state.document())
      };

      match publish_type {
        Some(PublishType::Integration) => {
          self
            .publish_integration_change(Some(&old_state), &options.sign_with)
            .await?;
        }
        Some(PublishType::Diff) => {
          self.publish_diff_change(&old_state, &options.sign_with).await?;
        }
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

  async fn publish_integration_change(
    &mut self,
    old_state: Option<&IdentityState>,
    signing_method_query: &Option<String>,
  ) -> Result<()> {
    log::debug!("[publish_integration_change] publishing {:?}", self.document().id());

    let new_state: &IdentityState = self.state();

    let mut new_doc: IotaDocument = new_state.document().to_owned();

    new_doc.metadata.previous_message_id = *self.chain_state().last_integration_message_id();

    self
      .sign_self(
        old_state.unwrap_or(new_state),
        new_state,
        signing_method_query,
        &mut new_doc,
      )
      .await?;

    log::debug!(
      "[publish_integration_change] publishing on index {}",
      new_doc.integration_index()
    );

    let message_id: MessageId = if self.config.testmode {
      // Fake publishing by returning a random message id.
      let mut bytes: [u8; 32] = [0; 32];
      crypto::utils::rand::fill(&mut bytes)?;
      MessageId::new(bytes)
    } else {
      self.client.publish_document(&new_doc).await?.into()
    };

    self.chain_state.set_last_integration_message_id(message_id);

    Ok(())
  }

  async fn publish_diff_change(
    &mut self,
    old_state: &IdentityState,
    signing_method_query: &Option<String>,
  ) -> Result<()> {
    log::debug!("[publish_diff_change] publishing {:?}", self.document().id());

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

    let signing_method: &IotaVerificationMethod = match signing_method_query {
      Some(fragment) => old_state.document().try_resolve_signing_method(fragment)?,
      None => old_state.document().default_signing_method()?,
    };

    let signing_key_location: KeyLocation = old_state.method_location(
      signing_method.type_(),
      // TODO: Should be a fatal error.
      signing_method
        .id()
        .fragment()
        .ok_or(Error::MethodMissingFragment)?
        .to_owned(),
    )?;

    old_state
      .sign_data(
        self.did(),
        self.storage().deref(),
        &signing_key_location,
        &mut diff,
        SignatureOptions::default(),
      )
      .await?;

    log::debug!(
      "[publish_diff_change] publishing on index {}",
      IotaDocument::diff_index(self.chain_state().last_integration_message_id())?
    );

    let message_id: MessageId = if self.config.testmode {
      // Fake publishing by returning a random message id.
      let mut bytes: [u8; 32] = [0; 32];
      crypto::utils::rand::fill(&mut bytes)?;
      MessageId::new(bytes)
    } else {
      self
        .client
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
        self.storage().deref().flush_changes().await?;
      }
      AutoSave::Batch(step) if force || (step != 0 && self.actions() % step == 0) => {
        self.storage().deref().flush_changes().await?;
      }
      AutoSave::Batch(_) | AutoSave::Never => {}
    }

    Ok(())
  }
}
