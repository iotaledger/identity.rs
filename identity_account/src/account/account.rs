// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;
use std::ops::Deref;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use identity_account_storage::crypto::RemoteEd25519;
use identity_account_storage::crypto::RemoteKey;
use identity_account_storage::identity::ChainState;
use identity_account_storage::storage::Storage;
#[cfg(feature = "encryption")]
use identity_account_storage::types::CekAlgorithm;
#[cfg(feature = "encryption")]
use identity_account_storage::types::EncryptedData;
#[cfg(feature = "encryption")]
use identity_account_storage::types::EncryptionAlgorithm;
use identity_account_storage::types::KeyLocation;
use identity_core::crypto::KeyType;
use identity_core::crypto::ProofOptions;
use identity_core::crypto::PublicKey;
use identity_core::crypto::SetSignature;
use identity_did::did::DID;
use identity_iota_client::chain::DocumentChain;
use identity_iota_client::document::ResolvedIotaDocument;
use identity_iota_client::tangle::Client;
use identity_iota_client::tangle::PublishType;
use identity_iota_client::tangle::SharedPtr;
use identity_iota_core::did::IotaDID;
use identity_iota_core::did::IotaDIDUrl;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::document::IotaVerificationMethod;
use identity_iota_core::tangle::MessageId;
use identity_iota_core::tangle::MessageIdExt;
use serde::Serialize;

use crate::account::AccountBuilder;
use crate::account::PublishOptions;
use crate::types::IdentitySetup;
use crate::types::IdentityUpdater;
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
  document: IotaDocument,
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
  async fn with_setup(setup: AccountSetup<C>, chain_state: ChainState, document: IotaDocument) -> Result<Self> {
    Ok(Self {
      config: setup.config,
      storage: setup.storage,
      client: setup.client,
      actions: AtomicUsize::new(0),
      chain_state,
      document,
    })
  }

  /// Creates a new identity and returns an [`Account`] instance to manage it.
  ///
  /// The identity is stored locally in the [`Storage`] given in [`AccountSetup`]. The DID network
  /// is automatically determined by the [`Client`] used to publish it.
  ///
  /// See [`IdentitySetup`] to customize the identity creation.
  pub(crate) async fn create_identity(account_setup: AccountSetup<C>, identity_setup: IdentitySetup) -> Result<Self> {
    let document: IotaDocument = create_identity(
      identity_setup,
      account_setup.client.deref().network().name(),
      account_setup.storage.deref(),
    )
    .await?;

    let mut account = Self::with_setup(account_setup, ChainState::new(), document).await?;

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
      return Err(Error::IotaClientError(
        identity_iota_client::Error::IncompatibleNetwork(format!(
          "DID network {} does not match account network {}",
          did.network_str(),
          setup.client.network().name_str()
        )),
      ));
    }

    // Ensure the identity exists in storage
    let document: IotaDocument = setup.storage.document_get(&did).await?.ok_or(Error::IdentityNotFound)?;
    let chain_state: ChainState = setup
      .storage
      .chain_state_get(&did)
      .await?
      .ok_or(Error::IdentityNotFound)?;

    Self::with_setup(setup, chain_state, document).await
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

  /// Return the chain state of the identity.
  pub fn chain_state(&self) -> &ChainState {
    &self.chain_state
  }

  /// Returns the DID document of the identity, which this account manages,
  /// with all updates applied.
  ///
  /// Note: the returned document only has a valid signature after publishing an integration chain update.
  /// In general, for use cases where the signature is required, it is advisable to resolve the
  /// document from the Tangle.
  pub fn document(&self) -> &IotaDocument {
    &self.document
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
    self.document = document;

    self.increment_actions();

    self.publish_internal(false, PublishOptions::default()).await?;

    Ok(())
  }

  /// Removes the identity from the local storage entirely.
  ///
  /// Note: This will remove all associated document updates and key material - recovery is NOT POSSIBLE!
  pub async fn delete_identity(self) -> Result<()> {
    // Remove all associated keys and events
    self.storage().did_purge(self.did()).await?;

    // Write the changes to disk
    self.save(false).await?;

    Ok(())
  }

  /// Signs `data` with the key specified by `fragment`.
  pub async fn sign<U>(&self, fragment: &str, data: &mut U, options: ProofOptions) -> Result<()>
  where
    U: Serialize + SetSignature,
  {
    let method: &IotaVerificationMethod = self
      .document()
      .resolve_method(fragment, None)
      .ok_or(Error::DIDError(identity_did::Error::MethodNotFound))?;

    self
      .remote_sign_data(self.document().id(), method, data, options)
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

  /// Fetches the latest document from the tangle and **overwrites** the local document.
  ///
  /// If a DID is managed from distributed accounts, this should be called before making changes
  /// to the identity, to avoid publishing updates that would be ignored.
  pub async fn fetch_document(&mut self) -> Result<()> {
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

    std::mem::swap(&mut self.document, &mut document_chain.current_mut().document);

    self.increment_actions();
    self.store_state().await?;
    Ok(())
  }

  /// If the document has a [`RevocationBitmap`][identity_did::revocation::RevocationBitmap] service identified by
  /// `fragment`, revoke all credentials with a `revocationBitmapIndex` in `credential_indices`.
  pub async fn revoke_credentials(&mut self, fragment: &str, credential_indices: &[u32]) -> Result<()> {
    // Find the service to be updated.
    let mut service_id: IotaDIDUrl = self.did().to_url();
    service_id.set_fragment(Some(fragment))?;

    self.document.revoke_credentials(&service_id, credential_indices)?;

    self.increment_actions();
    self.publish_internal(false, PublishOptions::default()).await?;
    Ok(())
  }

  /// If the document has a [`RevocationBitmap`][identity_did::revocation::RevocationBitmap] service identified by
  /// `fragment`, unrevoke all credentials with a `revocationBitmapIndex` in `credential_indices`.
  pub async fn unrevoke_credentials(&mut self, fragment: &str, credential_indices: &[u32]) -> Result<()> {
    // Find the service to be updated.
    let mut service_id: IotaDIDUrl = self.did().to_url();
    service_id.set_fragment(Some(fragment))?;

    self.document.unrevoke_credentials(&service_id, credential_indices)?;

    self.increment_actions();
    self.publish_internal(false, PublishOptions::default()).await?;
    Ok(())
  }

  /// Encrypts the given `plaintext` with the specified `encryption_algorithm` and `cek_algorithm`.
  ///
  /// Returns an [`EncryptedData`] instance.
  #[cfg(feature = "encryption")]
  pub async fn encrypt_data(
    &self,
    plaintext: &[u8],
    associated_data: &[u8],
    encryption_algorithm: &EncryptionAlgorithm,
    cek_algorithm: &CekAlgorithm,
    public_key: PublicKey,
  ) -> Result<EncryptedData> {
    self
      .storage()
      .data_encrypt(
        self.did(),
        plaintext.to_vec(),
        associated_data.to_vec(),
        encryption_algorithm,
        cek_algorithm,
        public_key,
      )
      .await
      .map_err(Into::into)
  }

  /// Decrypts the given `data` with the key identified by `fragment` using the given `encryption_algorithm` and
  /// `cek_algorithm`.
  ///
  /// Returns the decrypted text.
  #[cfg(feature = "encryption")]
  pub async fn decrypt_data(
    &self,
    data: EncryptedData,
    encryption_algorithm: &EncryptionAlgorithm,
    cek_algorithm: &CekAlgorithm,
    fragment: &str,
  ) -> Result<Vec<u8>> {
    let method: &IotaVerificationMethod = self
      .document()
      .resolve_method(fragment, None)
      .ok_or(Error::DIDError(identity_did::Error::MethodNotFound))?;
    let private_key: KeyLocation = KeyLocation::from_verification_method(method)?;
    self
      .storage()
      .data_decrypt(self.did(), data, encryption_algorithm, cek_algorithm, &private_key)
      .await
      .map_err(Into::into)
  }

  // ===========================================================================
  // Misc. Private
  // ===========================================================================

  pub(crate) async fn load_document(&self) -> Result<IotaDocument> {
    // TODO: An account always holds a valid identity,
    // so if None is returned, that's a broken invariant.
    // This should be mapped to a fatal error in the future.
    self
      .storage()
      .deref()
      .document_get(self.did())
      .await?
      .ok_or(Error::IdentityNotFound)
  }

  pub(crate) async fn process_update(&mut self, update: Update) -> Result<()> {
    let did = self.did().to_owned();
    update.process(&did, &mut self.document, self.storage.deref()).await?;

    self.increment_actions();

    self.publish_internal(false, PublishOptions::default()).await?;

    Ok(())
  }

  async fn sign_self(
    &self,
    old_doc: &IotaDocument,
    new_doc: &IotaDocument,
    signing_method_query: &Option<String>,
    document: &mut IotaDocument,
  ) -> Result<()> {
    let signing_doc: &IotaDocument = if self.chain_state().is_new_identity() {
      new_doc
    } else {
      old_doc
    };

    let signing_method: &IotaVerificationMethod = match signing_method_query {
      Some(fragment) => signing_doc.resolve_signing_method(fragment)?,
      None => signing_doc.default_signing_method()?,
    };

    self
      .remote_sign_data(signing_doc.id(), signing_method, document, ProofOptions::default())
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
      let old_doc: IotaDocument = self.load_document().await?;
      let new_doc: &IotaDocument = self.document();

      // NOTE: always publish an integration update (if needed); diff chain slated for removal.
      let publish_type: Option<PublishType> = if options.force_integration_update {
        Some(PublishType::Integration)
      } else if let Some(publish_type) = PublishType::new(&old_doc, new_doc) {
        if self.config.testmode {
          // Allow tests to pass as normal.
          Some(publish_type)
        } else {
          Some(PublishType::Integration)
        }
      } else {
        None
      };

      match publish_type {
        Some(PublishType::Integration) => {
          self
            .publish_integration_change(Some(&old_doc), &options.sign_with)
            .await?;
        }
        Some(PublishType::Diff) => {
          self.publish_diff_change(&old_doc, &options.sign_with).await?;
        }
        None => {
          // Can return early, as there is nothing new to publish or store.
          return Ok(());
        }
      }
    }

    self.store_state().await?;

    Ok(())
  }

  async fn store_state(&self) -> Result<()> {
    self.storage.document_set(self.did(), &self.document).await?;
    self.storage.chain_state_set(self.did(), self.chain_state()).await?;

    self.save(false).await?;

    Ok(())
  }

  async fn publish_integration_change(
    &mut self,
    old_doc: Option<&IotaDocument>,
    signing_method_query: &Option<String>,
  ) -> Result<()> {
    log::debug!("[publish_integration_change] publishing {:?}", self.document().id());

    let new_doc_ref: &IotaDocument = self.document();
    let mut new_doc: IotaDocument = new_doc_ref.to_owned();

    new_doc.metadata.previous_message_id = *self.chain_state().last_integration_message_id();

    self
      .sign_self(
        old_doc.unwrap_or(new_doc_ref),
        new_doc_ref,
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
      let bytes: [u8; 32] = rand::random();
      MessageId::new(bytes)
    } else {
      self.client.publish_document(&new_doc).await?.into()
    };

    self.chain_state.set_last_integration_message_id(message_id);
    // Overwrite the internal document with the published one, that has a signature.
    self.document = new_doc;

    Ok(())
  }

  async fn publish_diff_change(&mut self, old_doc: &IotaDocument, signing_method_query: &Option<String>) -> Result<()> {
    log::debug!("[publish_diff_change] publishing {:?}", self.document().id());

    let new_doc: &IotaDocument = &self.document;

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
      Some(fragment) => old_doc.resolve_signing_method(fragment)?,
      None => old_doc.default_signing_method()?,
    };

    self
      .remote_sign_data(old_doc.id(), signing_method, &mut diff, ProofOptions::default())
      .await?;

    log::debug!(
      "[publish_diff_change] publishing on index {}",
      IotaDocument::diff_index(self.chain_state().last_integration_message_id())?
    );

    let message_id: MessageId = if self.config.testmode {
      // Fake publishing by returning a random message id.
      let bytes: [u8; 32] = rand::random();
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

  // Helper function for remote signing.
  pub(crate) async fn remote_sign_data<D>(
    &self,
    did: &IotaDID,
    method: &IotaVerificationMethod,
    data: &mut D,
    options: ProofOptions,
  ) -> crate::Result<()>
  where
    D: Serialize + SetSignature,
  {
    let location: KeyLocation = KeyLocation::from_verification_method(method)?;

    // Create a private key suitable for identity_core::crypto
    let private: RemoteKey<'_> = RemoteKey::new(did, &location, self.storage().deref());

    let method_url: IotaDIDUrl = method.id().to_owned();

    match location.key_type {
      KeyType::Ed25519 => {
        RemoteEd25519::create_signature(data, method_url.to_string(), &private, options).await?;
      }
      KeyType::X25519 => return Err(identity_did::Error::InvalidMethodType.into()),
    }

    Ok(())
  }
}
