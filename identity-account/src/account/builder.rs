// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::IotaDID;
use identity_iota::tangle::ClientBuilder;
use identity_iota::tangle::ClientMap;
use identity_iota::tangle::Network;
use identity_iota::tangle::NetworkName;
use std::collections::HashMap;
#[cfg(feature = "stronghold")]
use std::path::PathBuf;
use std::sync::Arc;
#[cfg(feature = "stronghold")]
use zeroize::Zeroize;

use crate::account::Account;
use crate::error::Result;
use crate::identity::IdentitySetup;
use crate::storage::MemStore;
use crate::storage::Storage;
#[cfg(feature = "stronghold")]
use crate::storage::Stronghold;

use super::config::AccountConfig;
use super::config::AccountSetup;
use super::config::AutoSave;

/// The storage adapter used by an [Account].
///
/// Note that [AccountStorage::Stronghold] is only available if the `stronghold` feature is activated, which it is by
/// default.
#[derive(Debug)]
pub enum AccountStorage {
  Memory,
  #[cfg(feature = "stronghold")]
  Stronghold(PathBuf, Option<String>, Option<bool>),
  Custom(Arc<dyn Storage>),
}

/// An [`Account`] builder for easy account configuration.
///
/// To reduce memory usage, accounts created from the same builder share the same [`Storage`],
/// used to store identities, and [`ClientMap`], used to
/// publish identities to the Tangle. This means using [`AccountBuilder::client`]
/// to customize a client, will modify the existing client map in previously
/// built accounts, when the next account is built.
///
/// The configuration on the other hand is cloned, and therefore unique for each built account.
/// This means a builder can be reconfigured in-between account creations, without affecting
/// the configuration of previously built accounts.
#[derive(Debug)]
pub struct AccountBuilder {
  config: AccountConfig,
  storage_template: Option<AccountStorage>,
  storage: Option<Arc<dyn Storage>>,
  client_builders: Option<HashMap<NetworkName, ClientBuilder>>,
  client_map: Arc<ClientMap>,
}

impl AccountBuilder {
  /// Creates a new `AccountBuilder`.
  pub fn new() -> Self {
    Self {
      config: AccountConfig::new(),
      storage_template: Some(AccountStorage::Memory),
      storage: Some(Arc::new(MemStore::new())),
      client_builders: None,
      client_map: Arc::new(ClientMap::new()),
    }
  }

  /// Sets the account auto-save behaviour.
  ///
  /// See the config's [`autosave`][AccountConfig::autosave] documentation for details.
  pub fn autosave(mut self, value: AutoSave) -> Self {
    self.config = self.config.autosave(value);
    self
  }

  /// Sets the account auto-publish behaviour.
  ///
  /// See the config's [`autopublish`][AccountConfig::autopublish] documentation for details.
  pub fn autopublish(mut self, value: bool) -> Self {
    self.config = self.config.autopublish(value);
    self
  }

  /// Save a state snapshot every N actions.
  pub fn milestone(mut self, value: u32) -> Self {
    self.config = self.config.milestone(value);
    self
  }

  #[cfg(test)]
  /// Set whether the account is in testmode or not.
  /// In testmode, the account skips publishing to the tangle.
  pub(crate) fn testmode(mut self, value: bool) -> Self {
    self.config = self.config.testmode(value);
    self
  }

  /// Sets the account storage adapter.
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

  /// Apply configuration to the IOTA Tangle client for the given [`Network`].
  pub fn client<F>(mut self, network: Network, f: F) -> Self
  where
    F: FnOnce(ClientBuilder) -> ClientBuilder,
  {
    self
      .client_builders
      .get_or_insert_with(HashMap::new)
      .insert(network.name(), f(ClientBuilder::new().network(network)));
    self
  }

  async fn build_clients(&mut self) -> Result<()> {
    if let Some(hmap) = self.client_builders.take() {
      for builder in hmap.into_iter() {
        self.client_map.insert(builder.1.build().await?)
      }
    }

    Ok(())
  }

  async fn build_setup(&mut self) -> Result<AccountSetup> {
    self.build_clients().await?;

    Ok(AccountSetup::new_with_options(
      self.get_storage().await?,
      Some(self.config.clone()),
      Some(Arc::clone(&self.client_map)),
    ))
  }

  /// Creates a new identity based on the builder configuration and returns
  /// an [`Account`] instance to manage it.
  /// The identity is stored locally in the [`Storage`].
  ///
  /// See [`IdentitySetup`] to customize the identity creation.
  pub async fn create_identity(&mut self, input: IdentitySetup) -> Result<Account> {
    let setup: AccountSetup = self.build_setup().await?;
    Account::create_identity(setup, input).await
  }

  /// Loads an existing identity with the specified `did` using the current builder configuration.
  /// The identity must exist in the configured [`Storage`].
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
