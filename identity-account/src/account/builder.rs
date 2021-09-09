// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use hashbrown::HashMap;
use identity_iota::tangle::ClientBuilder;
use identity_iota::tangle::Network;
use identity_iota::tangle::NetworkName;
#[cfg(feature = "stronghold")]
use std::path::PathBuf;
#[cfg(feature = "stronghold")]
use zeroize::Zeroize;

use crate::account::Account;
use crate::account::AutoSave;
use crate::account::Config;
use crate::error::Result;
use crate::storage::MemStore;
use crate::storage::Storage;
#[cfg(feature = "stronghold")]
use crate::storage::Stronghold;

/// The storage adapter used by an [Account].
///
/// Note that [AccountStorage::Stronghold] is only available if the `stronghold` feature is activated, which it is by
/// default.
#[derive(Debug)]
pub enum AccountStorage {
  Memory,
  #[cfg(feature = "stronghold")]
  Stronghold(PathBuf, Option<String>),
  Custom(Box<dyn Storage>),
}

/// An [Account] builder for easier account configuration.
#[derive(Debug)]
pub struct AccountBuilder {
  config: Config,
  storage: AccountStorage,
  clients: Option<HashMap<NetworkName, ClientBuilder>>,
}

impl AccountBuilder {
  /// Creates a new `AccountBuilder`.
  pub fn new() -> Self {
    Self {
      config: Config::new(),
      storage: AccountStorage::Memory,
      clients: None,
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

  /// Sets the account storage adapter.
  pub fn storage(mut self, value: AccountStorage) -> Self {
    self.storage = value;
    self
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
  pub async fn build(mut self) -> Result<Account> {
    let account: Account = match self.storage {
      AccountStorage::Memory => Account::with_config(MemStore::new(), self.config).await?,
      #[cfg(feature = "stronghold")]
      AccountStorage::Stronghold(snapshot, password) => {
        let passref: Option<&str> = password.as_deref();
        let adapter: Stronghold = Stronghold::new(&snapshot, passref).await?;

        if let Some(mut password) = password {
          password.zeroize();
        }

        Account::with_config(adapter, self.config).await?
      }
      AccountStorage::Custom(adapter) => Account::with_config(adapter, self.config).await?,
    };

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
