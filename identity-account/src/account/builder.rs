// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use hashbrown::HashMap;
use identity_iota::did::IotaDID;
use identity_iota::tangle::ClientBuilder;
use identity_iota::tangle::Network;
use identity_iota::tangle::NetworkName;
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

use super::config::AccountConfig;
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
  clients: Option<HashMap<NetworkName, ClientBuilder>>,
}

impl AccountBuilder {
  /// Creates a new `AccountBuilder`.
  pub fn new() -> Self {
    Self {
      config: Config::new(),
      storage: Arc::new(MemStore::new()),
      clients: None,
    }
  }

  /// Sets the account auto-save behaviour.
  ///
  /// See the config's [`autosave`][Config::autosave] documentation for details.
  pub fn autosave(mut self, value: AutoSave) -> Self {
    self.config.autosave = value;
    self
  }

  /// Sets the account auto-publish behaviour.
  ///
  /// See the config's [`autopublish`][Config::autopublish] documentation for details.
  pub fn autopublish(mut self, value: bool) -> Self {
    self.config.autopublish = value;
    self
  }

  /// Save the account state on drop.
  ///
  /// See the config's [`dropsave`][Config::dropsave] documentation for details.
  pub fn dropsave(mut self, value: bool) -> Self {
    self.config.dropsave = value;
    self
  }

  /// Save a state snapshot every N actions.
  pub fn milestone(mut self, value: u32) -> Self {
    self.config.milestone = value;
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
  pub fn client<F>(mut self, network: Network, f: F) -> Self
  where
    F: FnOnce(ClientBuilder) -> ClientBuilder,
  {
    self
      .clients
      .get_or_insert_with(HashMap::new)
      .insert(network.name(), f(ClientBuilder::new().network(network)));
    self
  }

  /// Creates a new [Account] based on the builder configuration.
  pub async fn create_identity(mut self, input: IdentityCreate) -> Result<Account> {
    let config = AccountConfig::new_with_config(Arc::clone(&self.storage), self.config.clone());
    let account = Account::create_identity(config, input).await?;

    if let Some(clients) = self.clients.take() {
      for (_, client) in clients.into_iter() {
        account.set_client(client.build().await?);
      }
    }

    Ok(account)
  }

  /// Loads an existing identity from the given did.
  pub async fn load_identity(mut self, did: IotaDID) -> Result<Account> {
    let config = AccountConfig::new_with_config(Arc::clone(&self.storage), self.config.clone());
    let account = Account::load(did, config).await?;

    if let Some(clients) = self.clients.take() {
      for (_, client) in clients.into_iter() {
        account.set_client(client.build().await?);
      }
    }

    Ok(account)
  }
}

impl Default for AccountBuilder {
  fn default() -> Self {
    Self::new()
  }
}
