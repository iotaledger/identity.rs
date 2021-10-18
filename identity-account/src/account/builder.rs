// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::IotaDID;
use identity_iota::tangle::ClientBuilder;
use identity_iota::tangle::ClientMap;
use identity_iota::tangle::Network;
#[cfg(feature = "stronghold")]
use std::path::PathBuf;
use std::sync::Arc;
#[cfg(feature = "stronghold")]
use zeroize::Zeroize;

use crate::account::Account;
use crate::error::Result;
use crate::identity::IdentityCreate;
use crate::storage::MemStore;
use crate::storage::Storage;
#[cfg(feature = "stronghold")]
use crate::storage::Stronghold;

use super::config::AccountSetup;
use super::config::AutoSave;
use super::config::Config;

/// The storage adapter used by an [Account].
///
/// Note that [AccountStorage::Stronghold] is only available if the `stronghold` feature is activated, which it is by
/// default.
#[derive(Debug)]
pub enum AccountStorage {
  Memory,
  #[cfg(feature = "stronghold")]
  Stronghold(PathBuf, Option<String>),
  Custom(Arc<dyn Storage>),
}

/// An [Account] builder for easier account configuration.
#[derive(Debug)]
pub struct AccountBuilder {
  config: Config,
  storage: Arc<dyn Storage>,
  client_map: Arc<ClientMap>,
}

impl AccountBuilder {
  /// Creates a new `AccountBuilder`.
  pub fn new() -> Self {
    Self {
      config: Config::new(),
      storage: Arc::new(MemStore::new()),
      client_map: Arc::new(ClientMap::new()),
    }
  }

  /// Sets the account auto-save behaviour.
  ///
  /// See the config's [`autosave`][Config::autosave] documentation for details.
  pub fn autosave(mut self, value: AutoSave) -> Self {
    self.config = self.config.autosave(value);
    self
  }

  /// Sets the account auto-publish behaviour.
  ///
  /// See the config's [`autopublish`][Config::autopublish] documentation for details.
  pub fn autopublish(mut self, value: bool) -> Self {
    self.config = self.config.autopublish(value);
    self
  }

  /// Save the account state on drop.
  ///
  /// See the config's [`dropsave`][Config::dropsave] documentation for details.
  pub fn dropsave(mut self, value: bool) -> Self {
    self.config = self.config.dropsave(value);
    self
  }

  /// Save a state snapshot every N actions.
  pub fn milestone(mut self, value: u32) -> Self {
    self.config = self.config.milestone(value);
    self
  }

  #[allow(dead_code)]
  #[doc(hidden)]
  pub(crate) fn testmode(mut self, value: bool) -> Self {
    self.config = self.config.testmode(value);
    self
  }

  /// Sets the account storage adapter.
  pub async fn storage(mut self, value: AccountStorage) -> Result<Self> {
    self.storage = match value {
      AccountStorage::Memory => Arc::new(MemStore::new()),
      #[cfg(feature = "stronghold")]
      AccountStorage::Stronghold(snapshot, password) => {
        let passref: Option<&str> = password.as_deref();
        let adapter: Stronghold = Stronghold::new(&snapshot, passref).await?;

        if let Some(mut password) = password {
          password.zeroize();
        }

        Arc::new(adapter)
      }
      AccountStorage::Custom(adapter) => adapter,
    };

    Ok(self)
  }

  /// Apply configuration to the IOTA Tangle client for the given `Network`.
  pub async fn client<F>(self, network: Network, f: F) -> Result<Self>
  where
    F: FnOnce(ClientBuilder) -> ClientBuilder,
  {
    self
      .client_map
      .insert(f(ClientBuilder::new().network(network)).build().await?);
    Ok(self)
  }

  /// Creates a new [Account] based on the builder configuration.
  pub async fn create_identity(&self, input: IdentityCreate) -> Result<Account> {
    let setup = AccountSetup::new_with_options(
      Arc::clone(&self.storage),
      Some(self.config.clone()),
      Some(Arc::clone(&self.client_map)),
    );

    let account = Account::create_identity(setup, input).await?;

    Ok(account)
  }

  /// Loads an existing identity from the given did with the builder configuration.
  pub async fn load_identity(&self, did: IotaDID) -> Result<Account> {
    let setup = AccountSetup::new_with_options(
      Arc::clone(&self.storage),
      Some(self.config.clone()),
      Some(Arc::clone(&self.client_map)),
    );

    let account = Account::load(setup, did).await?;

    Ok(account)
  }
}

impl Default for AccountBuilder {
  fn default() -> Self {
    Self::new()
  }
}
