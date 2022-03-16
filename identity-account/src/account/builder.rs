// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "stronghold")]
use std::path::PathBuf;
use std::sync::Arc;

#[cfg(feature = "stronghold")]
use zeroize::Zeroize;

use identity_account_storage::storage::MemStore;
use identity_account_storage::storage::Storage;
#[cfg(feature = "stronghold")]
use identity_account_storage::storage::Stronghold;
use identity_iota::tangle::Client;
use identity_iota::tangle::ClientBuilder;
use identity_iota_core::did::IotaDID;

use super::config::AccountConfig;
use super::config::AccountSetup;
use super::config::AutoSave;
use crate::account::Account;
use crate::error::Result;
use crate::identity::IdentitySetup;

/// The storage adapter used by an [`Account`].
///
/// Note that [`AccountStorage::Stronghold`] is only available if the `stronghold` feature is
/// activated, which it is by default.
#[derive(Debug)]
pub enum AccountStorage {
  Memory,
  #[cfg(feature = "stronghold")]
  Stronghold(PathBuf, Option<String>, Option<bool>),
  Custom(Arc<dyn Storage>),
}

/// An [`Account`] builder for easy account configuration.
///
/// To reduce memory usage, accounts created from the same builder share the same [`Storage`]
/// used to store identities, and the same [`Client`] used to publish identities to the Tangle.
///
/// The configuration on the other hand is cloned, and therefore unique for each built account.
/// This means a builder can be reconfigured in-between account creations, without affecting
/// the configuration of previously built accounts.
#[derive(Debug)]
pub struct AccountBuilder {
  config: AccountConfig,
  storage_template: Option<AccountStorage>,
  storage: Option<Arc<dyn Storage>>,
  client_builder: Option<ClientBuilder>,
  client: Option<Arc<Client>>,
}

impl AccountBuilder {
  /// Creates a new `AccountBuilder`.
  pub fn new() -> Self {
    Self {
      config: AccountConfig::new(),
      storage_template: Some(AccountStorage::Memory),
      storage: Some(Arc::new(MemStore::new())),
      client_builder: None,
      client: None,
    }
  }

  /// Sets the account auto-save behaviour.
  ///
  /// See the config's [`autosave`][AccountConfig::autosave] documentation for details.
  #[must_use]
  pub fn autosave(mut self, value: AutoSave) -> Self {
    self.config = self.config.autosave(value);
    self
  }

  /// Sets the account auto-publish behaviour.
  ///
  /// See the config's [`autopublish`][AccountConfig::autopublish] documentation for details.
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
  pub fn storage(mut self, value: AccountStorage) -> Self {
    self.storage_template = Some(value);
    self
  }

  async fn get_storage(&mut self) -> Result<Arc<dyn Storage>> {
    match self.storage_template.take() {
      Some(AccountStorage::Memory) => {
        let storage = Arc::new(MemStore::new());
        self.storage = Some(storage);
      }
      #[cfg(feature = "stronghold")]
      Some(AccountStorage::Stronghold(snapshot, password, dropsave)) => {
        let passref: Option<&str> = password.as_deref();
        let adapter: Stronghold = Stronghold::new(&snapshot, passref, dropsave).await?;

        if let Some(mut password) = password {
          password.zeroize();
        }

        let storage = Arc::new(adapter);
        self.storage = Some(storage);
      }
      Some(AccountStorage::Custom(storage)) => {
        self.storage = Some(storage);
      }
      None => (),
    };

    // unwrap is fine, since by default, storage_template is `Some`,
    // which results in storage being `Some`.
    // Overwriting storage_template always produces `Some` storage.
    Ok(Arc::clone(self.storage.as_ref().unwrap()))
  }

  /// Sets the IOTA Tangle [`Client`], this determines the [`Network`] used by the identity.
  /// [`Accounts`](Account) created by the same [`AccountBuilder`] will share the same [`Client`].
  ///
  /// NOTE: this overwrites any [`ClientBuilder`] previously set by
  /// [`AccountBuilder::client_builder`].
  #[must_use]
  pub fn client(mut self, client: Arc<Client>) -> Self {
    self.client = Some(client);
    self.client_builder = None;
    self
  }

  /// Sets the IOTA Tangle [`Client`], this determines the [`Network`] used by the identity.
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
  async fn get_or_build_client(&mut self) -> Result<Arc<Client>> {
    if let Some(client) = &self.client {
      Ok(Arc::clone(client))
    } else if let Some(client_builder) = self.client_builder.take() {
      let client: Arc<Client> = Arc::new(client_builder.build().await?);
      self.client = Some(Arc::clone(&client));
      Ok(client)
    } else {
      let client: Arc<Client> = Arc::new(Client::new().await?);
      self.client = Some(Arc::clone(&client));
      Ok(client)
    }
  }

  async fn build_setup(&mut self) -> Result<AccountSetup> {
    let client: Arc<Client> = self.get_or_build_client().await?;

    Ok(AccountSetup::new(
      self.get_storage().await?,
      client,
      self.config.clone(),
    ))
  }

  /// Creates a new identity based on the builder configuration and returns
  /// an [`Account`] instance to manage it.
  ///
  /// The identity is stored locally in the [`Storage`]. The DID network is automatically determined
  /// by the [`Client`] used to publish it.
  ///
  /// See [`IdentitySetup`] to customize the identity creation.
  pub async fn create_identity(&mut self, input: IdentitySetup) -> Result<Account> {
    let setup: AccountSetup = self.build_setup().await?;
    Account::create_identity(setup, input).await
  }

  /// Loads an existing identity with the specified `did` using the current builder configuration.
  /// The identity must exist in the configured [`Storage`].
  ///
  /// # Warning
  ///
  /// Callers are expected **not** to load the same [`IotaDID`] into more than one account,
  /// as that would cause race conditions when updating the identity.
  pub async fn load_identity(&mut self, did: IotaDID) -> Result<Account> {
    let setup: AccountSetup = self.build_setup().await?;
    Account::load_identity(setup, did).await
  }
}

impl Default for AccountBuilder {
  fn default() -> Self {
    Self::new()
  }
}
