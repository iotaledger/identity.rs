// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use identity_account_storage::storage::MemStore;
use identity_account_storage::storage::Storage;
use identity_iota::tangle::Client;
use identity_iota::tangle::ClientBuilder;
use identity_iota::tangle::SharedPtr;
use identity_iota_core::did::IotaDID;

use crate::account::Account;
use crate::error::Result;
use crate::types::IdentitySetup;

use super::config::AccountConfig;
use super::config::AccountSetup;
use super::config::AutoSave;

/// An [`Account`] builder for easy account configuration.
///
/// To reduce memory usage, accounts created from the same builder share the same [`Storage`]
/// used to store identities, and the same [`Client`] used to publish identities to the Tangle.
///
/// The configuration on the other hand is cloned, and therefore unique for each built account.
/// This means a builder can be reconfigured in-between account creations, without affecting
/// the configuration of previously built accounts.
#[derive(Debug)]
pub struct AccountBuilder<C = Arc<Client>>
where
  C: SharedPtr<Client>,
{
  config: AccountConfig,
  storage: Option<Arc<dyn Storage>>,
  client_builder: Option<ClientBuilder>,
  client: Option<C>,
}

impl<C> AccountBuilder<C>
where
  C: SharedPtr<Client>,
{
  /// Creates a new `AccountBuilder`.
  pub fn new() -> Self {
    Self {
      config: AccountConfig::new(),
      storage: None,
      client_builder: None,
      client: None,
    }
  }

  /// Sets the account auto-save behaviour.
  #[must_use]
  pub fn autosave(mut self, value: AutoSave) -> Self {
    self.config = self.config.autosave(value);
    self
  }

  /// Sets the account auto-publish behaviour.
  #[must_use]
  pub fn autopublish(mut self, value: bool) -> Self {
    self.config = self.config.autopublish(value);
    self
  }

  /// Set whether the account is in testmode or not.
  /// In testmode, the account skips publishing to the tangle.
  #[cfg(test)]
  #[must_use]
  pub(crate) fn testmode(mut self, value: bool) -> Self {
    self.config = self.config.testmode(value);
    self
  }

  /// Sets the account storage adapter.
  #[must_use]
  pub fn storage<S: Storage + 'static>(mut self, value: S) -> Self {
    self.storage = Some(Arc::new(value));
    self
  }

  /// Sets the account storage adapter from a shared pointer.
  #[must_use]
  pub fn storage_shared(mut self, value: Arc<dyn Storage>) -> Self {
    self.storage = Some(value);
    self
  }

  fn get_storage(&mut self) -> Arc<dyn Storage> {
    if let Some(storage) = self.storage.as_ref() {
      Arc::clone(storage)
    } else {
      // TODO: throw an error or log a warning that MemStore is not persistent and should not
      //       be used in production?
      let storage: Arc<dyn Storage> = Arc::new(MemStore::new());
      self.storage = Some(Arc::clone(&storage));
      storage
    }
  }

  /// Sets the IOTA Tangle [`Client`], this determines the [`Network`][identity_iota_core::tangle::Network] used by the identity.
  /// [`Accounts`](Account) created by the same [`AccountBuilder`] will share the same [`Client`].
  ///
  /// NOTE: this overwrites any [`ClientBuilder`] previously set by
  /// [`AccountBuilder::client_builder`].
  #[must_use]
  pub fn client(mut self, client: C) -> Self {
    self.client = Some(client);
    self.client_builder = None;
    self
  }

  /// Sets the IOTA Tangle [`Client`], this determines the [`Network`][identity_iota_core::tangle::Network] used by the identity.
  /// [`Accounts`](Account) created by the same [`AccountBuilder`] will share the same [`Client`].
  ///
  /// NOTE: this overwrites any [`Client`] previously set by [`AccountBuilder::client`].
  #[must_use]
  pub fn client_builder(mut self, client_builder: ClientBuilder) -> Self {
    self.client = None;
    self.client_builder = Some(client_builder);
    self
  }

  /// Returns a previously set [`Client`] or builds a new one based on the configuration passed
  /// to [`AccountBuilder::client_builder`].
  ///
  /// If neither is set, instantiates and stores a default [`Client`].
  async fn get_or_build_client(&mut self) -> Result<C> {
    if let Some(client) = &self.client {
      Ok(C::clone(client))
    } else if let Some(client_builder) = self.client_builder.take() {
      let client: C = C::from(client_builder.build().await?);
      self.client = Some(C::clone(&client));
      Ok(client)
    } else {
      let client: C = C::from(Client::new().await?);
      self.client = Some(C::clone(&client));
      Ok(client)
    }
  }

  async fn build_setup(&mut self) -> Result<AccountSetup<C>> {
    let client: C = self.get_or_build_client().await?;

    Ok(AccountSetup::<C>::new(self.get_storage(), client, self.config.clone()))
  }

  /// Creates a new identity based on the builder configuration and returns
  /// an [`Account`] instance to manage it.
  ///
  /// The identity is stored locally in the [`Storage`]. The DID network is automatically determined
  /// by the [`Client`] used to publish it.
  ///
  /// See [`IdentitySetup`] to customize the identity creation.
  pub async fn create_identity(&mut self, input: IdentitySetup) -> Result<Account<C>> {
    let setup: AccountSetup<C> = self.build_setup().await?;
    Account::create_identity(setup, input).await
  }

  /// Loads an existing identity with the specified `did` using the current builder configuration.
  /// The identity must exist in the configured [`Storage`].
  ///
  /// # Warning
  ///
  /// Callers are expected **not** to load the same [`IotaDID`] into more than one account,
  /// as that would cause race conditions when updating the identity.
  pub async fn load_identity(&mut self, did: IotaDID) -> Result<Account<C>> {
    let setup: AccountSetup<C> = self.build_setup().await?;
    Account::load_identity(setup, did).await
  }
}

impl<C> Default for AccountBuilder<C>
where
  C: SharedPtr<Client>,
{
  fn default() -> Self {
    Self::new()
  }
}
